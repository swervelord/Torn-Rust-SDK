param(
    [string]$SpecPath,
    [string]$OutputPath,
    [string]$OverridesPath
)

Set-StrictMode -Version 1
$ErrorActionPreference = "Stop"

function Get-RepoRoot {
    return (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
}

function Get-RepoRelativePath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,
        [Parameter(Mandatory = $true)]
        [string]$RepoRoot
    )

    $resolvedPath = (Resolve-Path $Path).Path
    $resolvedRoot = (Resolve-Path $RepoRoot).Path
    if ($resolvedPath.StartsWith($resolvedRoot, [StringComparison]::OrdinalIgnoreCase)) {
        $relativePath = $resolvedPath.Substring($resolvedRoot.Length)
        $relativePath = $relativePath.TrimStart([char[]]@('\', '/'))
        return $relativePath.Replace('\', '/')
    }

    return $resolvedPath.Replace('\', '/')
}

function Decode-JsonRefToken {
    param([string]$Token)
    return $Token.Replace("~1", "/").Replace("~0", "~")
}

function Resolve-JsonRef {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Spec,
        [Parameter(Mandatory = $true)]
        [string]$Ref
    )

    if (-not $Ref.StartsWith("#/")) {
        return $null
    }

    $segments = $Ref.Substring(2).Split("/") | ForEach-Object { Decode-JsonRefToken -Token $_ }
    $current = $Spec
    foreach ($segment in $segments) {
        if ($null -eq $current) {
            return $null
        }

        $prop = $current.PSObject.Properties[$segment]
        if (-not $prop) {
            return $null
        }

        $current = $prop.Value
    }

    return $current
}

function Get-RefName {
    param([string]$Ref)
    if ([string]::IsNullOrWhiteSpace($Ref)) {
        return $null
    }

    return ($Ref -split "/")[-1]
}

function Test-IsPathParameterToken {
    param([string]$Segment)
    return $Segment.StartsWith("{") -and $Segment.EndsWith("}")
}

function Normalize-SelectionKey {
    param([string]$Value)
    if ([string]::IsNullOrWhiteSpace($Value)) {
        return ""
    }

    return ([regex]::Replace($Value.ToLowerInvariant(), "[^a-z0-9]", ""))
}

function Get-QuotedTokens {
    param([string]$Text)
    $tokens = New-Object System.Collections.Generic.List[string]
    if ([string]::IsNullOrWhiteSpace($Text)) {
        return @()
    }

    $matches = [regex]::Matches($Text, "'([^']+)'")
    foreach ($match in $matches) {
        $value = $match.Groups[1].Value.Trim()
        if (-not [string]::IsNullOrWhiteSpace($value)) {
            $tokens.Add($value)
        }
    }

    return $tokens.ToArray()
}

function Get-ListFromDescriptionSection {
    param(
        [string]$Description,
        [string]$Marker,
        [string]$UntilMarker
    )

    if ([string]::IsNullOrWhiteSpace($Description)) {
        return @()
    }

    $start = $Description.IndexOf($Marker, [StringComparison]::OrdinalIgnoreCase)
    if ($start -lt 0) {
        return @()
    }

    $end = $Description.Length
    if (-not [string]::IsNullOrWhiteSpace($UntilMarker)) {
        $until = $Description.IndexOf($UntilMarker, $start, [StringComparison]::OrdinalIgnoreCase)
        if ($until -gt $start) {
            $end = $until
        }
    }

    $section = $Description.Substring($start, $end - $start)
    return Get-QuotedTokens -Text $section
}

function Get-SchemaSummary {
    param([pscustomobject]$Schema)

    if ($null -eq $Schema) {
        return [ordered]@{}
    }

    function Try-GetSchemaPropValue {
        param(
            [pscustomobject]$SchemaObject,
            [string]$Name
        )
        $prop = $SchemaObject.PSObject.Properties[$Name]
        if ($prop) {
            return $prop.Value
        }
        return $null
    }

    $summary = [ordered]@{}
    $typeValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "type"
    $formatValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "format"
    $descriptionValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "description"
    $defaultValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "default"
    $minimumValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "minimum"
    $maximumValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "maximum"
    $minLengthValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "minLength"
    $maxLengthValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "maxLength"

    if ($null -ne $typeValue -and $typeValue -ne "") { $summary.type = $typeValue }
    if ($null -ne $formatValue -and $formatValue -ne "") { $summary.format = $formatValue }
    if ($null -ne $descriptionValue -and $descriptionValue -ne "") { $summary.description = $descriptionValue }
    if ($null -ne $defaultValue) { $summary.default = $defaultValue }
    if ($null -ne $minimumValue) { $summary.minimum = $minimumValue }
    if ($null -ne $maximumValue) { $summary.maximum = $maximumValue }
    if ($null -ne $minLengthValue) { $summary.min_length = $minLengthValue }
    if ($null -ne $maxLengthValue) { $summary.max_length = $maxLengthValue }

    $enumValue = Try-GetSchemaPropValue -SchemaObject $Schema -Name "enum"
    if ($enumValue) {
        $summary.enum = @($enumValue)
    }

    if ($Schema.PSObject.Properties["`$ref"]) {
        $summary.ref = $Schema.PSObject.Properties["`$ref"].Value
    }

    if ($Schema.items -and $Schema.items.PSObject.Properties["`$ref"]) {
        $summary.items_ref = $Schema.items.PSObject.Properties["`$ref"].Value
    }

    if ($Schema.oneOf) {
        $refs = @()
        foreach ($entry in $Schema.oneOf) {
            if ($entry.PSObject.Properties["`$ref"]) {
                $refs += $entry.PSObject.Properties["`$ref"].Value
            }
        }
        if ($refs.Count -gt 0) { $summary.one_of_refs = @($refs) }
    }

    if ($Schema.anyOf) {
        $refs = @()
        foreach ($entry in $Schema.anyOf) {
            if ($entry.PSObject.Properties["`$ref"]) {
                $refs += $entry.PSObject.Properties["`$ref"].Value
            }
        }
        if ($refs.Count -gt 0) { $summary.any_of_refs = @($refs) }
    }

    if ($Schema.allOf) {
        $refs = @()
        foreach ($entry in $Schema.allOf) {
            if ($entry.PSObject.Properties["`$ref"]) {
                $refs += $entry.PSObject.Properties["`$ref"].Value
            }
        }
        if ($refs.Count -gt 0) { $summary.all_of_refs = @($refs) }
    }

    return $summary
}

function Get-ParameterCapability {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Spec,
        [Parameter(Mandatory = $true)]
        [pscustomobject]$ParameterObject
    )

    $refValue = $null
    if ($ParameterObject.PSObject.Properties["`$ref"]) {
        $refValue = $ParameterObject.PSObject.Properties["`$ref"].Value
    }

    $resolved = $ParameterObject
    $refName = $null
    if ($refValue) {
        $resolvedRef = Resolve-JsonRef -Spec $Spec -Ref $refValue
        if ($null -ne $resolvedRef) {
            $resolved = $resolvedRef
        }
        $refName = Get-RefName -Ref $refValue
    }

    $schemaSummary = Get-SchemaSummary -Schema $resolved.schema
    $capability = [ordered]@{
        name = $resolved.name
        in = $resolved.in
        required = [bool]$resolved.required
    }

    if ($resolved.description) { $capability.description = $resolved.description }
    if ($resolved.style) { $capability.style = $resolved.style }
    if ($null -ne $resolved.explode) { $capability.explode = [bool]$resolved.explode }
    if ($refValue) { $capability.ref = $refValue }
    if ($refName) { $capability.ref_name = $refName }
    if ($schemaSummary.Keys.Count -gt 0) { $capability.schema = $schemaSummary }

    return $capability
}

function Collect-SchemaRefs {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Schema,
        [System.Collections.Generic.HashSet[string]]$Collector
    )

    if ($null -eq $Collector) {
        $Collector = New-Object 'System.Collections.Generic.HashSet[string]'
    }

    if ($Schema.PSObject.Properties["`$ref"]) {
        [void]$Collector.Add($Schema.PSObject.Properties["`$ref"].Value)
    }

    if ($Schema.items) {
        Collect-SchemaRefs -Schema $Schema.items -Collector $Collector | Out-Null
    }

    foreach ($compositeName in @("anyOf", "oneOf", "allOf")) {
        $composite = $Schema.PSObject.Properties[$compositeName]
        if ($composite -and $composite.Value) {
            foreach ($entry in $composite.Value) {
                Collect-SchemaRefs -Schema $entry -Collector $Collector | Out-Null
            }
        }
    }

    return ,$Collector
}

