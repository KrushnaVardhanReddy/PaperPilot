<#
.SYNOPSIS
    PaperPilot Flash Installer for Windows.

.DESCRIPTION
    Detects Windows architecture (x64 / arm64), supports component selection
    (Desktop GUI, CLI, or All-in-One, defaulting to Desktop), resolves the
    target release version (or latest), downloads the release installer/archive
    from GitHub Releases, verifies the SHA-256 checksum if present, and installs
    the selected PaperPilot components.

.EXAMPLE
    irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex

.EXAMPLE
    .\scripts\install.ps1 -Component Desktop
    .\scripts\install.ps1 -Component CLI -Version 0.7.0
    .\scripts\install.ps1 -Component All -DryRun

.EXAMPLE
    $env:PAPERPILOT_VERSION="0.7.0"; $env:PAPERPILOT_COMPONENT="CLI"; irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex
#>
param(
    [Parameter(Position = 0)]
    [string]$Version = "",

    [ValidateSet("", "Desktop", "CLI", "All")]
    [string]$Component = "",

    [switch]$Desktop,

    [switch]$Cli,

    [switch]$All,

    [switch]$NonInteractive,

    [switch]$DryRun,

    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help -or $Version -eq "-h" -or $Version -eq "--help") {
    Write-Host @'
PaperPilot Flash Installer (Windows PowerShell)

Usage:
  irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex
  .\scripts\install.ps1 [-Component <Desktop|CLI|All>] [-Version <VERSION>] [-DryRun] [-Help]

Component Options:
  -Component Desktop    Install PaperPilot Desktop GUI App (.exe / .msi) [Default]
  -Component CLI        Install standalone paperpilot.exe CLI binary only
  -Component All        Install Desktop App + CLI + MCP Server
  -Desktop / -Cli / -All  Switch aliases for -Component

Pin a specific version:
  $env:PAPERPILOT_VERSION="0.7.0"; irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex
  .\scripts\install.ps1 -Component All -Version 0.7.0
  .\scripts\install.ps1 v0.7.0

Environment Variables:
  PAPERPILOT_COMPONENT / COMPONENT  Component to install: Desktop, CLI, or All
  PAPERPILOT_VERSION / VERSION      Version or tag to install (overridden by -Version)
  PAPERPILOT_INSTALL_DIR            Override CLI install directory (default: $HOME\.paperpilot\bin)
  PAPERPILOT_NONINTERACTIVE=1       Skip interactive component prompt
  DRY_RUN=1                         Simulate installation without downloading or modifying files
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

function Normalize-ComponentName {
    param([string]$Raw)
    switch ($Raw.Trim().ToLowerInvariant()) {
        "desktop" { return "Desktop" }
        "gui"     { return "Desktop" }
        "cli"     { return "CLI" }
        "all"     { return "All" }
        "suite"   { return "All" }
        default   { throw "Invalid component '$Raw'. Expected Desktop, CLI, or All." }
    }
}

function Test-InteractiveTerminal {
    if ($NonInteractive.IsPresent) {
        return $false
    }
    if ($env:PAPERPILOT_NONINTERACTIVE -and $env:PAPERPILOT_NONINTERACTIVE -ne "0") {
        return $false
    }
    if ($env:NONINTERACTIVE -and $env:NONINTERACTIVE -ne "0") {
        return $false
    }
    if ($env:CI) {
        return $false
    }
    if (-not [Environment]::UserInteractive) {
        return $false
    }
    try {
        if ([Environment]::GetCommandLineArgs() -match '^-NonI') {
            return $false
        }
        if ([Console]::IsInputRedirected -or [Console]::IsOutputRedirected) {
            return $false
        }
        return $null -ne $Host.UI.RawUI
    } catch {
        return $false
    }
}

function Resolve-SelectedComponent {
    $Chosen = ""
    if ($Desktop.IsPresent) {
        $Chosen = "Desktop"
    }
    if ($Cli.IsPresent) {
        if ($Chosen -and $Chosen -ne "CLI") {
            throw "Conflicting component switches specified ('$Chosen' and 'CLI')."
        }
        $Chosen = "CLI"
    }
    if ($All.IsPresent) {
        if ($Chosen -and $Chosen -ne "All") {
            throw "Conflicting component switches specified ('$Chosen' and 'All')."
        }
        $Chosen = "All"
    }
    if (-not [string]::IsNullOrWhiteSpace($Component)) {
        $Norm = Normalize-ComponentName -Raw $Component
        if ($Chosen -and $Chosen -ne $Norm) {
            throw "Conflicting component parameters specified ('$Chosen' and '$Norm')."
        }
        $Chosen = $Norm
    }
    if ([string]::IsNullOrWhiteSpace($Chosen)) {
        $EnvComp = if (-not [string]::IsNullOrWhiteSpace($env:PAPERPILOT_COMPONENT)) {
            $env:PAPERPILOT_COMPONENT
        } else {
            $env:COMPONENT
        }
        if (-not [string]::IsNullOrWhiteSpace($EnvComp)) {
            $Chosen = Normalize-ComponentName -Raw $EnvComp
        }
    }

    if (-not [string]::IsNullOrWhiteSpace($Chosen)) {
        return $Chosen
    }

    if (Test-InteractiveTerminal) {
        Write-Host "Select PaperPilot components to install:"
        Write-Host "  1) Desktop App (Visual GUI workspace + PDF Viewer) [Default]"
        Write-Host "  2) CLI only (Headless terminal & scripting engine)"
        Write-Host "  3) All-in-One (CLI + Desktop App + MCP Server)"
        while ($true) {
            $Choice = Read-Host "Choice [1/2/3] (default: 1)"
            if ($null -eq $Choice) {
                $Choice = "1"
            }
            $Choice = $Choice.Trim()
            switch ($Choice) {
                ""  { return "Desktop" }
                "1" { return "Desktop" }
                "2" { return "CLI" }
                "3" { return "All" }
                default {
                    Write-Host "Invalid choice '$Choice'. Please enter 1, 2, or 3." -ForegroundColor Yellow
                }
            }
        }
    }

    Write-Host "Non-interactive environment detected; defaulting to Desktop App."
    return "Desktop"
}

function Invoke-DownloadVerifiedAsset {
    param(
        [string[]]$Candidates,
        [string]$BaseUrl,
        [string]$TempDir,
        [bool]$Optional = $false
    )

    $SelectedAsset = $null
    $SelectedUrl = $null
    $SelectedPath = $null

    foreach ($Candidate in $Candidates) {
        $CandidateUrl = "$BaseUrl/$Candidate"
        $CandidatePath = Join-Path $TempDir $Candidate
        Write-Host "Downloading $Candidate..."
        try {
            Invoke-WebRequest -Uri $CandidateUrl -OutFile $CandidatePath -UseBasicParsing -ErrorAction Stop
            $SelectedAsset = $Candidate
            $SelectedUrl = $CandidateUrl
            $SelectedPath = $CandidatePath
            break
        } catch {
            if (Test-Path $CandidatePath) {
                Remove-Item -Force $CandidatePath -ErrorAction SilentlyContinue
            }
        }
    }

    if (-not $SelectedPath) {
        if ($Optional) {
            return $null
        }
        throw "Failed to download a precompiled Windows release asset from $BaseUrl/"
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
                $ActualSum = (Get-FileHash -Path $SelectedPath -Algorithm SHA256).Hash.ToLowerInvariant()
                if ($ExpectedSum -ne $ActualSum) {
                    throw "Checksum verification failed! Expected: $ExpectedSum, Actual: $ActualSum"
                }
                Write-Host "Checksum verified."
            }
        }
    } catch {
        if ($_.Exception.Message -like "Checksum verification failed!*") {
            throw
        }
        Write-Host "Warning: Checksum file not found at $ChecksumUrl; skipping checksum verification."
    }

    return [PSCustomObject]@{
        AssetName = $SelectedAsset
        Url       = $SelectedUrl
        FilePath  = $SelectedPath
    }
}

