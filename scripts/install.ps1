$ErrorActionPreference = "Stop"

$Repo = "KrushnaVardhanReddy/PaperPilot"
$GithubApi = "https://api.github.com/repos/$Repo/releases/latest"
$GithubDl = "https://github.com/$Repo/releases/download"

Write-Host "Detecting architecture..."
if ([System.Environment]::Is64BitOperatingSystem) {
    $Target = "paperpilot-x86_64-pc-windows-msvc"
} else {
    Write-Error "Unsupported architecture. Only 64-bit Windows is supported."
    exit 1
}
Write-Host "Target architecture identified as: $Target"

Write-Host "Fetching latest release version..."
try {
    $Response = Invoke-RestMethod -Uri $GithubApi -ErrorAction Stop
    $Version = $Response.tag_name
} catch {
    Write-Host "Warning: Rate limited or unable to fetch from GitHub API. Defaulting to a stable tag (e.g. v1.0.0)"
    $Version = "v1.0.0"
}
Write-Host "Latest release: $Version"

$Filename = "$Target.zip"
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
    Write-Error "Failed to download $DownloadUrl"
    Remove-Item -Recurse -Force $TempDir
    exit 1
}

Write-Host "Extracting $Filename..."
try {
    Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force -ErrorAction Stop
} catch {
    Write-Error "Failed to extract $ZipPath"
    Remove-Item -Recurse -Force $TempDir
    exit 1
}

Write-Host "Installing to $InstallDir..."
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$ExeTempPath = Join-Path $TempDir "paperpilot.exe"
$ExeInstallPath = Join-Path $InstallDir "paperpilot.exe"
if (Test-Path $ExeTempPath) {
    Move-Item -Path $ExeTempPath -Destination $ExeInstallPath -Force
} else {
    # In case the zip contains a directory with the exe inside
    $FoundExe = Get-ChildItem -Path $TempDir -Recurse -Filter "paperpilot.exe" | Select-Object -First 1
    if ($FoundExe) {
        Move-Item -Path $FoundExe.FullName -Destination $ExeInstallPath -Force
    } else {
        Write-Error "Could not find paperpilot.exe in the downloaded archive."
        Remove-Item -Recurse -Force $TempDir
        exit 1
    }
}

Remove-Item -Recurse -Force $TempDir

Write-Host "Updating PATH environment variable..."
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -split ';' -notcontains $InstallDir) {
    $NewPath = "$UserPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "Added $InstallDir to User PATH."
} else {
    Write-Host "$InstallDir is already in User PATH."
}

Write-Host "PaperPilot $Version has been successfully installed to $InstallDir\paperpilot.exe"
Write-Host "Please restart your terminal for the PATH changes to take effect."
Write-Host "You can then run it by typing: paperpilot"
