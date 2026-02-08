param(
  [string]$Repo = "JoseMaurette1/jump"
)

$InstallDir = Join-Path $env:USERPROFILE ".local\bin"
$Platform = "windows-x86_64"

function Get-LatestVersion {
  try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    return $release.tag_name
  } catch {
    Write-Error "Could not determine latest version. Check your internet connection."
    return $null
  }
}

function Add-UserPath {
  param([string]$PathToAdd)
  $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
  if (-not $userPath) { $userPath = "" }
  if ($userPath -notlike "*$PathToAdd*") {
    if ([string]::IsNullOrWhiteSpace($userPath)) {
      [Environment]::SetEnvironmentVariable("Path", $PathToAdd, "User")
    } else {
      [Environment]::SetEnvironmentVariable("Path", "$PathToAdd;$userPath", "User")
    }
    Write-Host "Added jump to user PATH."
  } else {
    Write-Host "jump already in user PATH."
  }
}

Write-Host "Installing jump..."
Write-Host ""

$version = Get-LatestVersion
if (-not $version) { exit 1 }

$url = "https://github.com/$Repo/releases/download/$version/jump-$Platform.zip"
$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("jump-" + [System.Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Force -Path $tempDir | Out-Null

try {
  $zipPath = Join-Path $tempDir "jump.zip"
  Write-Host "Downloading jump $version for $Platform..."
  Invoke-WebRequest -Uri $url -OutFile $zipPath
  Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

  New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
  Move-Item -Force (Join-Path $tempDir "jump.exe") (Join-Path $InstallDir "jump.exe")

  if ($env:PATH -notlike "*$InstallDir*") {
    $env:PATH = "$InstallDir;$env:PATH"
  }

  Add-UserPath -PathToAdd $InstallDir
  Write-Host "Installed jump to $InstallDir\jump.exe"
} finally {
  Remove-Item -Recurse -Force $tempDir
}

Write-Host ""
Write-Host "Done! Restart your shell to use 'jump'."
