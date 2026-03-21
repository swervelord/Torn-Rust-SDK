Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$syncScript = Join-Path $PSScriptRoot "sync-torn-openapi.ps1"
& $syncScript @args
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$capabilitiesScript = Join-Path $PSScriptRoot "generate-capabilities.ps1"
& $capabilitiesScript
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$metadataPath = Join-Path $repoRoot "spec/metadata.json"
if (-not (Test-Path -Path $metadataPath)) {
    throw "Expected metadata file not found: $metadataPath"
}

$metadata = Get-Content -Path $metadataPath -Raw | ConvertFrom-Json
Write-Output ("Project start synced to Torn OpenAPI v{0}" -f $metadata.current_version)
Write-Output ("Use this spec path in tooling: {0}" -f $metadata.current_file)
Write-Output "Capabilities file updated: spec/capabilities.json"
