param(
    [string]$SourceUrl = "https://www.torn.com/swagger/openapi.json",
    [string]$UserAgent = "newrustsdk-spec-sync/1.0",
    [switch]$Force
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Get-RepoRoot {
    return (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
}

function Get-JsonVersion {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    $raw = Get-Content -Path $Path -Raw
    $spec = $raw | ConvertFrom-Json
    if (-not $spec) {
        throw "Downloaded file is empty or invalid JSON."
    }

    $infoProperty = $spec.PSObject.Properties["info"]
    if (-not $infoProperty) {
        throw "Downloaded JSON is not the Torn OpenAPI spec (missing info object)."
    }

    $versionProperty = $infoProperty.Value.PSObject.Properties["version"]
    if (-not $versionProperty -or -not $versionProperty.Value) {
        throw "Downloaded JSON is not the Torn OpenAPI spec (missing info.version)."
    }

    return [string]$versionProperty.Value
}

$repoRoot = Get-RepoRoot
$specDir = Join-Path $repoRoot "spec"
$versionsDir = Join-Path $specDir "versions"
$currentPath = Join-Path $specDir "torn-openapi-current.json"
$metadataPath = Join-Path $specDir "metadata.json"
$tempPath = Join-Path ([System.IO.Path]::GetTempPath()) ("torn-openapi-{0}.json" -f [Guid]::NewGuid().ToString("N"))

New-Item -ItemType Directory -Path $versionsDir -Force | Out-Null

try {
    $curlCandidates = @("curl.exe", "curl")
    $curl = $null
    foreach ($candidate in $curlCandidates) {
        $resolved = Get-Command $candidate -ErrorAction SilentlyContinue | Where-Object { $_.CommandType -eq "Application" } | Select-Object -First 1
        if ($resolved) {
            $curl = $resolved
            break
        }
    }
    if (-not $curl) {
        throw "curl is required to download the Torn OpenAPI spec."
    }

    & $curl.Source -sS -L -A $UserAgent -H "Accept: application/json" -o $tempPath $SourceUrl
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to download spec from '$SourceUrl' (curl exit code $LASTEXITCODE)."
    }

    $version = Get-JsonVersion -Path $tempPath
    $versionFileName = "torn-openapi-{0}.json" -f $version
    $versionPath = Join-Path $versionsDir $versionFileName
    $newHash = (Get-FileHash -Path $tempPath -Algorithm SHA256).Hash

    $shouldWriteVersionFile = $true
    if (Test-Path -Path $versionPath) {
        $existingHash = (Get-FileHash -Path $versionPath -Algorithm SHA256).Hash
        if ($existingHash -eq $newHash -and -not $Force) {
            $shouldWriteVersionFile = $false
        }
    }

    if ($shouldWriteVersionFile) {
        Copy-Item -Path $tempPath -Destination $versionPath -Force
    }

    Copy-Item -Path $tempPath -Destination $currentPath -Force

    $metadata = [ordered]@{
        source_url = $SourceUrl
        current_version = $version
        current_file = "spec/torn-openapi-current.json"
        version_file = ("spec/versions/{0}" -f $versionFileName)
        sha256 = $newHash
        fetched_at_utc = (Get-Date).ToUniversalTime().ToString("o")
        user_agent = $UserAgent
    }

    $metadata | ConvertTo-Json -Depth 3 | Set-Content -Path $metadataPath -Encoding UTF8

    Write-Output ("Synced Torn OpenAPI spec version {0}" -f $version)
    Write-Output ("Current file: {0}" -f $currentPath)
    Write-Output ("Versioned file: {0}" -f $versionPath)
}
finally {
    if (Test-Path -Path $tempPath) {
        Remove-Item -Path $tempPath -Force
    }
}
