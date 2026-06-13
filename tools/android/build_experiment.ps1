param(
    [string]$AppSpec,
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
$GeneratedJavaRoot = Join-Path $BuildRoot "generated-java"
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

function Get-SpecValue($Spec, $Name, $Default) {
    if ($Spec.PSObject.Properties.Name -contains $Name) {
        return $Spec.$Name
    }

    return $Default
}

function ConvertTo-Array($Value) {
    if ($null -eq $Value) {
        return @()
    }

    if ($Value -is [System.Array]) {
        return @($Value)
    }

    return @($Value)
}

function ConvertTo-JavaString($Value) {
    if ($null -eq $Value) {
        $Value = ""
    }

    $text = [string]$Value
    $text = $text.Replace('\', '\\')
    $text = $text.Replace('"', '\"')
    $text = $text.Replace("`r", '\r')
    $text = $text.Replace("`n", '\n')
    $text = $text.Replace("`t", '\t')
    return '"' + $text + '"'
}

function ConvertTo-JavaStringArray($Values) {
    $items = @(ConvertTo-Array $Values | ForEach-Object { ConvertTo-JavaString $_ })
    if ($items.Count -eq 0) {
        return "new String[]{}"
    }

    return "new String[]{" + ($items -join ", ") + "}"
}

function ConvertTo-JavaElementArray($Elements) {
    $rows = @()

    foreach ($element in (ConvertTo-Array $Elements)) {
        $style = Get-SpecValue $element "style" ([pscustomobject]@{})
        $values = @(
            Get-SpecValue $element "kind" "Text"
            Get-SpecValue $element "value" ""
            Get-SpecValue $style "color" ""
            Get-SpecValue $style "background_color" ""
            Get-SpecValue $style "font_size" ""
            Get-SpecValue $style "font_weight" ""
            Get-SpecValue $style "width" ""
            Get-SpecValue $style "height" ""
            Get-SpecValue $style "padding" ""
            Get-SpecValue $style "margin" ""
            Get-SpecValue $style "align" ""
        )
        $rows += "        new String[]{" + (($values | ForEach-Object { ConvertTo-JavaString $_ }) -join ", ") + "}"
    }

    if ($rows.Count -eq 0) {
        return "new String[][]{}"
    }

    return "new String[][]{`n" + ($rows -join ",`n") + "`n    }"
}

function Write-GeneratedAppSource($SpecPath, $Destination) {
    if ($SpecPath) {
        if (-not (Test-Path -LiteralPath $SpecPath)) {
            throw "Android app spec not found: $SpecPath"
        }

        $spec = Get-Content -Raw -LiteralPath $SpecPath | ConvertFrom-Json
    } else {
        $spec = [pscustomobject]@{
            title = "PyNative Android Experiment"
            source_path = "built-in experiment"
            texts = @("Count: 0")
            buttons = @("Increase")
            inputs = @("Username")
            images = @()
            root_style = [pscustomobject]@{
                background_color = "#F8FAFC"
                padding = 40
            }
            elements = @(
                [pscustomobject]@{
                    kind = "Text"
                    value = "Count: 0"
                    style = [pscustomobject]@{
                        color = "#0F172A"
                        font_size = 18
                    }
                },
                [pscustomobject]@{
                    kind = "Input"
                    value = "Username"
                    style = [pscustomobject]@{}
                },
                [pscustomobject]@{
                    kind = "Button"
                    value = "Increase"
                    style = [pscustomobject]@{}
                }
            )
            has_python_callbacks = $false
            node_count = 0
            max_depth = 0
        }
    }

    $rootStyle = Get-SpecValue $spec "root_style" ([pscustomobject]@{})
    $title = ConvertTo-JavaString (Get-SpecValue $spec "title" "PyNative Android")
    $sourcePath = ConvertTo-JavaString (Get-SpecValue $spec "source_path" "built-in experiment")
    $rootBackgroundColor = ConvertTo-JavaString (Get-SpecValue $rootStyle "background_color" "#F8FAFC")
    $rootPadding = [int](Get-SpecValue $rootStyle "padding" 40)
    $texts = ConvertTo-JavaStringArray (Get-SpecValue $spec "texts" @())
    $buttons = ConvertTo-JavaStringArray (Get-SpecValue $spec "buttons" @())
    $inputs = ConvertTo-JavaStringArray (Get-SpecValue $spec "inputs" @())
    $images = ConvertTo-JavaStringArray (Get-SpecValue $spec "images" @())
    $elements = ConvertTo-JavaElementArray (Get-SpecValue $spec "elements" @())
    $nodeCount = [int](Get-SpecValue $spec "node_count" 0)
    $maxDepth = [int](Get-SpecValue $spec "max_depth" 0)
    $hasPythonCallbacks = if ([bool](Get-SpecValue $spec "has_python_callbacks" $false)) { "true" } else { "false" }

    $destinationRoot = Split-Path -Parent $Destination
    New-Item -ItemType Directory -Path $destinationRoot -Force | Out-Null

    $javaSource = @"
package com.pynative.experiment;

public final class GeneratedApp {
    public static final String TITLE = $title;
    public static final String SOURCE_PATH = $sourcePath;
    public static final String ROOT_BACKGROUND_COLOR = $rootBackgroundColor;
    public static final int ROOT_PADDING = $rootPadding;
    public static final String[] TEXTS = $texts;
    public static final String[] BUTTON_LABELS = $buttons;
    public static final String[] INPUT_PLACEHOLDERS = $inputs;
    public static final String[] IMAGES = $images;
    public static final String[][] ELEMENTS = $elements;
    public static final int NODE_COUNT = $nodeCount;
    public static final int MAX_DEPTH = $maxDepth;
    public static final boolean HAS_PYTHON_CALLBACKS = $hasPythonCallbacks;

    private GeneratedApp() {
    }
}
"@

    $utf8NoBom = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllText($Destination, $javaSource, $utf8NoBom)
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
New-Item -ItemType Directory -Path $GeneratedRoot, $GeneratedJavaRoot, $CompiledRes, $ClassesRoot, $DexRoot | Out-Null
Copy-Item -LiteralPath $AndroidJar -Destination $LocalAndroidJar -Force
$GeneratedAppSource = Join-Path $GeneratedJavaRoot "com\pynative\experiment\GeneratedApp.java"
Write-GeneratedAppSource $AppSpec $GeneratedAppSource

Write-Host "Android SDK: $AndroidSdk"
Write-Host "Build tools: $BuildTools"
Write-Host "Platform: $Platform"
Write-Host "Java: $JavaHome"
if ($AppSpec) {
    Write-Host "App spec: $AppSpec"
}

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
    $GeneratedAppSource
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
