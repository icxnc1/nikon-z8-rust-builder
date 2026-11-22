# Developer setup: Windows 11 x64, Ubuntu 26.04 LTS, WSL2

The public repository provides Rust source, Windows/Linux executable releases, and two opt-in environment installers. Neither script downloads Nikon firmware or automatically flashes a camera. Running a prebuilt application requires no compiler, Python, QEMU or WSL; firmware builds additionally need *local upstream resources* (LOCAL_SETUP.md).

## Windows PowerShell

Inspect the installer before executing. Its default is a READ-ONLY check:

    powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\install-windows.ps1

Install development tools from the Microsoft winget sources:

    powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\install-windows.ps1 -Install

To also install Visual Studio C++ MSVC Build Tools with Windows SDK (large download, often needs Administrator):

    powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\install-windows.ps1 -Install -InstallMSVC

It handles these winget package IDs: Git.Git, Rustlang.Rustup, LLVM.LLVM, Kitware.CMake, Ninja-build.Ninja, Python.Python.3.13 and OpenJS.NodeJS.LTS. Native Rust MSVC builds need the C++ linker and Windows SDK; use -InstallMSVC if missing. Run the installer once more after a terminal restart to re-check the toolchain.

Verify and compile:

    cargo test --locked
    node .\test_i18n.cjs
    cargo build --release --locked

The native executable is target\release\nikon-z8-rust-builder.exe. Rust edition 2024 requires rustc 1.85+. Passing these tests does NOT validate firmware for flashing.

## Windows WSL2 + Ubuntu 26.04

In PowerShell (Administrator for enabling WSL):

    wsl --list --online
    wsl --install -d Ubuntu-26.04
    wsl --list --verbose

Alternatively, explicitly request WSL setup within the installer:

    powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\install-windows.ps1 -Install -InstallWSL2

The script avoids reinstalling an already existing Ubuntu-26.04 distribution. WSL feature activation may need Windows restart; create the Ubuntu Linux user on first launch. Confirm Ubuntu-26.04 shows VERSION 2.

In the Linux shell, as your normal user (not root):

    cd ~/nikon-z8-rust-builder
    bash install-ubuntu.sh --check
    bash install-ubuntu.sh --install
    bash install-ubuntu.sh --test
    cargo build --release --locked

The default --check does not install anything. --install runs apt-get update and installs native Ubuntu packages from configured apt repositories.

Packages: rustc cargo build-essential gcc g++ make clang lld llvm libclang-dev cmake ninja-build pkg-config libssl-dev; python3 python3-venv python3-pip nodejs npm git git-lfs gh jq openssl; QEMU AArch64 emulator via qemu-system-arm, qemu-user and qemu-utils; ARM64 cross compiler gcc-aarch64-linux-gnu and binutils-aarch64-linux-gnu; gdb-multiarch, file, xxd, zip, unzip, tar, curl, ca-certificates. Optional formal verification tools: z3 and cvc5 via bash install-ubuntu.sh --install --formal.

On Ubuntu 26.04 the distribution rustc/cargo is enough if rustc version >=1.85; rustup is not mandatory. Clang/LLVM is independent from Rust and permits native verification experiments; QEMU alone is NOT a complete Nikon Z8 emulator. On a new machine, the Ubuntu repositories can have newer versions, so always rerun --check after installation and use pinned Cargo.lock.

## Build and runtime verification levels

- cargo test: 4 Rust unit tests, not camera behavior.
- node test_i18n.cjs: 15 UI languages, 23 locale scenarios.
- Rust build/verify: requires your own exact Nikon original C3.11 and independently prepared upstream 0.5.1 toolkit/recipes.
- QEMU/Unicorn: research tools only; external Nikon dependencies may be stubbed; passing emulation never authorizes flashing.
- The scripts make no network requests to download Nikon firmware. See FIRMWARE_SOURCES.md for official source and third-party archival references.

All installers are plain text to support independent inspection; they do not submit secrets or diagnostics to any service.
