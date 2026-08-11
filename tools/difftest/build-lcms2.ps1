<#
=======================================================================
 tools/difftest/build-lcms2.ps1

 PURPOSE
   Build the pinned lcms2 on Windows with MSVC, producing the oracle
   binaries the differential tests invoke:

       vendor/build-msvc/transicc.exe   colour conversion calculator
       vendor/build-msvc/linkicc.exe    device-link builder
       vendor/build-msvc/psicc.exe      PostScript CRD/CSA emitter
       vendor/build-msvc/lcms2.lib      static core library
       vendor/build-msvc/testbed/testcms.exe   lcms2's own self-test

   Run fetch-lcms2.sh first; this script refuses to build a tree that is
   absent or not at the pinned commit.

 WHY A SCRIPT AND NOT "just run cmake"
   Because the flags are load-bearing and half of them are licence
   decisions, not build decisions. Writing them down once, here, is the
   difference between a reproducible oracle and a build that happened to
   work on somebody's machine on a Tuesday.

 TOOLCHAIN DISCOVERY
   This machine has no cmake, ninja or cl.exe on PATH. All three ship
   inside Visual Studio Build Tools, so the script finds the VS
   installation with vswhere.exe and uses the bundled copies:

     <VS>\VC\Auxiliary\Build\vcvars64.bat
     <VS>\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe
     <VS>\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe

   vswhere is asked with -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64
   because "a Visual Studio is installed" is not the same claim as "a C
   compiler is installed" — this machine has an instance of each kind,
   and picking the wrong one produces a confusing failure several minutes
   into a configure step.

   A cmake / ninja already on PATH is preferred if present, so a developer
   with a normal cmake install is not forced through the VS copies.

 THE BUILD FLAGS, AND WHY EACH ONE

   -DLCMS2_WITH_FASTFLOAT=OFF
   -DLCMS2_WITH_THREADED_PLUGIN=OFF
       LICENCE. These two plugins are GPL-3.0; everything else in lcms2
       is MIT. They are OFF upstream by default and we set them
       explicitly so that the intent is recorded rather than inherited.
       Turning either on would put GPL-3.0 code in the oracle, and
       although the oracle is out-of-tree and invoked as a subprocess,
       the correct posture is not to build it at all. See LEGAL.md §4.

       CORRECTNESS, secondarily: fast_float replaces lcms2's floating
       point pipeline with a faster approximate one. An oracle should be
       the reference implementation's most careful path, not its
       fastest. Comparing iccce against an approximation would make
       every disagreement ambiguous.

   -DLCMS2_BUILD_JPGICC=OFF -DLCMS2_BUILD_TIFICC=OFF -DLCMS2_BUILD_TIFDIFF=OFF
   -DLCMS2_WITH_JPEG=OFF -DLCMS2_WITH_TIFF=OFF -DLCMS2_WITH_ZLIB=OFF
       Those three tools need libjpeg / libtiff / zlib, which are not
       present here and which the oracle does not need: difftest compares
       colour numbers, not image files. Disabling them removes three
       find_package() calls that would otherwise fail or silently pull in
       whatever happens to be installed — a dependency the oracle picked
       up by accident is a dependency nobody pinned.

   -DLCMS2_BUILD_SHARED=OFF -DLCMS2_BUILD_STATIC=ON
       Static linking means transicc.exe is self-contained. No DLL to be
       found (or, worse, to be found in the wrong version from somewhere
       else on PATH) at the moment a test runs.

   -DCMAKE_BUILD_TYPE=Release
       Optimised, but note this is -O2 and NOT fast-math: MSVC defaults
       to /fp:precise. If a future toolchain change introduces
       /fp:fast the oracle's arithmetic would change under us, which is
       exactly the kind of silent movement this project is built to
       notice. Flagged here so the next person knows to look.

   -DLCMS2_BUILD_TESTS=ON
       Builds lcms2's own testbed. We run it once at pin time: an oracle
       that fails its own self-test is not an oracle, and establishing
       that it passes is a cheap, one-off piece of evidence that belongs
       in the record. See README.md, "Is the oracle sound?".

 EXIT CODES
   0  build succeeded; binaries are in vendor/build-msvc
   1  source tree missing or not at the pinned commit (run fetch first)
   2  no usable MSVC toolchain found — the message names exactly what is
      missing. This script does not fall back to a hand-waved build.
   3  cmake configure failed
   4  cmake build failed

 PARAMETERS
   -Clean       delete the build directory first
   -RunTestbed  after building, run lcms2's self-test and report
