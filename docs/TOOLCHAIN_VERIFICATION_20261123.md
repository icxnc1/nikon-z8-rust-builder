# Windows / Ubuntu 26.04 WSL2 toolchain audit (2026-11-23)

This record distinguishes **actually executed verification** from **packages merely offered by a setup script**. Local platform: Windows 11 x64, WSL2 Ubuntu 26.04.1, AMD64. This is **not a fresh-machine installation test**, and is not a camera safety certificate.

## Confirmed on the existing Windows machine

| Item | Observation |
|---|---|
| Windows Rust compiler | `rustc 1.99.0`, `stable-x86_64-pc-windows-msvc` |
| Native Rust source build | **PASS**, `cargo build --release --offline --locked` |
| PowerShell setup/check | **PASS**, `install-windows.ps1 -RunTests`; default no installation |
| Rust unit tests | **4/4 PASS**, compiled and run on Windows |
| UI localization (Node.js) | **15 language dictionaries and 23 browser-locale cases PASS** |
| Native binary release | Executable launched and returned `Nikon Z8 Native Rust Patch Builder`, experimental/tool-only |
| Git and Node.js | Found and executed successfully |
| Clang + CMake on Windows PATH | **MISSING / UNVERIFIED**; *not required* to compile the Rust firmware builder itself |
| Python on Windows | Only WindowsApps `python.exe` and `python3.exe` execution aliases detected, no `py.exe`; **real Python interpreter not verified**, so source-recipe generation on native Windows is **not ready** from this finding alone |
| MSVC/SDK | Actual Rust target builds; exact Visual Studio Build Tools/Windows SDK package inventory was not independently audited |
| winget | Accessible; verified `Rustlang.Rustup` registered and `Microsoft.VisualStudio.2022.BuildTools` package available |
| Fresh `install-windows.ps1 -Install` / `-InstallMSVC` | **NOT RUN**; installer behavior on a clean Windows machine remains **UNVERIFIED** |

## Confirmed in WSL2 Ubuntu 26.04.1

| Item | Observation |
|---|---|
| WSL distro | `Ubuntu-26.04`, **WSL version 2** |
| System Ubuntu apt Rust | `rustc/cargo 1.93.1ubuntu1` (the installer PATH); sufficient for Rust 2024 edition |
| Additional Rustup toolchain | `rustc/cargo 1.100.0` observed in a different bash startup PATH; distinguish from apt Rust |
| Clang/LLVM / CMake / Ninja | **Ubuntu Clang 21.1.8 (21.x), CMake 4.2.3, Ninja 1.13.2**, present; **not Apple Clang 21.0.0** |
| Native unit tests | **4/4 PASS** with `cargo test --offline --locked` |
| Linux Rust release | **PASS**, `cargo build --release --offline --locked` |
| Localization tests | **15 locales / 23 browser cases PASS** using explicitly invoked local Node binary |
| Node.js | v24.22.1 available at `~/.local/opt/node-v24.22.1-linux-x64/bin/node`, but `node` **NOT in the default WSL shell PATH**; installer check reports MISSING |
| AArch64 QEMU system/user, cross-GCC, gdb-multiarch | **NOT installed in this Ubuntu distro** during audit; no current emulator command can be claimed to run there |
| `z3` / `cvc5` | **Installed**, Z3 4.13.3 and cvc5 1.3.2; **no formal Nikon firmware proof** performed by those executables |
| Package dependency resolution | `apt-get -s install` for missing QEMU, cross-tools, Node and GitHub utilities **completed** against Ubuntu's configured repositories |
| `install-ubuntu.sh --check` | Script executed; **INCOMPLETE** because Node in PATH and ARM64 research tools were missing. Syntax `bash -n` **PASS** |
| `install-ubuntu.sh --install` | **NOT RUN**: requires apt administrative installation; do not report installed packages or a clean-machine pass |

The native builder **does not need QEMU, Clang, Python or Node.js at runtime**. These tools serve **source compilation, recipe generation, localization verification and independent emulation/research**, not camera flashing.

## How to recreate and test — minimal source build

Windows PowerShell in a newly cloned repository:

```powershell
git clone https://github.com/icxnc1/nikon-z8-rust-builder.git
cd nikon-z8-rust-builder
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\install-windows.ps1
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\install-windows.ps1 -Install -InstallMSVC
# Restart terminal after installing developer tools; verify actual compiler, SDK and paths
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\install-windows.ps1 -RunTests
cargo build --release --locked
.\target\release\nikon-z8-rust-builder.exe about
```

Ubuntu 26.04 via Windows WSL2 (or native Ubuntu):

```powershell
wsl --install -d Ubuntu-26.04
wsl --list --verbose
wsl -d Ubuntu-26.04
```

```bash
git clone https://github.com/icxnc1/nikon-z8-rust-builder.git
cd nikon-z8-rust-builder
bash install-ubuntu.sh --check
bash install-ubuntu.sh --install
bash install-ubuntu.sh --test
cargo build --release --locked
./target/release/nikon-z8-rust-builder about
```

The Ubuntu installer uses **apt and sudo** to install Rust/Cargo, GCC, Clang, LLVM/lld, CMake/Ninja, Python/pip/venv, Node/npm, Git/GitHub CLI, OpenSSL and **QEMU AArch64, GDB and ARM64 cross-compiler** from the configured Ubuntu repositories. Optional `--formal` adds Z3/cvc5. No installer downloads any Nikon firmware and neither installer flashes the camera.

**Privilege and reboot requirements:** Windows `-InstallMSVC` may require administrator permissions, WSL feature activation may require a reboot, and Ubuntu `--install` needs a normal Linux user with apt/sudo installation rights. When installation is unavailable, `--check` reports missing tools and does not modify the machine.

## Firmware-builder dependencies vs Night RAW research

- Source-compile dependency: **Rust/Cargo 1.85+**, Windows MSVC SDK/linker on Windows; Git needed for cloning, Node used only for `test_i18n.cjs`.
- Local v0.5.1 recipe-preparation dependency: a **real Python interpreter** plus the original upstream release's pinned **`capstone==5.0.7`** (`requirements.txt`), and your own correct original C3.11 firmware. This bootstrap produces private firmware-derived patch bytes that **must not be uploaded**.
- AArch64 firmware research: QEMU system/user, ARM64 binutils/GCC, GDB, Clang/LLVM; available package names in `install-ubuntu.sh`. **QEMU packages alone do not implement Nikon's peripherals**.
- Formal experiments: Z3/cvc5 are optional and cannot automatically certify a Nikon firmware image.
- **Offline RAW processor is a separate project** `z8_deep_raw`: requires its own `rawler` vendor patch, currently **missing** in local tree. It is **not a built component of this public installer**.
- Every binary in [Releases](../../releases) was downloaded from the public release and verified using the accompanying SHA256SUMS, but this does not establish clean-machine installer success or physical camera firmware safety.

[General setup](../SETUP_WINDOWS_WSL_UBUNTU.md) | [Night RAW/HDR research](../RESEARCH.md) | [Upstream toolkit](https://github.com/XinmiaoZhao/NikonFirmwarePatch)
