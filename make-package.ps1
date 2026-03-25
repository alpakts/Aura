# 📦 Aura Offline Packaging Script (v3 - ASCII ONLY - VSCode Support)
$PackageDir = "$PSScriptRoot\Aura-SDK"
$BinDir = "$PackageDir\bin"
$LibDir = "$PackageDir\lib"
$VSCodeExtDest = "$PackageDir\vscode-extension"

Write-Host "--- Aura Offline Package Preparing ---" -ForegroundColor Cyan

# 1. Clean & Setup Folders
if (Test-Path $PackageDir) { Remove-Item $PackageDir -Recurse -Force }
New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
New-Item -ItemType Directory -Path $LibDir -Force | Out-Null

# 2. Build Aura Compiler
Write-Host "Building Aura Compiler (Release)..." -ForegroundColor Yellow
cd "$PSScriptRoot\compiler"
& cargo build --release | Out-Null
Copy-Item "target\release\aura.exe" -Destination "$BinDir\aura.exe"

# 3. Find Clang and LLD
Write-Host "Collecting Clang binaries..." -ForegroundColor Yellow
$ClangPath = Get-Command "clang" -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
if ($ClangPath) {
    $ClangDir = Split-Path $ClangPath
    Copy-Item "$ClangDir\clang.exe" -Destination "$BinDir\clang.exe"
    Copy-Item "$ClangDir\lld-link.exe" -Destination "$BinDir\lld-link.exe" -ErrorAction SilentlyContinue
    Copy-Item "$ClangDir\*.dll" -Destination "$BinDir\" -ErrorAction SilentlyContinue
}

# 4. Search for .lib files
Write-Host "Collecting Windows SDK .lib files..." -ForegroundColor Yellow
$RequiredLibs = @("msvcrt.lib", "kernel32.lib", "user32.lib", "ucrt.lib", "vcruntime.lib", "legacy_stdio_definitions.lib")
$SearchRoots = @(
    "C:\Program Files (x86)\Windows Kits\10\Lib",
    "C:\Program Files\Microsoft Visual Studio\2022\*\VC\Tools\MSVC\*\lib\x64"
)

foreach ($lib in $RequiredLibs) {
    $found = $false
    foreach ($root in $SearchRoots) {
        if (Test-Path $root) {
            $matches = Get-ChildItem -Path $root -Filter $lib -Recurse -File -ErrorAction SilentlyContinue | Where-Object { $_.FullName -like "*x64*" }
            if ($matches) {
                Copy-Item $matches[0].FullName -Destination "$LibDir\"
                $found = $true
                Write-Host "Found: $lib" -ForegroundColor Green
                break
            }
        }
    }
}

# 5. Prepare VS Code Extension
Write-Host "`nPackaging VS Code Extension..." -ForegroundColor Yellow
$VSCodeExtSource = "$PSScriptRoot\vscode-extension"

if (Test-Path $VSCodeExtSource) {
    Copy-Item $VSCodeExtSource -Destination $VSCodeExtDest -Recurse -Force
    Write-Host "Extension copied successfully." -ForegroundColor Green
} else {
    Write-Host "Warning: vscode-extension folder not found." -ForegroundColor Red
}

Write-Host "`n--- FINAL CHECK: Aura-SDK is ready! ---" -ForegroundColor Cyan
Write-Host "Path: $PackageDir"