=======================================================================
#>

[CmdletBinding()]
param(
    [switch]$Clean,
    [switch]$RunTestbed
)

$ErrorActionPreference = 'Stop'

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$PinFile   = Join-Path $ScriptDir 'lcms2.pin'
$SrcDir    = Join-Path $ScriptDir 'vendor\lcms2'
$BuildDir  = Join-Path $ScriptDir 'vendor\build-msvc'

# ---------------------------------------------------------------------
# Step 1 — read the pin and confirm the checkout matches it.
#
# Same reasoning as fetch-lcms2.sh: a build of an unverified tree
# produces an oracle whose identity is unknown, and results attributed to
# an unknown oracle are not results.
# ---------------------------------------------------------------------
function Get-Pin([string]$Key) {
    $line = Select-String -Path $PinFile -Pattern "^$Key=(.*)$" | Select-Object -First 1
    if ($line) { return $line.Matches[0].Groups[1].Value.Trim() }
    return $null
}

if (-not (Test-Path $PinFile)) { Write-Error "pin file not found: $PinFile"; exit 1 }
$PinCommit = Get-Pin 'LCMS2_COMMIT'
$PinTag    = Get-Pin 'LCMS2_TAG'

if (-not (Test-Path (Join-Path $SrcDir '.git'))) {
    Write-Host "lcms2 source not present at $SrcDir"
    Write-Host "run:  sh tools/difftest/fetch-lcms2.sh"
    exit 1
}

$ActualCommit = (& git -C $SrcDir rev-parse HEAD).Trim()
if ($ActualCommit -ne $PinCommit) {
    Write-Host "PIN MISMATCH -- refusing to build."
    Write-Host "  expected $PinCommit ($PinTag)"
    Write-Host "  actual   $ActualCommit"
    Write-Host "  run:  sh tools/difftest/fetch-lcms2.sh --force"
    exit 1
}
Write-Host "lcms2 source verified at $PinTag ($PinCommit)"

# ---------------------------------------------------------------------
# Step 2 — find a C toolchain. Name what is missing rather than guessing.
# ---------------------------------------------------------------------
$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
if (-not (Test-Path $vswhere)) {
    Write-Host "MISSING: vswhere.exe at $vswhere"
    Write-Host "         Visual Studio (or Build Tools) does not appear to be installed."
    Write-Host "         Install 'Desktop development with C++' from"
    Write-Host "         https://visualstudio.microsoft.com/downloads/ (Build Tools is enough)."
    exit 2
}

# -requires ...VC.Tools.x86.x64 filters to instances that actually carry a
# C compiler. -latest then picks the newest of those.
$vsPath = & $vswhere -latest -products * `
    -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 `
    -property installationPath
if (-not $vsPath) {
    Write-Host "MISSING: no Visual Studio instance with the C++ toolset (VC.Tools.x86.x64)."
    Write-Host "         Instances found by vswhere:"
    & $vswhere -products * -property installationPath | ForEach-Object { Write-Host "           $_" }
    Write-Host "         Add the 'Desktop development with C++' workload to one of them."
    exit 2
}
$vsPath = $vsPath.Trim()
Write-Host "MSVC instance: $vsPath"

$vcvars = Join-Path $vsPath 'VC\Auxiliary\Build\vcvars64.bat'
if (-not (Test-Path $vcvars)) {
    Write-Host "MISSING: $vcvars"
    exit 2
}

# Prefer a cmake/ninja already on PATH; fall back to the VS-bundled copies.
$cmake = (Get-Command cmake -ErrorAction SilentlyContinue)?.Source
if (-not $cmake) {
    $cmake = Join-Path $vsPath 'Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe'
}
$ninja = (Get-Command ninja -ErrorAction SilentlyContinue)?.Source
if (-not $ninja) {
    $ninja = Join-Path $vsPath 'Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe'
}
if (-not (Test-Path $cmake)) { Write-Host "MISSING: cmake (not on PATH, not bundled at $cmake)"; exit 2 }
if (-not (Test-Path $ninja)) { Write-Host "MISSING: ninja (not on PATH, not bundled at $ninja)"; exit 2 }
Write-Host "cmake: $cmake"
Write-Host "ninja: $ninja"