function Get-SchemaTopLevelProperties {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Spec,
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Schema,
        [System.Collections.Generic.HashSet[string]]$VisitedRefs
    )

    if ($null -eq $VisitedRefs) {
        $VisitedRefs = New-Object 'System.Collections.Generic.HashSet[string]'
    }

    $result = New-Object System.Collections.Generic.HashSet[string]

    if ($Schema.PSObject.Properties["`$ref"]) {
        $ref = $Schema.PSObject.Properties["`$ref"].Value
        if ($VisitedRefs.Add($ref)) {
            $resolved = Resolve-JsonRef -Spec $Spec -Ref $ref
            if ($null -ne $resolved) {
                $nested = Get-SchemaTopLevelProperties -Spec $Spec -Schema $resolved -VisitedRefs $VisitedRefs
                foreach ($name in $nested) {
                    [void]$result.Add($name)
                }
            }
        }

        return ,$result
    }

    if ($Schema.properties) {
        foreach ($prop in $Schema.properties.PSObject.Properties.Name) {
            [void]$result.Add($prop)
        }
    }

    if ($Schema.allOf) {
        foreach ($entry in $Schema.allOf) {
            $nested = Get-SchemaTopLevelProperties -Spec $Spec -Schema $entry -VisitedRefs $VisitedRefs
            foreach ($name in $nested) {
                [void]$result.Add($name)
            }
        }
    }

    return ,$result
}

