$ErrorActionPreference = "Stop"

$Repo = "KrushnaVardhanReddy/PaperPilot"
$GithubReleasesApi = "https://api.github.com/repos/$Repo/releases"
$GithubDl = "https://github.com/$Repo/releases/download"

Write-Host "Detecting architecture..."
if ([System.Environment]::Is64BitOperatingSystem) {
    $Target = "windows-latest-x86_64-pc-windows-msvc"
} else {
    Write-Error "Unsupported architecture. Only 64-bit Windows is supported."
    exit 1
}
Write-Host "Target architecture identified as: $Target"

Write-Host "Fetching latest release version..."
try {
    $Releases = Invoke-RestMethod -Uri $GithubReleasesApi -ErrorAction Stop
    $Version = $Releases[0].tag_name
} catch {
    Write-Host "Warning: Rate limited or unable to fetch from GitHub API. Defaulting to a stable tag (v0.6.1)"
    $Version = "v0.6.1"
}
Write-Host "Latest release: $Version"

$Filename = "paperpilot-cli-$Target.zip"
$DownloadUrl = "$GithubDl/$Version/$Filename"
$ChecksumUrl = "$DownloadUrl.sha256"

$InstallDir = "$env:LOCALAPPDATA\Programs\PaperPilot\bin"

if ($env:DRY_RUN) {
    Write-Host "DRY_RUN mode enabled. Would have downloaded from $DownloadUrl"
    Write-Host "Would have installed to $InstallDir"
    exit 0
}

$TempDir = Join-Path $env:TEMP "paperpilot_install_$(Get-Random)"
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null
$ZipPath = Join-Path $TempDir $Filename

Write-Host "Downloading $Filename..."
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -ErrorAction Stop
} catch {
    # Fallback to all-in-one suite or legacy name
    $AltFilename = "paperpilot-all-$Target.zip"
    $AltUrl = "$GithubDl/$Version/$AltFilename"
    Write-Host "Trying alternative bundle $AltFilename..."
    try {
        Invoke-WebRequest -Uri $AltUrl -OutFile $ZipPath -ErrorAction Stop
        $ChecksumUrl = "$AltUrl.sha256"
    } catch {
        Write-Error "Failed to download $DownloadUrl"
        Remove-Item -Recurse -Force $TempDir
        exit 1
    }
}

Write-Host "Downloading checksum file..."
$ChecksumFile = Join-Path $TempDir "$Filename.sha256"
try {
    Invoke-WebRequest -Uri $ChecksumUrl -OutFile $ChecksumFile -ErrorAction SilentlyContinue
    if (Test-Path $ChecksumFile) {
        Write-Host "Verifying checksum..."
        $ExpectedSum = (Get-Content $ChecksumFile | Select-Object -First 1).Split()[0].Trim()
        $ActualSum = (Get-FileHash -Path $ZipPath -Algorithm SHA256).Hash.ToLower()
        if ($ExpectedSum.ToLower() -ne $ActualSum) {
            Write-Error "Checksum verification failed! Expected: $ExpectedSum, Got: $ActualSum"
            Remove-Item -Recurse -Force $TempDir
            exit 1
        }
        Write-Host "Checksum OK."
    }
} catch {
    Write-Host "Warning: Skipping checksum verification."
}

Write-Host "Extracting $Filename..."
Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$ExePath = Join-Path $TempDir "paperpilot.exe"
if (-not (Test-Path $ExePath)) {
    $ExePath = Join-Path $TempDir "paperpilot-cli.exe"
}

if (-not (Test-Path $ExePath)) {
    Write-Error "Could not find paperpilot.exe in downloaded archive."
    Remove-Item -Recurse -Force $TempDir
    exit 1
}

Copy-Item -Path $ExePath -Destination (Join-Path $InstallDir "paperpilot.exe") -Force

# Clean up
Remove-Item -Recurse -Force $TempDir

# Check PATH
$UserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to user PATH..."
    [System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path += ";$InstallDir"
}

Write-Host "PaperPilot $Version has been successfully installed to $InstallDir\paperpilot.exe"
Write-Host "You can now run 'paperpilot' in any terminal window (restart terminal if not found)."
