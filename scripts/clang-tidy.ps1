[CmdletBinding()]
param(
    [switch]$DebugBuild,
    [switch]$NoBuild,
    [string]$Checks = "clang-analyzer-*"
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$ClangTidy = (Get-Command clang-tidy -CommandType Application -ErrorAction Stop).Source
$Profile = if ($DebugBuild) { "debug" } else { "release" }
$BuildDirectory = Join-Path $RepositoryRoot "target\aegilex\$Profile"
$CompilationDatabase = Join-Path $BuildDirectory "compile_commands.json"

if (-not $NoBuild) {
    $BuildArguments = @("xtask", "build")
    if ($DebugBuild) {
        $BuildArguments += "--debug"
    }
    & cargo @BuildArguments
    if ($LASTEXITCODE -ne 0) {
        throw "cargo xtask build failed."
    }
}

if (-not (Test-Path -LiteralPath $CompilationDatabase -PathType Leaf)) {
    throw "Missing compilation database: $CompilationDatabase. Run without -NoBuild to create it."
}

$NativeRoot = (Resolve-Path (Join-Path $RepositoryRoot "native")).Path
$NativePrefix = "$NativeRoot$([IO.Path]::DirectorySeparatorChar)"
$Entries = Get-Content -LiteralPath $CompilationDatabase -Raw | ConvertFrom-Json
$Files = @(
    $Entries |
        Where-Object {
            $File = [IO.Path]::GetFullPath($_.file)
            $IsNative = $File.StartsWith($NativePrefix, [StringComparison]::OrdinalIgnoreCase)
            $IsPluginTarget = $_.output -match '[\\/]CMakeFiles[\\/]aegilex\.dir[\\/]'
            $IsNative -and $IsPluginTarget
        } |
        ForEach-Object { [IO.Path]::GetFullPath($_.file) } |
        Sort-Object -Unique
)

if ($Files.Count -eq 0) {
    throw "The compilation database contains no production Aegilex native translation units."
}

$Arguments = @(
    "-p", $BuildDirectory,
    "-checks=$Checks",
    "--warnings-as-errors=$Checks"
)
$Failed = $false
foreach ($file in $Files) {
    & $ClangTidy @Arguments $file
    if ($LASTEXITCODE -ne 0) {
        $Failed = $true
    }
}

if ($Failed) {
    throw "clang-tidy reported diagnostics."
}