function Get-SchemaRequiredTopLevelFields {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Spec,
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Schema,
        [System.Collections.Generic.HashSet[string]]$VisitedRefs
    )

    if ($null -eq $VisitedRefs) {
        $VisitedRefs = New-Object 'System.Collections.Generic.HashSet[string]'
    }

    $result = New-Object System.Collections.Generic.HashSet[string]

    if ($Schema.PSObject.Properties["`$ref"]) {
        $ref = $Schema.PSObject.Properties["`$ref"].Value
        if ($VisitedRefs.Add($ref)) {
            $resolved = Resolve-JsonRef -Spec $Spec -Ref $ref
            if ($null -ne $resolved) {
                $nested = Get-SchemaRequiredTopLevelFields -Spec $Spec -Schema $resolved -VisitedRefs $VisitedRefs
                foreach ($name in $nested) {
                    [void]$result.Add($name)
                }
            }
        }

        return ,$result
    }

    if ($Schema.required) {
        foreach ($item in $Schema.required) {
            [void]$result.Add($item)
        }
    }

    if ($Schema.allOf) {
        foreach ($entry in $Schema.allOf) {
            $nested = Get-SchemaRequiredTopLevelFields -Spec $Spec -Schema $entry -VisitedRefs $VisitedRefs
            foreach ($name in $nested) {
                [void]$result.Add($name)
            }
        }
    }

    return ,$result
}

function Collect-SchemaFieldPaths {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Spec,
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Schema,
        [string]$Prefix,
        [System.Collections.Generic.HashSet[string]]$Collector,
        [System.Collections.Generic.HashSet[string]]$VisitedRefs
    )

    if ($null -eq $Collector) {
        $Collector = New-Object 'System.Collections.Generic.HashSet[string]'
    }
    if ($null -eq $VisitedRefs) {
        $VisitedRefs = New-Object 'System.Collections.Generic.HashSet[string]'
    }

    if ($Schema.PSObject.Properties["`$ref"]) {
        $ref = $Schema.PSObject.Properties["`$ref"].Value
        if ($VisitedRefs.Add($ref)) {
            $resolved = Resolve-JsonRef -Spec $Spec -Ref $ref
            if ($resolved) {
                Collect-SchemaFieldPaths -Spec $Spec -Schema $resolved -Prefix $Prefix -Collector $Collector -VisitedRefs $VisitedRefs | Out-Null
            }
            [void]$VisitedRefs.Remove($ref)
        }
        return ,$Collector
    }

    foreach ($compositeName in @("allOf", "oneOf", "anyOf")) {
        $composite = $Schema.PSObject.Properties[$compositeName]
        if ($composite -and $composite.Value) {
            foreach ($entry in $composite.Value) {
                Collect-SchemaFieldPaths -Spec $Spec -Schema $entry -Prefix $Prefix -Collector $Collector -VisitedRefs $VisitedRefs | Out-Null
            }
        }
    }

    if ($Schema.properties) {
        foreach ($property in $Schema.properties.PSObject.Properties) {
            $propertyName = [string]$property.Name
            $path = if ([string]::IsNullOrWhiteSpace($Prefix)) { $propertyName } else { "$Prefix.$propertyName" }
            [void]$Collector.Add($path)

            $propertySchema = $property.Value
            if ($propertySchema) {
                Collect-SchemaFieldPaths -Spec $Spec -Schema $propertySchema -Prefix $path -Collector $Collector -VisitedRefs $VisitedRefs | Out-Null
            }
        }
    }

    if ($Schema.items) {
        $arrayPath = if ([string]::IsNullOrWhiteSpace($Prefix)) { "[]" } else { "$Prefix[]" }
        [void]$Collector.Add($arrayPath)
        Collect-SchemaFieldPaths -Spec $Spec -Schema $Schema.items -Prefix $arrayPath -Collector $Collector -VisitedRefs $VisitedRefs | Out-Null
    }

    return ,$Collector
}

