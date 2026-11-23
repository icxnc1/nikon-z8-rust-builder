#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
cd "$(dirname "$0")"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
cc -std=c11 -O2 -Wall -Wextra -Werror -pedantic -fsanitize=undefined,bounds -fno-sanitize-recover=all night_dng_v2.c tests/test_v2.c -o "$TMP/t"
"$TMP/t" "$TMP"
python3 tests/check_v2.py "$TMP"
cc -std=c11 -O2 -fPIC -shared night_dng_v2.c -o "$TMP/libnight.so"
NS2_LIB="$TMP/libnight.so" python3 tests/diff_fuzz.py
mkdir -p "$TMP/hdr"
python3 tests/check_hdr.py "$TMP/hdr"
if command -v clang >/dev/null; then
    clang -target aarch64-none-elf -ffreestanding -nostdlib -O2 -Wall -Wextra -Werror -c night_dng_v2.c -o "$TMP/arm64.o"
    file "$TMP/arm64.o"
fi
printf 'TEST_SUITE PASS\n'
