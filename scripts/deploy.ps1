[CmdletBinding()]
param(
    [string]$ServerRoot = (Join-Path $PSScriptRoot "..\..\server\bedrock_server")
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$ServerRoot = (Resolve-Path $ServerRoot).Path
$PluginsRoot = Join-Path $ServerRoot "plugins"
$NativeSource = Join-Path $RepositoryRoot "target\aegilex\release\endstone_aegilex.dll"
$ComponentSource = Join-Path $RepositoryRoot "examples\hello-component\target\wasm32-wasip1\release\aegilex_hello_component.wasm"
$PolicySource = Join-Path $RepositoryRoot "examples\hello-component\aegilex.toml"
$EventTestSource = Join-Path $RepositoryRoot "examples\event-test-component\target\wasm32-wasip1\release\aegilex_event_test_component.wasm"
$EventTestPolicySource = Join-Path $RepositoryRoot "examples\event-test-component\aegilex.toml"
$FullTestSource = Join-Path $RepositoryRoot "examples\full-test-component\target\wasm32-wasip1\release\aegilex_full_test_component.wasm"
$FullTestPolicySource = Join-Path $RepositoryRoot "examples\full-test-component\aegilex.toml"
$NativeDestination = Join-Path $PluginsRoot "endstone_aegilex.dll"
$ComponentDirectory = Join-Path $PluginsRoot "example_hello"
$ComponentDestination = Join-Path $ComponentDirectory "plugin.wasm"
$PolicyDestination = Join-Path $ComponentDirectory "aegilex.toml"
$EventTestDirectory = Join-Path $PluginsRoot "event_test"
$EventTestDestination = Join-Path $EventTestDirectory "plugin.wasm"
$EventTestPolicyDestination = Join-Path $EventTestDirectory "aegilex.toml"
$FullTestDirectory = Join-Path $PluginsRoot "full_test"
$FullTestDestination = Join-Path $FullTestDirectory "plugin.wasm"
$FullTestPolicyDestination = Join-Path $FullTestDirectory "aegilex.toml"

if (-not (Test-Path -LiteralPath $PluginsRoot -PathType Container)) {
    throw "Endstone plugins directory does not exist: $PluginsRoot"
}

Push-Location $RepositoryRoot
try {
    & cargo xtask build
    if ($LASTEXITCODE -ne 0) {
        throw "Native Aegilex build failed with exit code $LASTEXITCODE."
    }

    & cargo build --release --target wasm32-wasip1 --manifest-path "examples/hello-component/Cargo.toml"
    if ($LASTEXITCODE -ne 0) {
        throw "Hello component build failed with exit code $LASTEXITCODE."
    }

    & cargo build --release --target wasm32-wasip1 --manifest-path "examples/event-test-component/Cargo.toml"
    if ($LASTEXITCODE -ne 0) {
        throw "Event test component build failed with exit code $LASTEXITCODE."
    }

    & cargo build --release --target wasm32-wasip1 --manifest-path "examples/full-test-component/Cargo.toml"
    if ($LASTEXITCODE -ne 0) {
        throw "Full test component build failed with exit code $LASTEXITCODE."
    }
}
finally {
    Pop-Location
}

if (-not (Test-Path -LiteralPath $NativeSource -PathType Leaf)) {
    throw "Native build artifact is missing: $NativeSource"
}
if (-not (Test-Path -LiteralPath $ComponentSource -PathType Leaf)) {
    throw "Hello component artifact is missing: $ComponentSource"
}
if (-not (Test-Path -LiteralPath $PolicySource -PathType Leaf)) {
    throw "Hello component authorization policy is missing: $PolicySource"
}
if (-not (Test-Path -LiteralPath $EventTestSource -PathType Leaf)) {
    throw "Event test component artifact is missing: $EventTestSource"
}
if (-not (Test-Path -LiteralPath $EventTestPolicySource -PathType Leaf)) {
    throw "Event test component authorization policy is missing: $EventTestPolicySource"
}
if (-not (Test-Path -LiteralPath $FullTestSource -PathType Leaf)) {
    throw "Full test component artifact is missing: $FullTestSource"
}
if (-not (Test-Path -LiteralPath $FullTestPolicySource -PathType Leaf)) {
    throw "Full test component authorization policy is missing: $FullTestPolicySource"
}

New-Item -ItemType Directory -Path $ComponentDirectory -Force | Out-Null
New-Item -ItemType Directory -Path $EventTestDirectory -Force | Out-Null
New-Item -ItemType Directory -Path $FullTestDirectory -Force | Out-Null
Copy-Item -LiteralPath $NativeSource -Destination $NativeDestination -Force
Copy-Item -LiteralPath $ComponentSource -Destination $ComponentDestination -Force
Copy-Item -LiteralPath $PolicySource -Destination $PolicyDestination -Force
Copy-Item -LiteralPath $EventTestSource -Destination $EventTestDestination -Force
Copy-Item -LiteralPath $EventTestPolicySource -Destination $EventTestPolicyDestination -Force
Copy-Item -LiteralPath $FullTestSource -Destination $FullTestDestination -Force
Copy-Item -LiteralPath $FullTestPolicySource -Destination $FullTestPolicyDestination -Force

foreach ($artifact in @(
    @{ Source = $NativeSource; Destination = $NativeDestination },
    @{ Source = $ComponentSource; Destination = $ComponentDestination },
    @{ Source = $PolicySource; Destination = $PolicyDestination },
    @{ Source = $EventTestSource; Destination = $EventTestDestination },
    @{ Source = $EventTestPolicySource; Destination = $EventTestPolicyDestination },
    @{ Source = $FullTestSource; Destination = $FullTestDestination },
    @{ Source = $FullTestPolicySource; Destination = $FullTestPolicyDestination }
)) {
    $sourceHash = (Get-FileHash -LiteralPath $artifact.Source -Algorithm SHA256).Hash
    $destinationHash = (Get-FileHash -LiteralPath $artifact.Destination -Algorithm SHA256).Hash
    if ($sourceHash -ne $destinationHash) {
        throw "Deployment verification failed: $($artifact.Destination)"
    }
    "Deployed $($artifact.Destination) ($sourceHash)"
}

"Restart Endstone to load the deployed Aegilex DLL and component."