function Get-ResponseJsonSchema {
    param([pscustomobject]$Operation)

    $responses = $Operation.responses
    if ($null -eq $responses) {
        return $null
    }

    $ok = $responses.PSObject.Properties["200"]
    if (-not $ok) {
        return $null
    }

    $content = $ok.Value.content
    if ($null -eq $content) {
        return $null
    }

    $jsonContent = $content.PSObject.Properties["application/json"]
    if (-not $jsonContent) {
        return $null
    }

    return $jsonContent.Value.schema
}

function Get-SelectionHintFromPath {
    param(
        [string]$ResourceName,
        [string]$Path
    )

    $prefix = "/$ResourceName"
    if (-not $Path.StartsWith($prefix)) {
        return $null
    }

    $suffix = $Path.Substring($prefix.Length).Trim("/")
    if ([string]::IsNullOrWhiteSpace($suffix)) {
        return $null
    }

    $segments = $suffix.Split("/") | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    $staticSegments = @($segments | Where-Object { -not (Test-IsPathParameterToken -Segment $_) })
    if ($staticSegments.Count -eq 0) {
        return $null
    }

    $lastSegment = [string]$staticSegments[$staticSegments.Count - 1]
    return $lastSegment.ToLowerInvariant()
}

function Get-SelectionSchemaInfo {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Spec,
        [Parameter(Mandatory = $true)]
        [pscustomobject]$GenericEndpoint
    )

    $selectionParam = $GenericEndpoint.parameters | Where-Object { $_.name -eq "selections" } | Select-Object -First 1
    if (-not $selectionParam) {
        return [ordered]@{
            schema_ref = $null
            schema_name = $null
            enum = @()
            accepts_unknown_strings = $false
            fallback_to_v1 = @()
            unavailable_in_v2 = @()
        }
    }

    $schemaRef = $selectionParam.schema.items.PSObject.Properties["`$ref"].Value
    $schemaName = Get-RefName -Ref $schemaRef
    $selectionSchema = Resolve-JsonRef -Spec $Spec -Ref $schemaRef

    $enumValues = New-Object System.Collections.Generic.List[string]
    $acceptsUnknownStrings = $false
    $fallbackValues = New-Object System.Collections.Generic.List[string]
    $unavailableValues = New-Object System.Collections.Generic.List[string]

    if ($selectionSchema) {
        if ($selectionSchema.enum) {
            foreach ($value in $selectionSchema.enum) {
                $enumValues.Add([string]$value)
            }
        }

        if ($selectionSchema.oneOf) {
            foreach ($variant in $selectionSchema.oneOf) {
                if ($variant.enum) {
                    foreach ($value in $variant.enum) {
                        $enumValues.Add([string]$value)
                    }
                }

                if ($variant.type -eq "string" -and -not $variant.enum) {
                    $acceptsUnknownStrings = $true
                }

                if ($variant.description) {
                    $fallback = Get-ListFromDescriptionSection -Description $variant.description -Marker "fallback to API v1" -UntilMarker "not available in API v2"
                    foreach ($value in $fallback) {
                        $fallbackValues.Add([string]$value)
                    }

                    $unavailable = Get-ListFromDescriptionSection -Description $variant.description -Marker "not available in API v2" -UntilMarker ""
                    foreach ($value in $unavailable) {
                        $unavailableValues.Add([string]$value)
                    }
                }
            }
        }
    }

    $uniqueEnums = @($enumValues.ToArray() | Sort-Object -Unique)
    $uniqueFallback = @($fallbackValues.ToArray() | Sort-Object -Unique)
    $uniqueUnavailable = @($unavailableValues.ToArray() | Sort-Object -Unique)

    return [ordered]@{
        schema_ref = $schemaRef
        schema_name = $schemaName
        enum = $uniqueEnums
        accepts_unknown_strings = $acceptsUnknownStrings
        fallback_to_v1 = $uniqueFallback
        unavailable_in_v2 = $uniqueUnavailable
    }
}

