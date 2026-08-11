#!/bin/sh
# =====================================================================
# tools/difftest/build-lcms2.sh
#
# PURPOSE
#   Build the pinned lcms2 on a POSIX system (Linux, macOS, MSYS2) with
#   cmake, producing the same oracle binaries build-lcms2.ps1 produces on
#   Windows:
#
#       vendor/build-posix/transicc
#       vendor/build-posix/linkicc
#       vendor/build-posix/psicc
#       vendor/build-posix/testbed/testcms
#
#   ROADMAP Pass 0 requires CI on Linux as well as Windows, and the
#   sibling project's lesson was that an unchecked platform quietly stops
#   compiling. This is the Linux half.
#
#   NOT YET EXERCISED. As of 2026-08-11 this script has been written but
#   not run: the development machine is Windows-only and has no POSIX C
#   toolchain (no gcc, no clang, no make in Git Bash). It is here so CI
#   has something to call, and it is stated as unexercised so nobody
#   mistakes "a script exists" for "the Linux build works". Whoever first
#   runs Linux CI should replace this paragraph with the result.
#
# FLAGS
#   Identical to build-lcms2.ps1, and for identical reasons. The
#   important pair is:
#       -DLCMS2_WITH_FASTFLOAT=OFF -DLCMS2_WITH_THREADED_PLUGIN=OFF
#   Those two plugins are GPL-3.0 while the rest of lcms2 is MIT. Do not
#   turn them on. See docs/LEGAL.md §4 and the long commentary in
#   build-lcms2.ps1.
#
# EXIT CODES
#   0  build succeeded
#   1  source tree missing or not at the pinned commit (run fetch first)
#   2  no usable C toolchain / cmake — the message names what is missing
#   3  cmake configure failed
#   4  cmake build failed
#
# ARGUMENTS
#   --clean         remove the build directory first
#   --run-testbed   run lcms2's own self-test after building
# =====================================================================

set -e

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PIN_FILE="$SCRIPT_DIR/lcms2.pin"
SRC_DIR="$SCRIPT_DIR/vendor/lcms2"
BUILD_DIR="$SCRIPT_DIR/vendor/build-posix"

CLEAN=0
RUN_TESTBED=0
for arg in "$@"; do
    case "$arg" in
        --clean)        CLEAN=1 ;;
        --run-testbed)  RUN_TESTBED=1 ;;
        *) echo "build-lcms2.sh: unknown argument '$arg'" >&2; exit 1 ;;
    esac
done

pin_get() { sed -n "s/^$1=\\(.*\\)\$/\\1/p" "$PIN_FILE" | head -n 1; }

[ -f "$PIN_FILE" ] || { echo "pin file not found: $PIN_FILE" >&2; exit 1; }
PIN_COMMIT=$(pin_get LCMS2_COMMIT)
PIN_TAG=$(pin_get LCMS2_TAG)

if [ ! -d "$SRC_DIR/.git" ]; then
    echo "lcms2 source not present at $SRC_DIR" >&2
    echo "run:  sh tools/difftest/fetch-lcms2.sh" >&2
    exit 1
fi

ACTUAL=$(git -C "$SRC_DIR" rev-parse HEAD)
if [ "$ACTUAL" != "$PIN_COMMIT" ]; then
    echo "PIN MISMATCH -- refusing to build." >&2
    echo "  expected $PIN_COMMIT ($PIN_TAG)" >&2
    echo "  actual   $ACTUAL" >&2
    exit 1
fi
echo "lcms2 source verified at $PIN_TAG ($PIN_COMMIT)"

# --- toolchain: name what is missing, do not improvise -----------------
MISSING=""
command -v cmake >/dev/null 2>&1 || MISSING="$MISSING cmake"
if ! command -v cc >/dev/null 2>&1 && ! command -v gcc >/dev/null 2>&1 && ! command -v clang >/dev/null 2>&1; then
    MISSING="$MISSING c-compiler(cc|gcc|clang)"
fi
if ! command -v make >/dev/null 2>&1 && ! command -v ninja >/dev/null 2>&1; then
    MISSING="$MISSING build-tool(make|ninja)"
fi
if [ -n "$MISSING" ]; then
    echo "MISSING:$MISSING" >&2
    echo "  Debian/Ubuntu:  sudo apt-get install build-essential cmake" >&2
    echo "  Fedora:         sudo dnf install gcc make cmake" >&2
    echo "  macOS:          xcode-select --install && brew install cmake" >&2
    exit 2
fi

[ "$CLEAN" -eq 1 ] && rm -rf "$BUILD_DIR"

cmake -S "$SRC_DIR" -B "$BUILD_DIR" \
    -DCMAKE_BUILD_TYPE=Release \
    -DLCMS2_BUILD_TOOLS=ON \
    -DLCMS2_BUILD_TESTS=ON \
    -DLCMS2_WITH_FASTFLOAT=OFF \
    -DLCMS2_WITH_THREADED_PLUGIN=OFF \
    -DLCMS2_BUILD_JPGICC=OFF \
    -DLCMS2_BUILD_TIFICC=OFF \
    -DLCMS2_BUILD_TIFDIFF=OFF \
    -DLCMS2_WITH_JPEG=OFF \
    -DLCMS2_WITH_TIFF=OFF \
    -DLCMS2_WITH_ZLIB=OFF \
    -DLCMS2_BUILD_SHARED=OFF \
    -DLCMS2_BUILD_STATIC=ON \
    || exit 3

cmake --build "$BUILD_DIR" --config Release || exit 4

echo ""
echo "built:"
for exe in transicc linkicc psicc; do
    [ -f "$BUILD_DIR/$exe" ] && echo "  $BUILD_DIR/$exe"
done

if [ "$RUN_TESTBED" -eq 1 ] && [ -x "$BUILD_DIR/testbed/testcms" ]; then
    echo ""
    echo "running lcms2 self-test ..."
    ( cd "$BUILD_DIR/testbed" && ./testcms > /tmp/lcms2-testbed.log 2>&1 ) && code=0 || code=$?
    echo "  exit code : $code"
    echo "  checks Ok : $(grep -c 'Ok\.' /tmp/lcms2-testbed.log || true)"
    [ "$code" -ne 0 ] && echo "  THE ORACLE FAILS ITS OWN SELF-TEST -- see /tmp/lcms2-testbed.log"
fi
