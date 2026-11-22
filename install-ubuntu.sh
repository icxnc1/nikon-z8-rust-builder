#!/usr/bin/env bash
# Nikon Z8 Rust Builder Ubuntu 26.04/WSL2 setup. No Nikon firmware downloaded.
set -Eeuo pipefail
MODE=check
FORMAL=no
for arg in "$@"; do
  case "$arg" in
    --check) MODE=check ;;
    --install) MODE=install ;;
    --test) MODE=test ;;
    --formal) FORMAL=yes ;;
    --help|-h)
      echo "Usage: bash install-ubuntu.sh [--check | --install | --test] [--formal]"
      echo "--check (default): report only; --install: apt dependencies; --test: cargo and UI tests"
      exit 0 ;;
    *) echo "Unrecognized argument: $arg" >&2; exit 2 ;;
  esac
done
if ! command -v apt-get >/dev/null 2>&1; then echo 'Ubuntu apt required' >&2; exit 2; fi
if [ ! -f /etc/os-release ]; then echo 'os-release missing' >&2; exit 2; fi
. /etc/os-release
if [ "$ID" != ubuntu ]; then echo "Ubuntu only: $ID" >&2; exit 2; fi
echo "Detected $PRETTY_NAME; mode=$MODE"
if [ "$VERSION_ID" != "26.04" ]; then echo "WARN: target is Ubuntu 26.04"; fi
PACKAGES=(
  ca-certificates curl git git-lfs gh
  build-essential gcc g++ make pkg-config
  clang lld llvm libclang-dev cmake ninja-build
  rustc cargo
  python3 python3-venv python3-pip nodejs npm
  jq openssl libssl-dev file xxd zip unzip tar
  qemu-system-arm qemu-user qemu-utils gdb-multiarch
  gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
)
if [ "$FORMAL" = yes ]; then PACKAGES+=(z3 cvc5); fi

if [ "$MODE" = install ]; then
  if [ "$(id -u)" -eq 0 ]; then echo 'Run as normal user, not as root' >&2; exit 2; fi
  command -v sudo >/dev/null || { echo 'sudo unavailable' >&2; exit 2; }
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends "${PACKAGES[@]}"
fi

FAILED=0
checkcmd() {
  if command -v "$1" >/dev/null 2>&1; then echo "PASS $1: $(command -v "$1")";
  else echo "MISSING $1"; FAILED=1; fi
}
for cmd in git gcc g++ clang clang++ llvm-objdump cmake ninja rustc cargo \
  python3 node npm qemu-system-aarch64 qemu-aarch64 gdb-multiarch \
  aarch64-linux-gnu-gcc aarch64-linux-gnu-objdump sha256sum; do
  checkcmd "$cmd"
done
if [ "$FORMAL" = yes ]; then checkcmd z3; checkcmd cvc5; fi
if command -v rustc >/dev/null 2>&1; then
  RVER="$(rustc --version | awk '{print $2}')"
  echo "Rust version: $RVER"
  if ! printf '%s\n%s\n' '1.85.0' "$RVER" | sort -V -C; then
    echo 'Rust 1.85+ required (edition 2024)' >&2; FAILED=1
  fi
fi
if grep -qi microsoft /proc/version 2>/dev/null; then
  echo "WSL detected: confirm WSL version 2 with Windows command: wsl --list --verbose"
fi

if [ "$MODE" = test ]; then
  HERE="$(cd -- "$(dirname -- "$0")" && pwd -P)"
  echo 'Running Rust unit tests (locked dependencies)'
  CARGO_TARGET_DIR="$HOME/.cache/nikon-z8-rust-target" \
    cargo test --manifest-path "$HERE/Cargo.toml" --locked
  echo 'Running 15-language UI tests'
  node "$HERE/test_i18n.cjs"
  echo 'NIKON_RUST_UBUNTU_TEST_PASS'
fi

if [ "$FAILED" -ne 0 ]; then
  echo 'NIKON_UBUNTU_TOOLS_INCOMPLETE: run bash install-ubuntu.sh --install' >&2
  exit 3
fi
echo 'NIKON_UBUNTU_TOOLCHAIN_OK (camera firmware flashing NOT approved)'
