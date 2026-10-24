param(
    [ValidateSet('Build', 'Dev', 'Test')]
    [string]$Action = 'Build'
)
$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)
$env:Path = [Environment]::GetEnvironmentVariable('Path', 'Machine') + ';' + [Environment]::GetEnvironmentVariable('Path', 'User') + ';' + $env:Path
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path
if (-not (Get-Command bun -ErrorAction SilentlyContinue)) {
    $bunExe = Get-ChildItem "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\Oven-sh.Bun_*" -Recurse -Filter bun.exe -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($bunExe) { $env:Path = $bunExe.DirectoryName + ';' + $env:Path }
}
foreach ($tool in 'bun', 'cargo', 'cmake') {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) { throw "Missing prerequisite: $tool" }
}
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vswhere)) { throw 'Install Visual Studio Build Tools with the C++ workload.' }
$vs = & $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $vs) { throw 'The Visual Studio C++ workload is not installed.' }
$env:VULKAN_SDK = [Environment]::GetEnvironmentVariable('VULKAN_SDK', 'Machine')
if (-not $env:VULKAN_SDK) {
    $sdk = Get-ChildItem C:\VulkanSDK -Directory -ErrorAction SilentlyContinue | Sort-Object Name -Descending | Select-Object -First 1
    if ($sdk) { $env:VULKAN_SDK = $sdk.FullName }
}
if (-not $env:VULKAN_SDK) { throw 'Install the Vulkan SDK.' }
$env:Path = "$env:VULKAN_SDK\Bin;" + $env:Path
$redist = Get-ChildItem "$vs\VC\Redist\MSVC\*\x64\Microsoft.VC*.CRT" -Directory | Sort-Object FullName -Descending | Select-Object -First 1
if ($redist) { $env:HANDY_VC_REDIST_DIRS = $redist.FullName }
# Keep build paths short on Windows, including for checkouts containing spaces.
$env:CARGO_TARGET_DIR = "$env:LOCALAPPDATA\VozPrompt\target"
$env:CARGO_BUILD_JOBS = '2'
$env:HANDY_DISABLE_UPDATER = '1'
if ($Action -eq 'Test') {
    & cargo test --manifest-path src-tauri/Cargo.toml -p vozprompt-core
} elseif ($Action -eq 'Dev') {
    & bun run tauri dev --config src-tauri/tauri.local-windows.conf.json
} else {
    & bun run tauri build --config src-tauri/tauri.local-windows.conf.json --bundles nsis
}
if ($LASTEXITCODE -ne 0) { throw "$Action failed with exit code $LASTEXITCODE" }
