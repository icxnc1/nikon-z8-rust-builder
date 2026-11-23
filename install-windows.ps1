# Nikon Z8 Rust Builder / Windows 11 x64 toolchain and WSL2 optional setup.
# Default: check only. Explicit -Install enables package installations.
[CmdletBinding()]
param(
  [switch]$Install,
  [switch]$InstallWSL2,
  [switch]$InstallMSVC,
  [switch]$RunTests
)
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
Write-Output 'Nikon Z8 / Native Windows Rust + Ubuntu WSL2 toolchain'
if (![Environment]::Is64BitOperatingSystem) { throw 'Windows x64 required' }
if ($Install -and !(Get-Command winget.exe -ErrorAction SilentlyContinue)) {
  throw 'winget unavailable. Install Microsoft App Installer from official Microsoft sources.'
}
if ($Install) {
  $packages = @(
    'Git.Git',
    'Rustlang.Rustup',
    'LLVM.LLVM',
    'Kitware.CMake',
    'Ninja-build.Ninja',
    'Python.Python.3.13',
    'OpenJS.NodeJS.LTS'
  )
  foreach ($pkg in $packages) {
    Write-Output ('winget install ' + $pkg)
    & winget.exe install --exact --id $pkg --source winget --silent --disable-interactivity --accept-source-agreements --accept-package-agreements
    if ($LASTEXITCODE -ne 0) { throw "winget failed for $pkg : $LASTEXITCODE" }
  }
  if ($InstallMSVC) {
    Write-Output 'Install native Visual Studio C++ build tools and Windows SDK; requires administrator rights'
    & winget.exe install --exact --id 'Microsoft.VisualStudio.2022.BuildTools' --source winget --silent --disable-interactivity --accept-source-agreements --accept-package-agreements --override '--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended'
    if ($LASTEXITCODE -ne 0) { throw 'Visual Studio C++ Build Tools install failed' }
  }
}
if ($InstallWSL2) {
  if (!$Install) { throw '-InstallWSL2 requires -Install to avoid unintended system changes' }
  $list = (& wsl.exe --list --quiet 2>&1) -join [Environment]::NewLine
  if ($list -notmatch 'Ubuntu-26.04') {
    Write-Output 'Installing Ubuntu-26.04 via official WSL distribution mechanism'
    & wsl.exe --install -d Ubuntu-26.04
    if ($LASTEXITCODE -ne 0) { throw 'WSL Ubuntu-26.04 install failed; check wsl --list --online and Windows virtualization settings' }
    Write-Warning 'WSL installation can require a restart and first-run Linux user creation'
  } else { Write-Output 'Ubuntu-26.04 already exists in WSL; not changing it' }
}
$paths = @(
  (Join-Path $env:USERPROFILE '.cargo\bin'),
  'C:\Program Files\Git\cmd',
  'C:\Program Files\CMake\bin',
  'C:\Program Files\LLVM\bin',
  'C:\Program Files\nodejs'
)
foreach ($p in $paths) {
  if ((Test-Path -LiteralPath $p) -and (($env:PATH -split ';') -notcontains $p)) {
    $env:PATH += (';' + $p)
  }
}
$required = @('git.exe','rustc.exe','cargo.exe','node.exe')
$recommended = @('cmake.exe','python.exe')
$missing = @()
foreach ($tool in $required) {
  $cmd = Get-Command $tool -ErrorAction SilentlyContinue
  if ($cmd) { Write-Output ('PASS ' + $tool + ': ' + $cmd.Source) }
  else { $missing += $tool; Write-Output ('MISSING ' + $tool) }
}
foreach ($tool in $recommended) {
  $cmd = Get-Command $tool -ErrorAction SilentlyContinue
  if ($tool -eq 'python.exe' -and $cmd -and $cmd.Source -match '[\\/]WindowsApps[\\/]python') {
    Write-Warning ('OPTIONAL_UNVERIFIED python.exe is a WindowsApps execution alias, not proof of a working Python interpreter. Install Python.Python.3.13 to generate local patch recipes.')
  }
  elseif ($cmd) { Write-Output ('OPTIONAL_PASS ' + $tool + ': ' + $cmd.Source) }
  else { Write-Warning ('OPTIONAL_MISSING ' + $tool + ' (needed for additional research/bootstrapping, not Rust core tests)') }
}
$clang = Get-Command clang.exe -ErrorAction SilentlyContinue
if ($clang) { Write-Output ('PASS clang.exe: ' + $clang.Source) }
else { Write-Warning 'clang.exe not found. Native Rust MSVC build may still work, but LLVM verification needs Clang.' }

# Native rustc must target MSVC and have the Windows SDK + C++ linker installed.
if ((Get-Command rustc.exe -ErrorAction SilentlyContinue) -and (Get-Command cargo.exe -ErrorAction SilentlyContinue)) {
  $rust = (& rustc.exe --version) -join ''
  Write-Output $rust
  if ($rust -notmatch 'rustc ([0-9]+)\.([0-9]+)') { throw 'Cannot determine Rust version' }
  if ([version]::new([int]$Matches[1],[int]$Matches[2]) -lt [version]::new(1,85)) {
    throw 'Rust 1.85+ required (edition 2024)'
  }
  & rustup.exe show active-toolchain 2>$null | Out-Host
}
if (Get-Command wsl.exe -ErrorAction SilentlyContinue) {
  Write-Output 'WSL distributions (check Ubuntu-26.04 uses VERSION=2):'
  & wsl.exe --list --verbose | Out-Host
}
if ($RunTests) {
  if ($missing.Count -gt 0) { throw 'Cannot test: toolchain prerequisites missing' }
  $here = $PSScriptRoot
  Write-Output 'Running locked Cargo Rust unit tests'
  & cargo.exe test --locked --manifest-path (Join-Path $here 'Cargo.toml')
  if ($LASTEXITCODE -ne 0) { throw 'Rust unit tests failed' }
  Write-Output 'Running localization tests'
  & node.exe (Join-Path $here 'test_i18n.cjs')
  if ($LASTEXITCODE -ne 0) { throw 'Localization tests failed' }
}
if ($missing.Count -gt 0) {
  Write-Warning ('Missing '+($missing -join ', ')+'. Run install-windows.ps1 -Install, or run the prebuilt application from Releases without the toolchain.')
  exit 3
}
Write-Output 'NIKON_WINDOWS_TOOLCHAIN_CHECK_PASS - no firmware flashing was performed'