function Resolve-SelectionName {
    param(
        [string]$SelectionHint,
        [string[]]$SelectionEnum,
        [hashtable]$EndpointSelectionOverrides
    )

    if ([string]::IsNullOrWhiteSpace($SelectionHint)) {
        return $null
    }

    $hintNorm = Normalize-SelectionKey -Value $SelectionHint
    foreach ($selection in $SelectionEnum) {
        if ((Normalize-SelectionKey -Value $selection) -eq $hintNorm) {
            return $selection
        }
    }

    if ($SelectionHint.EndsWith("s")) {
        $singularHintNorm = Normalize-SelectionKey -Value $SelectionHint.TrimEnd("s")
        foreach ($selection in $SelectionEnum) {
            if ((Normalize-SelectionKey -Value $selection) -eq $singularHintNorm) {
                return $selection
            }
        }
    }

    return $null
}

function Get-OperationParameters {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Spec,
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Operation
    )

    $result = New-Object System.Collections.Generic.List[object]
    foreach ($param in @($Operation.parameters)) {
        $result.Add((Get-ParameterCapability -Spec $Spec -ParameterObject $param))
    }
    return $result.ToArray()
}

function Get-EndpointCapability {
    param(
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Spec,
        [Parameter(Mandatory = $true)]
        [string]$ResourceName,
        [Parameter(Mandatory = $true)]
        [string]$Path,
        [Parameter(Mandatory = $true)]
        [pscustomobject]$Operation
    )

    $parameters = Get-OperationParameters -Spec $Spec -Operation $Operation
    $responseSchema = Get-ResponseJsonSchema -Operation $Operation
    $responseSchemaRefs = @()
    $responseTopLevelFields = @()
    $responseRequiredFields = @()
    $responseFieldPaths = @()

    if ($responseSchema) {
        $refs = Collect-SchemaRefs -Schema $responseSchema
        $responseSchemaRefs = @(Convert-ToStringArray -Value $refs | Sort-Object -Unique)

        $topFieldsSet = New-Object 'System.Collections.Generic.HashSet[string]'
        $requiredSet = New-Object 'System.Collections.Generic.HashSet[string]'

        foreach ($schemaRef in $responseSchemaRefs) {
            $resolved = Resolve-JsonRef -Spec $Spec -Ref $schemaRef
            if ($resolved) {
                $topFields = Get-SchemaTopLevelProperties -Spec $Spec -Schema $resolved
                foreach ($field in $topFields) {
                    if ($field -ne "_metadata") {
                        [void]$topFieldsSet.Add($field)
                    }
                }

                $requiredFields = Get-SchemaRequiredTopLevelFields -Spec $Spec -Schema $resolved
                foreach ($field in $requiredFields) {
                    if ($field -ne "_metadata") {
                        [void]$requiredSet.Add($field)
                    }
                }
            }
        }

        $responseTopLevelFields = @(Convert-ToStringArray -Value $topFieldsSet | Sort-Object -Unique)
        $responseRequiredFields = @(Convert-ToStringArray -Value $requiredSet | Sort-Object -Unique)

        $fieldPaths = Collect-SchemaFieldPaths -Spec $Spec -Schema $responseSchema -Prefix ""
        $responseFieldPaths = @(Convert-ToStringArray -Value $fieldPaths | Sort-Object -Unique)
    }

    $selectionHint = Get-SelectionHintFromPath -ResourceName $ResourceName -Path $Path
    $standaloneOnly = $false
    if ($Operation.description -and $Operation.description -match "standalone and cannot be used together with other selections") {
        $standaloneOnly = $true
    }

    return [ordered]@{
        path = $Path
        method = "get"
        operation_id = $Operation.operationId
        summary = $Operation.summary
        description = $Operation.description
        stability = $Operation.'x-stability'
        selection_hint = $selectionHint
        standalone_only = $standaloneOnly
        parameters = $parameters
        response_schema_refs = $responseSchemaRefs
        response_top_level_fields = $responseTopLevelFields
        response_required_fields = $responseRequiredFields
        response_field_paths = $responseFieldPaths
    }
}