function Add-ToUserPath {
    param([string]$Directory)

    $NormalizedDir = $Directory.TrimEnd('\')
    $CurrentUserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
    $UserEntries = @()
    if (-not [string]::IsNullOrWhiteSpace($CurrentUserPath)) {
        $UserEntries = $CurrentUserPath -split ';' | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    }
    $InUserPath = $UserEntries | Where-Object { $_.TrimEnd('\').Equals($NormalizedDir, [System.StringComparison]::OrdinalIgnoreCase) }
    if (-not $InUserPath) {
        Write-Host "Adding $Directory to User PATH..."
        $NewUserPath = ($UserEntries + $Directory) -join ';'
        [System.Environment]::SetEnvironmentVariable("Path", $NewUserPath, "User")
    }

    $SessionEntries = ($env:Path -split ';') | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    $InSessionPath = $SessionEntries | Where-Object { $_.TrimEnd('\').Equals($NormalizedDir, [System.StringComparison]::OrdinalIgnoreCase) }
    if (-not $InSessionPath) {
        $env:Path = ($SessionEntries + $Directory) -join ';'
    }
}

function Install-DesktopApp {
    param(
        [string[]]$Candidates,
        [string]$BaseUrl,
        [string]$TempDir
    )

    $Asset = Invoke-DownloadVerifiedAsset -Candidates $Candidates -BaseUrl $BaseUrl -TempDir $TempDir
    if ($Asset.AssetName.EndsWith(".msi", [System.StringComparison]::OrdinalIgnoreCase)) {
        Write-Host "Running Windows Installer ($($Asset.AssetName))..."
        $MsiArgs = @("/i", "`"$($Asset.FilePath)`"", "/passive")
        $Proc = Start-Process -FilePath "msiexec.exe" -ArgumentList $MsiArgs -Wait -PassThru
        if ($Proc.ExitCode -ne 0 -and $Proc.ExitCode -ne 3010) {
            throw "MSI installer failed with exit code $($Proc.ExitCode)."
        }
    } else {
        Write-Host "Running PaperPilot Desktop Setup ($($Asset.AssetName))..."
        $Proc = Start-Process -FilePath $Asset.FilePath -ArgumentList "/S" -Wait -PassThru
        if ($Proc.ExitCode -ne 0) {
            throw "Desktop installer failed with exit code $($Proc.ExitCode)."
        }
    }

    Write-Host "PaperPilot Desktop App installation completed."
}

function Install-ArchiveBinary {
    param(
        [string[]]$Candidates,
        [string[]]$BinaryNames,
        [string]$DestName,
        [string]$BaseUrl,
        [string]$TempDir,
        [string]$InstallDir,
        [bool]$Optional = $false
    )

    $FoundExe = $null
    $CachedExtract = Join-Path $TempDir "extract_paperpilot.exe"
    if (Test-Path $CachedExtract) {
        $FoundExe = Get-ChildItem -Path $CachedExtract -Recurse -File |
            Where-Object { $BinaryNames -contains $_.Name } |
            Select-Object -First 1
    }

    if (-not $FoundExe) {
        $Asset = Invoke-DownloadVerifiedAsset -Candidates $Candidates -BaseUrl $BaseUrl -TempDir $TempDir -Optional $Optional
        if (-not $Asset) {
            Write-Host "Warning: Could not download $DestName archive from $BaseUrl/; skipping."
            return $false
        }

        $ExtractDir = Join-Path $TempDir "extract_$DestName"
        New-Item -ItemType Directory -Force -Path $ExtractDir | Out-Null
        Write-Host "Extracting $($Asset.AssetName)..."
        Expand-Archive -Path $Asset.FilePath -DestinationPath $ExtractDir -Force

        $FoundExe = Get-ChildItem -Path $ExtractDir -Recurse -File |
            Where-Object { $BinaryNames -contains $_.Name } |
            Select-Object -First 1

        if (-not $FoundExe) {
            if ($Optional) {
                Write-Host "Warning: Could not find $DestName inside $($Asset.AssetName); skipping."
                return $false
            }
            throw "Could not find $DestName inside downloaded archive ($($Asset.AssetName))."
        }
    }

    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    $DestExe = Join-Path $InstallDir $DestName
    Copy-Item -Path $FoundExe.FullName -Destination $DestExe -Force
    Add-ToUserPath -Directory $InstallDir

    Write-Host "Validating $DestName installation..."
    $VersionOutput = & $DestExe --version
    if ($LASTEXITCODE -ne 0) {
        throw "Verification failed: '$DestExe --version' exited with code $LASTEXITCODE"
    }
    Write-Host "Verified: $VersionOutput"
    return $true
}

Write-Host "Detecting Windows architecture..."
$Arch = Resolve-WindowsArch
$RustTarget = "$Arch-pc-windows-msvc"
Write-Host "Detected architecture: $Arch ($RustTarget)"

$SelectedComponent = Resolve-SelectedComponent
Write-Host "Selected component: $SelectedComponent"

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

$CleanVersion = $ResolvedVersion.TrimStart('v', 'V')
Write-Host "Installing PaperPilot version: $ResolvedVersion"

$DesktopCandidates = @()
if ($Arch -eq "aarch64") {
    $DesktopCandidates += @(
        "PaperPilot_${CleanVersion}_arm64-setup.exe",
        "PaperPilot_${CleanVersion}_arm64_en-US.msi",
        "PaperPilot_${CleanVersion}_arm64.msi"
    )
}
$DesktopCandidates += @(
    "PaperPilot_${CleanVersion}_x64-setup.exe",
    "PaperPilot_${CleanVersion}_x64_en-US.msi",
    "PaperPilot_${CleanVersion}_x64.msi"
)

$CliCandidates = @()
if ($Arch -eq "aarch64") {
    $CliCandidates += @(
        "paperpilot-cli-windows-latest-aarch64-pc-windows-msvc.zip",
        "paperpilot-windows-latest-aarch64-pc-windows-msvc.zip",
        "paperpilot-aarch64-pc-windows-msvc.zip",
        "paperpilot-all-windows-latest-aarch64-pc-windows-msvc.zip"
    )
}
$CliCandidates += @(
    "paperpilot-cli-windows-latest-x86_64-pc-windows-msvc.zip",
    "paperpilot-windows-latest-x86_64-pc-windows-msvc.zip",
    "paperpilot-x86_64-pc-windows-msvc.zip",
    "paperpilot-all-windows-latest-x86_64-pc-windows-msvc.zip"
)

$McpCandidates = @()
if ($Arch -eq "aarch64") {
    $McpCandidates += @(
        "paperpilot-mcp-windows-latest-aarch64-pc-windows-msvc.zip",
        "paperpilot-all-windows-latest-aarch64-pc-windows-msvc.zip"
    )
}
$McpCandidates += @(
    "paperpilot-mcp-windows-latest-x86_64-pc-windows-msvc.zip",
    "paperpilot-all-windows-latest-x86_64-pc-windows-msvc.zip"
)

$UserHome = if (-not [string]::IsNullOrWhiteSpace($HOME)) { $HOME } else { $env:USERPROFILE }
$InstallDir = if (-not [string]::IsNullOrWhiteSpace($env:PAPERPILOT_INSTALL_DIR)) {
    $env:PAPERPILOT_INSTALL_DIR
} else {
    Join-Path $UserHome ".paperpilot\bin"
}

$BaseDownloadUrl = "$GithubDl/$ResolvedVersion"
$PrimaryDesktopUrl = "$BaseDownloadUrl/$($DesktopCandidates[0])"
$PrimaryCliUrl = "$BaseDownloadUrl/$($CliCandidates[0])"
$PrimaryMcpUrl = "$BaseDownloadUrl/$($McpCandidates[0])"

$IsDryRun = $DryRun.IsPresent -or ($env:DRY_RUN -and $env:DRY_RUN -ne "0")
if ($IsDryRun) {
    Write-Host "DRY_RUN mode enabled."
    Write-Host "  Component:    $SelectedComponent"
    Write-Host "  Target:       $RustTarget"
    Write-Host "  Version:      $ResolvedVersion"
    if ($SelectedComponent -eq "Desktop" -or $SelectedComponent -eq "All") {
        Write-Host "  Desktop URL:      $PrimaryDesktopUrl"
        Write-Host "  Desktop Checksum: $PrimaryDesktopUrl.sha256"
        Write-Host "  Desktop action:   Execute Windows setup installer silently"
    }
    if ($SelectedComponent -eq "CLI" -or $SelectedComponent -eq "All") {
        Write-Host "  CLI URL:          $PrimaryCliUrl"
        Write-Host "  CLI Checksum:     $PrimaryCliUrl.sha256"
        Write-Host "  CLI path:         $(Join-Path $InstallDir 'paperpilot.exe')"
        Write-Host "  CLI verification: $(Join-Path $InstallDir 'paperpilot.exe') --version"
    }
    if ($SelectedComponent -eq "All") {
        Write-Host "  MCP URL:          $PrimaryMcpUrl"
        Write-Host "  MCP Checksum:     $PrimaryMcpUrl.sha256"
        Write-Host "  MCP path:         $(Join-Path $InstallDir 'paperpilot-mcp.exe')"
        Write-Host "  MCP verification: $(Join-Path $InstallDir 'paperpilot-mcp.exe') --version"
    }
    exit 0
}

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) "paperpilot_install_$([System.Guid]::NewGuid().ToString('N'))"
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

$PreviousProgressPreference = $ProgressPreference
$ProgressPreference = "SilentlyContinue"

try {
    $CliInstalled = $false
    $McpInstalled = $false

    if ($SelectedComponent -eq "Desktop" -or $SelectedComponent -eq "All") {
        Install-DesktopApp -Candidates $DesktopCandidates -BaseUrl $BaseDownloadUrl -TempDir $TempDir
    }

    if ($SelectedComponent -eq "CLI" -or $SelectedComponent -eq "All") {
        $CliInstalled = Install-ArchiveBinary `
            -Candidates $CliCandidates `
            -BinaryNames @("paperpilot.exe", "paperpilot-cli.exe") `
            -DestName "paperpilot.exe" `
            -BaseUrl $BaseDownloadUrl `
            -TempDir $TempDir `
            -InstallDir $InstallDir `
            -Optional $false
    }

    if ($SelectedComponent -eq "All") {
        $McpInstalled = Install-ArchiveBinary `
            -Candidates $McpCandidates `
            -BinaryNames @("paperpilot-mcp.exe") `
            -DestName "paperpilot-mcp.exe" `
            -BaseUrl $BaseDownloadUrl `
            -TempDir $TempDir `
            -InstallDir $InstallDir `
            -Optional $true
    }

    Write-Host ""
    Write-Host "PaperPilot $ResolvedVersion ($SelectedComponent) installation complete!"
    if ($SelectedComponent -eq "Desktop" -or $SelectedComponent -eq "All") {
        Write-Host "  Desktop App: Launch PaperPilot from the Windows Start Menu."
    }
    if ($CliInstalled) {
        Write-Host "  CLI Binary:  $(Join-Path $InstallDir 'paperpilot.exe')"
        Write-Host "  Run 'paperpilot --help' (restart your terminal if 'paperpilot' is not yet recognized in older shells)."
    }
    if ($McpInstalled) {
        Write-Host "  MCP Server:  $(Join-Path $InstallDir 'paperpilot-mcp.exe')"
    }
} finally {
    $ProgressPreference = $PreviousProgressPreference
    if (Test-Path $TempDir) {
        Remove-Item -Recurse -Force $TempDir -ErrorAction SilentlyContinue
    }
}
