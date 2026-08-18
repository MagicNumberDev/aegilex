[CmdletBinding()]
param(
    [switch]$Fix
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$ClangFormat = (Get-Command clang-format -CommandType Application -ErrorAction Stop).Source
$Extensions = @(".c", ".cc", ".cpp", ".cxx", ".h", ".hh", ".hpp", ".hxx")
$SourceRoots = @("examples", "include", "native", "tests")
$Files = foreach ($sourceRoot in $SourceRoots) {
    $path = Join-Path $RepositoryRoot $sourceRoot
    if (Test-Path -LiteralPath $path -PathType Container) {
        Get-ChildItem -LiteralPath $path -File -Recurse |
            Where-Object { $Extensions -contains $_.Extension } |
            ForEach-Object FullName
    }
}
$Files = @($Files | Sort-Object -Unique)

if ($Files.Count -eq 0) {
    throw "No C or C++ source files found."
}

$Arguments = @("--style=file", "--fallback-style=none")
if ($Fix) {
    $Arguments += "-i"
}
else {
    $Arguments += "--dry-run"
    $Arguments += "--Werror"
}

& $ClangFormat @Arguments @Files
if ($LASTEXITCODE -ne 0) {
    if ($Fix) {
        throw "clang-format failed."
    }
    throw "clang-format found files that do not match .clang-format. Run .\scripts\clang-format.ps1 -Fix to update them."
}
