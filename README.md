# Nikon Z8 Rust Patch Builder — Windows / Linux / WSL

> **⚠️ EXPERIMENTAL — WORK IN PROGRESS — EXPERTS ONLY. NOT APPROVED FOR FLASHING.**
> Modified firmware may **permanently disable a camera**. No beginner tutorials or individual troubleshooting are provided. Bug reports and proposals require reproducible, well-documented evidence. **Read [the mandatory bilingual warning and contribution policy](WARNING.md) before using this project.**

**Community research project — NOT official Nikon software.** This independent Rust application is **not reviewed, maintained or supported by XinmiaoZhao/NikonFirmwarePatch**. Report issues about builds made with this tool **[here](../../issues)**, not upstream; a claim of upstream-equivalent output requires the same version, feature groups and SHA-256. See the [version-specific SHA / Apple Clang / adversarial cross-check report](docs/UPSTREAM_SHA256_APPLECLANG_NIGHT_DNG16_20261123.md).

A native **Rust** firmware builder/verifier and local browser-based GUI for Windows x64 and Linux/Ubuntu WSL, derived from the workflow in [XinmiaoZhao/NikonFirmwarePatch](https://github.com/XinmiaoZhao/NikonFirmwarePatch) v0.5.1 (upstream commit 25fefa1e17f321ee285bea77b3c54427f9912e4b). Licensed GPL-3.0-only; the original author is credited.

## Research in progress: Night RAW and Night HDR

**Night RAW and Night HDR are NOT contained in the released builder or any proven installable firmware.** We are experimentally combining 4/8/16 Nikon Z8 RAW frames for lower nighttime shadow noise and testing −2/0/+2 EV Bayer-domain HDR. Actual **16-frame dark NEFs** measured 21.14 → 5.85 DN noise sigma; independently checked offline arithmetic matched on a **1024×1024 ROI**. This is **NOT a validated in-camera Night mode or a claim of greater sensor ADC bit depth**. Read [the full Night RAW/HDR research, evidence and capture protocol](RESEARCH.md) and the [Claude / real RAW adversarial audit](docs/CLAUDE_REAL_RAW_EVIDENCE_20261123.md).

## Installation, compilers and firmware downloads

- [Verified vs missing tools and tests, Windows 11 + Ubuntu 26.04 WSL2](docs/TOOLCHAIN_VERIFICATION_20261123.md)
- [Windows PowerShell / WSL2 Ubuntu 26.04 / Ubuntu automatic installation and verification](SETUP_WINDOWS_WSL_UBUNTU.md)
- [Windows installer: install-windows.ps1](install-windows.ps1) (check-only by default)
- [Ubuntu 26.04 installer: install-ubuntu.sh](install-ubuntu.sh) (check-only by default)
- [Firmware source links including the owner's Softpedia Nikon Z8 search](FIRMWARE_SOURCES.md)
- [Locally preparing original firmware and upstream v0.5.1 resources](LOCAL_SETUP.md)

## Applications (source + downloadable binaries)

See the [Releases](../../releases) page for the Windows x64 executable and Linux x86-64 executable. The program itself is written in Rust and **does not use Python at runtime**. The GUI is served by a native Rust process on localhost (127.0.0.1), displayed in your existing web browser, **not** a native windowing toolkit.

The GitHub release binaries are **tool-only**: no Nikon firmware and no Nikon-derived binary patch payloads are included. A local setup is required before the build operation works; see [LOCAL_SETUP.md](LOCAL_SETUP.md). After setup, the Rust tool checks input file hashes, 15 reviewed feature combinations and 20 upstream binary units, patch windows, CRC16, rebuilding and readback verification. Generated output never receives automatic camera installation approval.

### Windows x64

Download the Windows .exe from Releases and use a command prompt:

    nikon-z8-rust-builder-windows-x64.exe gui --toolkit "C:\local\upstream-v051" --recipes "C:\local\generated-recipes" --port 8769

Then open http://127.0.0.1:8769/ in your browser. An unmodified Nikon C3.11 image must be provided locally.

### Linux x86-64

Download the Linux executable from Releases, make it executable and run:

    chmod +x nikon-z8-rust-builder-linux-x64
    ./nikon-z8-rust-builder-linux-x64 gui --toolkit /local/upstream-v051 --recipes /local/generated-recipes --port 8768

Then open http://127.0.0.1:8768/.

### Compile from source

    cargo build --release --locked
    cargo test --locked
    node test_i18n.cjs

The localization test verifies **15 languages** and **23 browser-language cases**.

### GUI languages

Romanian, English, German, Japanese, Simplified Chinese, Traditional Chinese, Spanish, Italian, French, Polish, Norwegian Bokmål, Swedish, Finnish, Portuguese and Korean. The selector preserves preferences locally; translation is fully bundled, requiring no cloud service. Russian and Ukrainian are not offered by this project.

## Testing and evidence (with clear limits)

- Rust full image reproduction: the local tool reproduced the C3.11-based experimental 04.08.p0 image **bit-for-bit**, SHA-256 73a151f7ce1f033a5eaadb44cfcf5e37c74bc250addc7841061a3a606fc4bab5, with independent Python cross-check.
- Version regression: all 7 feature combinations without the video group had matching main images across upstream v0.5.0 and v0.5.1.
- QEMU: 10/10 exercised AArch64 firmware-hook cases passed, with external Nikon functions stubbed; **this is not full firmware/camera emulation**.
- The project owner **reports having installed and tested Nikon Z8 firmware built using upstream v0.5.0 and v0.5.1** on a physical Z8. This is a **user-reported hardware observation**, not independent instrumented proof or blanket certification of all features. In particular, Night RAW and Night HDR are not implemented/tested as in-camera features.

The exact C3.11 input must match the pinned SHA-256. **C3.10, C3.12, and new Night modes are not currently supported as safe installable outputs of this builder.**

## Collaboration and roadmap

Read [RESEARCH.md](RESEARCH.md) for the planned Night RAW stacking, Night HDR exposure fusion, metadata preservation, motion/ghost handling and resource safety. Contributions from upstream maintainers are welcome, including bug reports and algorithmic criticism.

**No Nikon original/update binary, patched binary, derived patch payload, memory dump, private repository data or API credentials are in this public repository or release.** Source is under GPL-3.0-only. Not affiliated with Nikon.
