<#
.SYNOPSIS
    PaperPilot Flash Installer for Windows.

.DESCRIPTION
    Detects Windows architecture (x64 / arm64), resolves the target release
    version (or latest), downloads the Windows release zip from GitHub Releases,
    verifies the SHA-256 checksum if present, extracts paperpilot.exe to
    $HOME\.paperpilot\bin, adds the directory to the user PATH if missing, and
    validates execution with --version.

.EXAMPLE
    irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex

.EXAMPLE
    $env:PAPERPILOT_VERSION="0.7.0"; irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex

.EXAMPLE
    .\scripts\install.ps1 -Version 0.7.0
    .\scripts\install.ps1 v0.7.0
#>
param(
    [Parameter(Position = 0)]
    [string]$Version = "",

    [switch]$DryRun,

    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help -or $Version -eq "-h" -or $Version -eq "--help") {
    Write-Host @'
PaperPilot Flash Installer (Windows PowerShell)

Usage:
  irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex
  .\scripts\install.ps1 [-Version <VERSION>] [-DryRun] [-Help]

Pin a specific version:
  $env:PAPERPILOT_VERSION="0.7.0"; irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex
  .\scripts\install.ps1 -Version 0.7.0
  .\scripts\install.ps1 v0.7.0

Environment Variables:
  PAPERPILOT_VERSION / VERSION  Version or tag to install (overridden by -Version)
  PAPERPILOT_INSTALL_DIR        Override install directory (default: $HOME\.paperpilot\bin)
  DRY_RUN=1                     Simulate installation without downloading or modifying files
'@
    exit 0
}

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
} catch {
    # Managed automatically on newer .NET runtimes
}

$Repo = "KrushnaVardhanReddy/PaperPilot"
$GithubReleasesApi = "https://api.github.com/repos/$Repo/releases"
$GithubBase = if ($env:GITHUB_BASE) { $env:GITHUB_BASE } else { "https://github.com/$Repo" }
$GithubDl = if ($env:GITHUB_DL) { $env:GITHUB_DL } else { "$GithubBase/releases/download" }
$FallbackVersion = "v0.7.0"

function Resolve-WindowsArch {
    $OsArch = $null
    try {
        $OsArch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()
    } catch {
        $OsArch = $null
    }

    if ($OsArch) {
        switch ($OsArch) {
            "x64"   { return "x86_64" }
            "arm64" { return "aarch64" }
            default {
                Write-Error "Unsupported Windows architecture: $OsArch. Only 64-bit Windows (x64, arm64) is supported."
                exit 1
            }
        }
    }

    $EnvArch = $env:PROCESSOR_ARCHITEW6432
    if ([string]::IsNullOrWhiteSpace($EnvArch)) {
        $EnvArch = $env:PROCESSOR_ARCHITECTURE
    }
    if (-not [string]::IsNullOrWhiteSpace($EnvArch)) {
        switch ($EnvArch.ToUpperInvariant()) {
            "AMD64" { return "x86_64" }
            "ARM64" { return "aarch64" }
        }
    }
    if ([System.Environment]::Is64BitOperatingSystem) {
        return "x86_64"
    }
    Write-Error "Unsupported architecture ($EnvArch). Only 64-bit Windows is supported."
    exit 1
}

function Normalize-ReleaseTag {
    param([string]$Raw)
    if ([string]::IsNullOrWhiteSpace($Raw)) {
        return ""
    }
    $Trimmed = $Raw.Trim()
    if ($Trimmed.ToLowerInvariant() -eq "latest") {
        return ""
    }
    $Clean = $Trimmed.TrimStart('v', 'V')
    if ($Clean -notmatch '^[A-Za-z0-9._+-]+$') {
        throw "Invalid version format '$Raw'."
    }
    return "v$Clean"
}

Write-Host "Detecting Windows architecture..."
$Arch = Resolve-WindowsArch
$RustTarget = "$Arch-pc-windows-msvc"
Write-Host "Detected architecture: $Arch ($RustTarget)"

