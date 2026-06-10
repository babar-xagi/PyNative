param(
    [switch]$Install,
    [switch]$Launch
)

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$ProjectRoot = Join-Path $RepoRoot "android\experiment"
$PackageName = "com.pynative.experiment"
$ActivityName = "$PackageName/.MainActivity"
$BuildRoot = Join-Path $RepoRoot "build\android-experiment"
$GeneratedRoot = Join-Path $BuildRoot "generated"
$CompiledRes = Join-Path $BuildRoot "compiled-res"
$ClassesRoot = Join-Path $BuildRoot "classes"
$ClassesJar = Join-Path $BuildRoot "classes.jar"
$DexRoot = Join-Path $BuildRoot "dex"
$LocalAndroidJar = Join-Path $BuildRoot "android.jar"
$UnsignedApk = Join-Path $BuildRoot "pynative-android-unsigned.apk"
$AlignedApk = Join-Path $BuildRoot "pynative-android-aligned.apk"
$OutputApk = Join-Path $BuildRoot "pynative-android-debug.apk"
$Keystore = Join-Path $RepoRoot "build\android-debug.keystore"

function Resolve-AndroidSdk {
    if ($env:ANDROID_HOME -and (Test-Path -LiteralPath $env:ANDROID_HOME)) {
        return (Resolve-Path -LiteralPath $env:ANDROID_HOME).Path
    }

    if ($env:ANDROID_SDK_ROOT -and (Test-Path -LiteralPath $env:ANDROID_SDK_ROOT)) {
        return (Resolve-Path -LiteralPath $env:ANDROID_SDK_ROOT).Path
    }

    $defaultSdk = Join-Path $env:LOCALAPPDATA "Android\Sdk"
    if (Test-Path -LiteralPath $defaultSdk) {
        return (Resolve-Path -LiteralPath $defaultSdk).Path
    }

    throw "Android SDK not found. Set ANDROID_HOME or install Android Studio SDK."
}

function Resolve-LatestDirectory($Path) {
    $directory = Get-ChildItem -LiteralPath $Path -Directory | Sort-Object Name -Descending | Select-Object -First 1
    if (-not $directory) {
        throw "No directory found in $Path"
    }
    return $directory.FullName
}

function Resolve-JavaHome {
    if ($env:JAVA_HOME -and (Test-Path -LiteralPath (Join-Path $env:JAVA_HOME "bin\javac.exe"))) {
        return (Resolve-Path -LiteralPath $env:JAVA_HOME).Path
    }

    $androidStudioJbr = "C:\Program Files\Android\Android Studio\jbr"
    if (Test-Path -LiteralPath (Join-Path $androidStudioJbr "bin\javac.exe")) {
        return $androidStudioJbr
    }

    throw "Java JDK not found. Install Android Studio or set JAVA_HOME."
}

$AndroidSdk = Resolve-AndroidSdk
$BuildTools = Resolve-LatestDirectory (Join-Path $AndroidSdk "build-tools")
$Platform = Resolve-LatestDirectory (Join-Path $AndroidSdk "platforms")
$JavaHome = Resolve-JavaHome
$env:JAVA_HOME = $JavaHome
$env:Path = (Join-Path $JavaHome "bin") + [System.IO.Path]::PathSeparator + $env:Path

$Aapt2 = Join-Path $BuildTools "aapt2.exe"
$D8 = Join-Path $BuildTools "d8.bat"
$Zipalign = Join-Path $BuildTools "zipalign.exe"
$Apksigner = Join-Path $BuildTools "apksigner.bat"
$Adb = Join-Path $AndroidSdk "platform-tools\adb.exe"
$AndroidJar = Join-Path $Platform "android.jar"
$Javac = Join-Path $JavaHome "bin\javac.exe"
$Jar = Join-Path $JavaHome "bin\jar.exe"
$Keytool = Join-Path $JavaHome "bin\keytool.exe"