function Merge-UniqueStrings {
    param(
        [string[]]$Existing,
        [string[]]$Incoming
    )
    return ,(@(@($Existing) + @($Incoming) | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Sort-Object -Unique))
}

function Convert-ToStringArray {
    param($Value)

    if ($null -eq $Value) {
        return @()
    }

    if ($Value -is [string]) {
        return @([string]$Value)
    }

    $output = New-Object System.Collections.Generic.List[string]
    foreach ($item in $Value) {
        if ($null -ne $item -and -not [string]::IsNullOrWhiteSpace([string]$item)) {
            $output.Add([string]$item)
        }
    }

    return $output.ToArray()
}

$repoRoot = Get-RepoRoot
if ([string]::IsNullOrWhiteSpace($SpecPath)) {
    $SpecPath = Join-Path $repoRoot "spec/torn-openapi-current.json"
}
if ([string]::IsNullOrWhiteSpace($OutputPath)) {
    $OutputPath = Join-Path $repoRoot "spec/capabilities.json"
}
if ([string]::IsNullOrWhiteSpace($OverridesPath)) {
    $OverridesPath = Join-Path $repoRoot "spec/capability-overrides.json"
}

if (-not (Test-Path -Path $SpecPath)) {
    throw "Spec file not found: $SpecPath"
}

$raw = Get-Content -Path $SpecPath -Raw
$spec = $raw | ConvertFrom-Json

$overrides = [ordered]@{
    endpoint_selection_overrides = [ordered]@{}
    standalone_selections = [ordered]@{}
    selection_notes = [ordered]@{}
}

if (Test-Path -Path $OverridesPath) {
    $overrideRaw = Get-Content -Path $OverridesPath -Raw
    if (-not [string]::IsNullOrWhiteSpace($overrideRaw)) {
        $parsed = $overrideRaw | ConvertFrom-Json
        if ($parsed.endpoint_selection_overrides) { $overrides.endpoint_selection_overrides = $parsed.endpoint_selection_overrides }
        if ($parsed.standalone_selections) { $overrides.standalone_selections = $parsed.standalone_selections }
        if ($parsed.selection_notes) { $overrides.selection_notes = $parsed.selection_notes }
    }
}

$paths = $spec.paths.PSObject.Properties
$resourceNames = @($paths.Name | ForEach-Object {
    $segments = $_.Trim("/").Split("/")
    if ($segments.Length -gt 0) { $segments[0] }
} | Sort-Object -Unique)

$resourcesOutput = [ordered]@{}
$allEndpoints = New-Object System.Collections.Generic.List[object]