$RawVersion = $Version
if ([string]::IsNullOrWhiteSpace($RawVersion)) {
    $RawVersion = $env:PAPERPILOT_VERSION
}
if ([string]::IsNullOrWhiteSpace($RawVersion)) {
    $RawVersion = $env:VERSION
}

$ResolvedVersion = Normalize-ReleaseTag -Raw $RawVersion

if ([string]::IsNullOrWhiteSpace($ResolvedVersion)) {
    Write-Host "Fetching latest release version..."
    try {
        $Headers = @{ "User-Agent" = "PaperPilot-Installer" }
        $Releases = Invoke-RestMethod -Uri $GithubReleasesApi -Headers $Headers -TimeoutSec 10 -ErrorAction Stop
        if ($Releases -and $Releases.Count -gt 0 -and $Releases[0].tag_name) {
            $ResolvedVersion = $Releases[0].tag_name
        }
    } catch {
        Write-Host "Warning: Rate limited or unable to reach GitHub API. Defaulting to stable tag $FallbackVersion"
    }

    if ([string]::IsNullOrWhiteSpace($ResolvedVersion)) {
        $ResolvedVersion = $FallbackVersion
    }
}

Write-Host "Installing PaperPilot version: $ResolvedVersion"

$Candidates = @()
if ($Arch -eq "aarch64") {
    $Candidates += @(
        "paperpilot-cli-windows-latest-aarch64-pc-windows-msvc.zip",
        "paperpilot-windows-latest-aarch64-pc-windows-msvc.zip",
        "paperpilot-aarch64-pc-windows-msvc.zip",
        "paperpilot-all-windows-latest-aarch64-pc-windows-msvc.zip"
    )
}
# x86_64 candidates (primary for x64, and emulation fallback for Windows ARM64)
$Candidates += @(
    "paperpilot-cli-windows-latest-x86_64-pc-windows-msvc.zip",
    "paperpilot-windows-latest-x86_64-pc-windows-msvc.zip",
    "paperpilot-x86_64-pc-windows-msvc.zip",
    "paperpilot-all-windows-latest-x86_64-pc-windows-msvc.zip"
)

$UserHome = if (-not [string]::IsNullOrWhiteSpace($HOME)) { $HOME } else { $env:USERPROFILE }
$InstallDir = if (-not [string]::IsNullOrWhiteSpace($env:PAPERPILOT_INSTALL_DIR)) {
    $env:PAPERPILOT_INSTALL_DIR
} else {
    Join-Path $UserHome ".paperpilot\bin"
}

$PrimaryUrl = "$GithubDl/$ResolvedVersion/$($Candidates[0])"
$PrimaryChecksumUrl = "$PrimaryUrl.sha256"

$IsDryRun = $DryRun.IsPresent -or ($env:DRY_RUN -and $env:DRY_RUN -ne "0")
if ($IsDryRun) {
    Write-Host "DRY_RUN mode enabled."
    Write-Host "  Target:       $RustTarget"
    Write-Host "  Version:      $ResolvedVersion"
    Write-Host "  Primary URL:  $PrimaryUrl"
    Write-Host "  Checksum URL: $PrimaryChecksumUrl"
    Write-Host "  Install path: $(Join-Path $InstallDir 'paperpilot.exe')"
    Write-Host "  Verification: $(Join-Path $InstallDir 'paperpilot.exe') --version"
    exit 0
}

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) "paperpilot_install_$([System.Guid]::NewGuid().ToString('N'))"
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

$PreviousProgressPreference = $ProgressPreference
$ProgressPreference = "SilentlyContinue"