if ($Clean -and (Test-Path $BuildDir)) {
    Write-Host "-Clean: removing $BuildDir"
    Remove-Item -Recurse -Force $BuildDir
}

# ---------------------------------------------------------------------
# Step 3 — configure.
#
# vcvars64.bat is a batch file that mutates environment variables, so it
# has to run in the same cmd.exe process as the tool it is setting up for.
# Hence the `cmd /c "vcvars && cmake"` shape rather than calling cmake
# directly from PowerShell.
# ---------------------------------------------------------------------
$configureArgs = @(
    '-G', 'Ninja'
    "-DCMAKE_MAKE_PROGRAM=`"$ninja`""
    '-DCMAKE_BUILD_TYPE=Release'
    '-DLCMS2_BUILD_TOOLS=ON'
    '-DLCMS2_BUILD_TESTS=ON'
    # --- licence: GPL-3.0 plugins stay off ---
    '-DLCMS2_WITH_FASTFLOAT=OFF'
    '-DLCMS2_WITH_THREADED_PLUGIN=OFF'
    # --- image-format tools we neither need nor have the libs for ---
    '-DLCMS2_BUILD_JPGICC=OFF'
    '-DLCMS2_BUILD_TIFICC=OFF'
    '-DLCMS2_BUILD_TIFDIFF=OFF'
    '-DLCMS2_WITH_JPEG=OFF'
    '-DLCMS2_WITH_TIFF=OFF'
    '-DLCMS2_WITH_ZLIB=OFF'
    # --- self-contained binaries ---
    '-DLCMS2_BUILD_SHARED=OFF'
    '-DLCMS2_BUILD_STATIC=ON'
    '-S', "`"$SrcDir`""
    '-B', "`"$BuildDir`""
) -join ' '

Write-Host ""
Write-Host "configuring ..."
cmd /c "`"$vcvars`" >nul && `"$cmake`" $configureArgs"
if ($LASTEXITCODE -ne 0) { Write-Host "cmake configure failed ($LASTEXITCODE)"; exit 3 }

Write-Host ""
Write-Host "building ..."
cmd /c "`"$vcvars`" >nul && `"$cmake`" --build `"$BuildDir`" --config Release"
if ($LASTEXITCODE -ne 0) { Write-Host "cmake build failed ($LASTEXITCODE)"; exit 4 }

Write-Host ""
Write-Host "built:"
foreach ($exe in 'transicc.exe','linkicc.exe','psicc.exe') {
    $p = Join-Path $BuildDir $exe
    if (Test-Path $p) { Write-Host ("  {0}  ({1} bytes)" -f $p, (Get-Item $p).Length) }
}

# ---------------------------------------------------------------------
# Step 4 (optional) — the oracle's own self-test.
#
# testcms.exe must run with its working directory set to the directory
# holding the generated test profiles, hence the --chdir argument.
# `--exhaustive` exists and takes considerably longer; the default run is
# what we record at pin time.
# ---------------------------------------------------------------------
if ($RunTestbed) {
    $testbed = Join-Path $BuildDir 'testbed\testcms.exe'
    if (-not (Test-Path $testbed)) { Write-Host "testbed not built"; exit 0 }
    Write-Host ""
    Write-Host "running lcms2 self-test ..."
    Push-Location (Split-Path -Parent $testbed)
    try {
        $out = & $testbed 2>&1
        $code = $LASTEXITCODE
    } finally { Pop-Location }
    $ok = ($out | Select-String -Pattern 'Ok\.' -AllMatches).Count
    Write-Host ("  exit code : {0}" -f $code)
    Write-Host ("  checks Ok : {0}" -f $ok)
    if ($code -ne 0) {
        Write-Host "  THE ORACLE FAILS ITS OWN SELF-TEST. Do not rely on it until this is understood."
        $out | Select-Object -Last 40 | ForEach-Object { Write-Host "    $_" }
    }
}
