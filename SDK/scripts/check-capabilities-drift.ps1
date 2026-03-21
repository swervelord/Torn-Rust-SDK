param(
    [string]$SpecPath,
    [string]$CapabilitiesPath,
    [string]$OverridesPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Get-RepoRoot {
    return (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
}

$repoRoot = Get-RepoRoot
if ([string]::IsNullOrWhiteSpace($SpecPath)) {
    $SpecPath = Join-Path $repoRoot "spec/torn-openapi-current.json"
}
if ([string]::IsNullOrWhiteSpace($CapabilitiesPath)) {
    $CapabilitiesPath = Join-Path $repoRoot "spec/capabilities.json"
}
if ([string]::IsNullOrWhiteSpace($OverridesPath)) {
    $OverridesPath = Join-Path $repoRoot "spec/capability-overrides.json"
}

if (-not (Test-Path -Path $SpecPath)) {
    throw "Spec file not found: $SpecPath"
}
if (-not (Test-Path -Path $CapabilitiesPath)) {
    throw "Capabilities file not found: $CapabilitiesPath"
}

$tempOutputPath = Join-Path ([System.IO.Path]::GetTempPath()) ("capabilities-check-{0}.json" -f [Guid]::NewGuid().ToString("N"))

try {
    $generateScript = Join-Path $PSScriptRoot "generate-capabilities.ps1"
    & $generateScript -SpecPath $SpecPath -OutputPath $tempOutputPath -OverridesPath $OverridesPath

    $currentNormalized = (Get-Content -Path $CapabilitiesPath -Raw | ConvertFrom-Json) | ConvertTo-Json -Depth 40 -Compress
    $generatedNormalized = (Get-Content -Path $tempOutputPath -Raw | ConvertFrom-Json) | ConvertTo-Json -Depth 40 -Compress

    if ($currentNormalized -ne $generatedNormalized) {
        Write-Error "Capabilities drift detected: spec/capabilities.json is stale. Run scripts/generate-capabilities.ps1 and commit the result."
        exit 1
    }

    Write-Output "Capabilities are in sync with the generator."
}
finally {
    if (Test-Path -Path $tempOutputPath) {
        Remove-Item -Path $tempOutputPath -Force
    }
}