try {
    $SelectedAsset = $null
    $SelectedUrl = $null
    $ZipPath = $null

    foreach ($Candidate in $Candidates) {
        $CandidateUrl = "$GithubDl/$ResolvedVersion/$Candidate"
        $CandidatePath = Join-Path $TempDir $Candidate
        Write-Host "Downloading $Candidate..."
        try {
            Invoke-WebRequest -Uri $CandidateUrl -OutFile $CandidatePath -UseBasicParsing -ErrorAction Stop
            $SelectedAsset = $Candidate
            $SelectedUrl = $CandidateUrl
            $ZipPath = $CandidatePath
            break
        } catch {
            if (Test-Path $CandidatePath) {
                Remove-Item -Force $CandidatePath -ErrorAction SilentlyContinue
            }
        }
    }

    if (-not $ZipPath) {
        Write-Error "Failed to download a precompiled Windows release archive from $GithubDl/$ResolvedVersion/"
        exit 1
    }

    $ChecksumUrl = "$SelectedUrl.sha256"
    $ChecksumPath = Join-Path $TempDir "$SelectedAsset.sha256"
    Write-Host "Checking SHA-256 checksum..."
    try {
        Invoke-WebRequest -Uri $ChecksumUrl -OutFile $ChecksumPath -UseBasicParsing -ErrorAction Stop
        if (Test-Path $ChecksumPath) {
            $ExpectedLine = Get-Content -Path $ChecksumPath -ErrorAction Stop | Select-Object -First 1
            if (-not [string]::IsNullOrWhiteSpace($ExpectedLine)) {
                $ExpectedSum = $ExpectedLine.Trim().Split()[0].ToLowerInvariant()
                $ActualSum = (Get-FileHash -Path $ZipPath -Algorithm SHA256).Hash.ToLowerInvariant()
                if ($ExpectedSum -ne $ActualSum) {
                    Write-Error "Checksum verification failed! Expected: $ExpectedSum, Actual: $ActualSum"
                    exit 1
                }
                Write-Host "Checksum verified."
            }
        }
    } catch {
        Write-Host "Warning: Checksum file not found at $ChecksumUrl; skipping checksum verification."
    }

    $ExtractDir = Join-Path $TempDir "extract"
    New-Item -ItemType Directory -Force -Path $ExtractDir | Out-Null
    Write-Host "Extracting $SelectedAsset..."
    Expand-Archive -Path $ZipPath -DestinationPath $ExtractDir -Force

    $FoundExe = Get-ChildItem -Path $ExtractDir -Recurse -File |
        Where-Object { $_.Name -eq "paperpilot.exe" -or $_.Name -eq "paperpilot-cli.exe" } |
        Select-Object -First 1

    if (-not $FoundExe) {
        Write-Error "Could not find paperpilot.exe inside downloaded archive ($SelectedAsset)."
        exit 1
    }

    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    $DestExe = Join-Path $InstallDir "paperpilot.exe"
    Copy-Item -Path $FoundExe.FullName -Destination $DestExe -Force

    # Ensure install directory is in User PATH
    $NormalizedInstallDir = $InstallDir.TrimEnd('\')
    $CurrentUserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
    $UserEntries = @()
    if (-not [string]::IsNullOrWhiteSpace($CurrentUserPath)) {
        $UserEntries = $CurrentUserPath -split ';' | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    }
    $InUserPath = $UserEntries | Where-Object { $_.TrimEnd('\').Equals($NormalizedInstallDir, [System.StringComparison]::OrdinalIgnoreCase) }
    if (-not $InUserPath) {
        Write-Host "Adding $InstallDir to User PATH..."
        $NewUserPath = ($UserEntries + $InstallDir) -join ';'
        [System.Environment]::SetEnvironmentVariable("Path", $NewUserPath, "User")
    }

    $SessionEntries = ($env:Path -split ';') | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    $InSessionPath = $SessionEntries | Where-Object { $_.TrimEnd('\').Equals($NormalizedInstallDir, [System.StringComparison]::OrdinalIgnoreCase) }
    if (-not $InSessionPath) {
        $env:Path = ($SessionEntries + $InstallDir) -join ';'
    }

    Write-Host "Validating installation..."
    $VersionOutput = & $DestExe --version
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Verification failed: '$DestExe --version' exited with code $LASTEXITCODE"
        exit 1
    }
    Write-Host "Verified: $VersionOutput"
    Write-Host "PaperPilot $ResolvedVersion has been successfully installed to $DestExe"
    Write-Host "You can run 'paperpilot --help' (restart your terminal if 'paperpilot' is not yet recognized in older shells)."
} finally {
    $ProgressPreference = $PreviousProgressPreference
    if (Test-Path $TempDir) {
        Remove-Item -Recurse -Force $TempDir -ErrorAction SilentlyContinue
    }
}