foreach ($tool in @($Aapt2, $D8, $Zipalign, $Apksigner, $AndroidJar, $Javac, $Jar, $Keytool)) {
    if (-not (Test-Path -LiteralPath $tool)) {
        throw "Required Android build tool not found: $tool"
    }
}

Remove-Item -LiteralPath $BuildRoot -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $GeneratedRoot, $CompiledRes, $ClassesRoot, $DexRoot | Out-Null
Copy-Item -LiteralPath $AndroidJar -Destination $LocalAndroidJar -Force

Write-Host "Android SDK: $AndroidSdk"
Write-Host "Build tools: $BuildTools"
Write-Host "Platform: $Platform"
Write-Host "Java: $JavaHome"

& $Aapt2 compile --dir (Join-Path $ProjectRoot "res") -o $CompiledRes
if ($LASTEXITCODE -ne 0) { throw "aapt2 compile failed" }

$CompiledResourceFiles = @(Get-ChildItem -Path $CompiledRes -Recurse -Filter "*.flat" | ForEach-Object { $_.FullName })
if ($CompiledResourceFiles.Count -eq 0) {
    throw "No compiled Android resource files found in $CompiledRes"
}

& $Aapt2 link `
    -o $UnsignedApk `
    -I $LocalAndroidJar `
    --manifest (Join-Path $ProjectRoot "AndroidManifest.xml") `
    --java $GeneratedRoot `
    $CompiledResourceFiles
if ($LASTEXITCODE -ne 0) { throw "aapt2 link failed" }

$SourceFiles = @(
    Join-Path $ProjectRoot "src\com\pynative\experiment\MainActivity.java"
) + @(Get-ChildItem -Path $GeneratedRoot -Recurse -Filter "*.java" | ForEach-Object { $_.FullName })

& $Javac `
    -encoding UTF-8 `
    -Xlint:-options `
    -source 17 `
    -target 17 `
    -classpath $LocalAndroidJar `
    -d $ClassesRoot `
    $SourceFiles
if ($LASTEXITCODE -ne 0) { throw "javac failed" }

& $Jar cf $ClassesJar -C $ClassesRoot .
if ($LASTEXITCODE -ne 0) { throw "creating classes jar failed" }

& $D8 `
    --lib $LocalAndroidJar `
    --output $DexRoot `
    $ClassesJar
if ($LASTEXITCODE -ne 0) { throw "d8 failed" }

& $Jar uf $UnsignedApk -C $DexRoot classes.dex
if ($LASTEXITCODE -ne 0) { throw "adding classes.dex failed" }

& $Zipalign -f 4 $UnsignedApk $AlignedApk
if ($LASTEXITCODE -ne 0) { throw "zipalign failed" }

if (-not (Test-Path -LiteralPath $Keystore)) {
    & $Keytool `
        -genkeypair `
        -keystore $Keystore `
        -storepass android `
        -alias androiddebugkey `
        -keypass android `
        -dname "CN=Android Debug,O=Android,C=US" `
        -keyalg RSA `
        -keysize 2048 `
        -validity 10000 | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "debug keystore generation failed" }
}

& $Apksigner sign `
    --ks $Keystore `
    --ks-pass pass:android `
    --key-pass pass:android `
    --out $OutputApk `
    $AlignedApk
if ($LASTEXITCODE -ne 0) { throw "apksigner failed" }

Write-Host "Built APK: $OutputApk"

if ($Install -or $Launch) {
    if (-not (Test-Path -LiteralPath $Adb)) {
        throw "adb not found: $Adb"
    }

    & $Adb devices
    if ($LASTEXITCODE -ne 0) { throw "adb devices failed" }

    & $Adb install -r $OutputApk
    if ($LASTEXITCODE -ne 0) { throw "adb install failed. Connect a device/emulator, then retry." }
}

if ($Launch) {
    & $Adb shell am start -n $ActivityName
    if ($LASTEXITCODE -ne 0) { throw "adb launch failed" }
}