foreach ($resourceName in $resourceNames) {
    $resourcePathPrefix = "/$resourceName"
    $resourcePaths = $paths | Where-Object { $_.Name -eq $resourcePathPrefix -or $_.Name.StartsWith("$resourcePathPrefix/") }
    $getPaths = $resourcePaths | Where-Object { $_.Value.get }

    $genericPathEntry = $getPaths | Where-Object { $_.Name -eq $resourcePathPrefix } | Select-Object -First 1
    $lookupPath = "$resourcePathPrefix/lookup"
    $timestampPath = "$resourcePathPrefix/timestamp"

    $endpointCapabilities = New-Object System.Collections.Generic.List[object]
    foreach ($entry in $getPaths) {
        $cap = Get-EndpointCapability -Spec $spec -ResourceName $resourceName -Path $entry.Name -Operation $entry.Value.get
        $endpointCapabilities.Add($cap)
        $allEndpoints.Add($cap)
    }

    $genericEndpoint = $null
    $selectionSchemaInfo = [ordered]@{
        schema_ref = $null
        schema_name = $null
        enum = @()
        accepts_unknown_strings = $false
        fallback_to_v1 = @()
        unavailable_in_v2 = @()
    }
    $genericParameters = @()
    $genericSupportsSelections = $false
    $genericSupportsLegacy = $false

    if ($genericPathEntry) {
        $genericEndpoint = Get-EndpointCapability -Spec $spec -ResourceName $resourceName -Path $genericPathEntry.Name -Operation $genericPathEntry.Value.get
        $genericParameters = @($genericEndpoint.parameters)
        $genericSupportsSelections = $genericParameters.name -contains "selections"
        $genericSupportsLegacy = $genericParameters.name -contains "legacy"
        if ($genericSupportsSelections) {
            $selectionSchemaInfo = Get-SelectionSchemaInfo -Spec $spec -GenericEndpoint $genericPathEntry.Value.get
        }
    }

    $selectionMap = [ordered]@{}
    foreach ($selection in @($selectionSchemaInfo.enum)) {
        $selectionMap[$selection] = [ordered]@{
            name = $selection
            in_selection_enum = $true
            standalone_only = $false
            fallback_to_v1 = $selectionSchemaInfo.fallback_to_v1 -contains $selection
            unavailable_in_v2 = $selectionSchemaInfo.unavailable_in_v2 -contains $selection
            endpoints = @()
            query_parameters = @()
            required_path_parameters = @()
            response_schema_refs = @()
            response_top_level_fields = @()
            response_field_paths = @()
            notes = @()
        }
    }

    $unmatchedEndpoints = New-Object System.Collections.Generic.List[string]
    foreach ($endpoint in $endpointCapabilities) {
        if ($endpoint.path -eq $resourcePathPrefix -or $endpoint.path -eq $lookupPath -or $endpoint.path -eq $timestampPath) {
            continue
        }

        $effectiveHint = $endpoint.selection_hint
        $overrideValue = $overrides.endpoint_selection_overrides.PSObject.Properties[$endpoint.path]
        if ($overrideValue) {
            $effectiveHint = [string]$overrideValue.Value
        }

        $matchedSelection = Resolve-SelectionName -SelectionHint $effectiveHint -SelectionEnum @($selectionSchemaInfo.enum)
        if (-not $matchedSelection -and -not [string]::IsNullOrWhiteSpace($effectiveHint)) {
            $matchedSelection = $effectiveHint.ToLowerInvariant()
            if (-not $selectionMap.Contains($matchedSelection)) {
                $selectionMap[$matchedSelection] = [ordered]@{
                    name = $matchedSelection
                    in_selection_enum = $false
                    standalone_only = $false
                    fallback_to_v1 = $false
                    unavailable_in_v2 = $false
                    endpoints = @()
                    query_parameters = @()
                    required_path_parameters = @()
                    response_schema_refs = @()
                    response_top_level_fields = @()
                    response_field_paths = @()
                    notes = @("Discovered from dedicated endpoint path; not present in generic selection enum.")
                }
            }
        }

        if (-not $matchedSelection -or -not $selectionMap.Contains($matchedSelection)) {
            $unmatchedEndpoints.Add($endpoint.path)
            continue
        }

        $entry = $selectionMap[$matchedSelection]
        $entry.endpoints = Merge-UniqueStrings -Existing @($entry.endpoints) -Incoming @($endpoint.path)

        $queryParams = @($endpoint.parameters | Where-Object { $_.in -eq "query" } | ForEach-Object { $_.name })
        $entry.query_parameters = Merge-UniqueStrings -Existing @($entry.query_parameters) -Incoming $queryParams

        $requiredPathParams = @($endpoint.parameters | Where-Object { $_.in -eq "path" -and $_.required } | ForEach-Object { $_.name })
        $entry.required_path_parameters = Merge-UniqueStrings -Existing @($entry.required_path_parameters) -Incoming $requiredPathParams

        $entry.response_schema_refs = Merge-UniqueStrings -Existing @($entry.response_schema_refs) -Incoming @($endpoint.response_schema_refs)
        $entry.response_top_level_fields = Merge-UniqueStrings -Existing @($entry.response_top_level_fields) -Incoming @($endpoint.response_top_level_fields)
        $entry.response_field_paths = Merge-UniqueStrings -Existing @($entry.response_field_paths) -Incoming @($endpoint.response_field_paths)

        if ($endpoint.standalone_only) {
            $entry.standalone_only = $true
        }

        $selectionMap[$matchedSelection] = $entry
    }

    $resourceStandaloneOverride = $overrides.standalone_selections.PSObject.Properties[$resourceName]
    if ($resourceStandaloneOverride) {
        foreach ($selectionName in @($resourceStandaloneOverride.Value)) {
            $normalizedName = [string]$selectionName
            if ($selectionMap.Contains($normalizedName)) {
                $selectionMap[$normalizedName].standalone_only = $true
            }
        }
    }

    $resourceNotesOverride = $overrides.selection_notes.PSObject.Properties[$resourceName]
    if ($resourceNotesOverride) {
        foreach ($selectionNoteProp in $resourceNotesOverride.Value.PSObject.Properties) {
            $selectionName = $selectionNoteProp.Name
            if ($selectionMap.Contains($selectionName)) {
                $existingNotes = @($selectionMap[$selectionName].notes)
                $selectionMap[$selectionName].notes = Merge-UniqueStrings -Existing $existingNotes -Incoming @($selectionNoteProp.Value)
            }
        }
    }

    $selectionEntries = @($selectionMap.Values | Sort-Object -Property name)
    foreach ($selectionEntry in $selectionEntries) {
        $selectionEntry.can_use_generic_endpoint = $genericSupportsSelections -and $selectionEntry.in_selection_enum -and -not $selectionEntry.unavailable_in_v2
        $selectionEntry.requires_direct_endpoint_only = -not $selectionEntry.can_use_generic_endpoint
    }

    $genericFilterParameters = @($genericParameters | Where-Object {
        $_.name -in @("limit", "from", "to", "sort", "cat", "stat", "filters", "striptags", "offset", "timestamp")
    })

    $resourceOutput = [ordered]@{
        generic_path = if ($genericPathEntry) { $resourcePathPrefix } else { $null }
        lookup_path = if ($getPaths.Name -contains $lookupPath) { $lookupPath } else { $null }
        timestamp_path = if ($getPaths.Name -contains $timestampPath) { $timestampPath } else { $null }
        selection_schema = $selectionSchemaInfo.schema_name
        selection_schema_ref = $selectionSchemaInfo.schema_ref
        selections_accept_unknown_strings = [bool]$selectionSchemaInfo.accepts_unknown_strings
        selection_count = @($selectionSchemaInfo.enum).Count
        generic_supports_legacy = [bool]$genericSupportsLegacy
        generic_parameters = $genericParameters
        generic_filter_parameters = $genericFilterParameters
        selections = $selectionEntries
        unmatched_selection_endpoints = @($unmatchedEndpoints | Sort-Object -Unique)
        endpoints = @($endpointCapabilities | Sort-Object -Property path)
    }

    $resourcesOutput[$resourceName] = $resourceOutput
}

