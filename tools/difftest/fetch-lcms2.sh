#!/bin/sh
# =====================================================================
# tools/difftest/fetch-lcms2.sh
#
# PURPOSE
#   Obtain the pinned lcms2 source tree into tools/difftest/vendor/lcms2,
#   and refuse to proceed if what arrived is not what the pin names.
#
# WHY THIS EXISTS AT ALL
#   lcms2 is the differential oracle for iccce. An oracle whose version
#   drifts is worse than no oracle: a test that "passed against lcms2"
#   without recording WHICH lcms2 is an unfalsifiable claim, because the
#   next person cannot reproduce the comparison. So the version is pinned
#   to a commit hash, the hash is committed to this repository, and this
#   script verifies the hash rather than trusting the tag.
#
#   Tags are mutable. `git clone --branch <tag>` will happily give you a
#   different tree if upstream force-moves the tag. The commit hash is the
#   only thing that cannot lie, so the hash is what we check.
#
# WHY THE SOURCE IS NOT VENDORED INTO THE REPOSITORY
#   Two reasons, in increasing order of importance:
#     1. lcms2 is a third-party codebase we neither own nor maintain, and
#        an MIT colour engine should not carry a second colour engine in
#        its history.
#     2. lcms2's licensing is NOT uniform. The core library and the
#        command-line utilities are MIT; the optional plugins
#        plugins/fast_float and plugins/threaded are GPL-3.0. Keeping the
#        clone out of the tree keeps GPL-3.0 source out of an MIT
#        repository as a matter of fact, rather than as a matter of a
#        build flag someone might flip. See LEGAL.md §4.
#
# INPUTS
#   tools/difftest/lcms2.pin   — LCMS2_URL, LCMS2_TAG, LCMS2_COMMIT
#
# OUTPUTS
#   tools/difftest/vendor/lcms2/   — a detached-HEAD checkout at the pin.
#                                    Git-ignored; see the repo .gitignore.
#
# EXIT CODES
#   0  the tree at vendor/lcms2 is present and its HEAD equals LCMS2_COMMIT
#   1  pin file missing or malformed
#   2  git not available
#   3  clone or fetch failed (usually network)
#   4  HEAD does not match the pinned commit — THE ORACLE IS NOT THE
#      PINNED ORACLE. This is a hard stop, never a warning: continuing
#      would produce comparison results attributed to a version that did
#      not produce them.
#
# IDEMPOTENT
#   Safe to re-run. If vendor/lcms2 already exists and is at the pinned
#   commit, it does nothing and exits 0. Pass --force to delete and
#   re-clone (use this after moving the pin).
# =====================================================================

set -e

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PIN_FILE="$SCRIPT_DIR/lcms2.pin"
VENDOR_DIR="$SCRIPT_DIR/vendor"
SRC_DIR="$VENDOR_DIR/lcms2"

FORCE=0
for arg in "$@"; do
    case "$arg" in
        --force) FORCE=1 ;;
        -h|--help)
            sed -n '2,60p' "$0"
            exit 0
            ;;
        *)
            echo "fetch-lcms2.sh: unknown argument '$arg'" >&2
            exit 1
            ;;
    esac
done

# ---------------------------------------------------------------------
# Step 1 — read the pin.
#
# Deliberately parsed by hand rather than `. "$PIN_FILE"`. Sourcing a file
# executes it; a pin file is data and must not be able to run anything.
# ---------------------------------------------------------------------
if [ ! -f "$PIN_FILE" ]; then
    echo "fetch-lcms2.sh: pin file not found: $PIN_FILE" >&2
    exit 1
fi

pin_get() {
    # $1 = key. Prints the value, or nothing if absent.
    sed -n "s/^$1=\\(.*\\)\$/\\1/p" "$PIN_FILE" | head -n 1
}

LCMS2_URL=$(pin_get LCMS2_URL)
LCMS2_TAG=$(pin_get LCMS2_TAG)
LCMS2_COMMIT=$(pin_get LCMS2_COMMIT)

if [ -z "$LCMS2_URL" ] || [ -z "$LCMS2_TAG" ] || [ -z "$LCMS2_COMMIT" ]; then
    echo "fetch-lcms2.sh: pin file is missing LCMS2_URL, LCMS2_TAG or LCMS2_COMMIT" >&2
    exit 1
fi

echo "pin: $LCMS2_TAG ($LCMS2_COMMIT)"
echo "from: $LCMS2_URL"

command -v git >/dev/null 2>&1 || { echo "fetch-lcms2.sh: git not found on PATH" >&2; exit 2; }

# ---------------------------------------------------------------------
# Step 2 — obtain the tree, unless it is already correct.
# ---------------------------------------------------------------------
if [ "$FORCE" -eq 1 ] && [ -d "$SRC_DIR" ]; then
    echo "--force: removing $SRC_DIR"
    rm -rf "$SRC_DIR"
fi

if [ -d "$SRC_DIR/.git" ]; then
    echo "existing checkout found at $SRC_DIR"
else
    mkdir -p "$VENDOR_DIR"
    echo "cloning (shallow, single tag) ..."
    # --depth 1 --branch <tag> keeps this to a few MB. We do not need
    # lcms2's history; we need exactly one tree.
    git clone --quiet --depth 1 --branch "$LCMS2_TAG" "$LCMS2_URL" "$SRC_DIR" \
        || { echo "fetch-lcms2.sh: clone failed" >&2; exit 3; }
fi

# ---------------------------------------------------------------------
# Step 3 — verify. This is the step the whole script exists for.
# ---------------------------------------------------------------------
ACTUAL=$(git -C "$SRC_DIR" rev-parse HEAD)
if [ "$ACTUAL" != "$LCMS2_COMMIT" ]; then
    echo "" >&2
    echo "fetch-lcms2.sh: PIN MISMATCH -- refusing to proceed." >&2
    echo "  expected HEAD: $LCMS2_COMMIT   (from lcms2.pin)" >&2
    echo "  actual   HEAD: $ACTUAL" >&2
    echo "" >&2
    echo "  Either the checkout is stale (re-run with --force), or upstream" >&2
    echo "  moved the tag '$LCMS2_TAG'. If the latter, do NOT simply update" >&2
    echo "  the pin: read the upstream LICENSE and plugin headers at the new" >&2
    echo "  commit and re-record the verification in docs/LEGAL.md §4 first." >&2
    exit 4
fi

echo "verified: HEAD == $LCMS2_COMMIT"

# ---------------------------------------------------------------------
# Step 4 — restate the licence situation on every fetch.
#
# Not decoration. The GPL-3.0 plugins are the one fact about this
# dependency that can hurt us, and a fact printed only in a document is a
# fact nobody reads.
# ---------------------------------------------------------------------
cat <<'EOF'

licence: MIT for the core library (src/, include/) and the command-line
         utilities we use. NOT MIT for plugins/fast_float and
         plugins/threaded, which are GPL-3.0. The build scripts here set
         -DLCMS2_WITH_FASTFLOAT=OFF -DLCMS2_WITH_THREADED_PLUGIN=OFF
         (which is also upstream's default). Do not turn them on.
         See docs/LEGAL.md §4.

next: build-lcms2.ps1   (Windows / MSVC)
      build-lcms2.sh    (Linux, macOS, MSYS2)
EOF