$parametersCatalog = New-Object System.Collections.Generic.List[object]
foreach ($parameterProperty in $spec.components.parameters.PSObject.Properties) {
    $parameter = $parameterProperty.Value
    $schemaSummary = Get-SchemaSummary -Schema $parameter.schema
    $entry = [ordered]@{
        name = $parameter.name
        component_name = $parameterProperty.Name
        in = $parameter.in
        required = [bool]$parameter.required
    }
    if ($parameter.description) { $entry.description = $parameter.description }
    if ($schemaSummary.Keys.Count -gt 0) { $entry.schema = $schemaSummary }
    $parametersCatalog.Add($entry)
}

$output = [ordered]@{
    spec = [ordered]@{
        title = $spec.info.title
        version = $spec.info.version
        openapi = $spec.openapi
        source_file = Get-RepoRelativePath -Path $SpecPath -RepoRoot $repoRoot
    }
    planner_notes = @(
        "Selection compatibility is not fully machine-readable in the OpenAPI spec. Use `standalone_only` flags, then fallback to request splitting on API errors.",
        "Use `can_use_generic_endpoint=true` selections for request batching via generic `/resource` endpoints.",
        "`generic_filter_parameters` lists filters accepted by generic endpoints, but filter relevance can still vary by selection."
    )
    resources = $resourcesOutput
    parameters = @($parametersCatalog | Sort-Object -Property component_name)
}

$outputDir = Split-Path -Path $OutputPath -Parent
if (-not (Test-Path -Path $outputDir)) {
    New-Item -Path $outputDir -ItemType Directory -Force | Out-Null
}

$output | ConvertTo-Json -Depth 40 | Set-Content -Path $OutputPath -Encoding UTF8

Write-Output ("Generated capabilities from Torn OpenAPI v{0}" -f $spec.info.version)
Write-Output ("Output file: {0}" -f (Resolve-Path $OutputPath).Path)
