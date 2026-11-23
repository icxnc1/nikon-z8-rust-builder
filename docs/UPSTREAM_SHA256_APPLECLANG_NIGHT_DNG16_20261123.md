# Nikon Z8 upstream binary identity, cross-checks and Night DNG v2 — 2026-11-23

**Independent research; NOT maintained or reviewed by Xinmiao Zhao. No modified firmware or Nikon code is redistributed. `FLASH_APPROVED=NO`.**

## 1. Exact releases and checksums

All example packages below use the independently acquired, unchanged **Nikon Z8 C 3.11**, file `Z_8_0311.bin`, **99,051,117 bytes**, SHA-256:

```text
d67413c2ede6b9ce8887175f20700bd9fbad2a973a4820968c45503d15d80d96
```

| Original source | Toolkit/release | Four groups, version and tag | SHA-256 | Evidence |
|---|---|---|---|---|
| Nikon C3.11 | [upstream v0.4.0](https://github.com/XinmiaoZhao/NikonFirmwarePatch/tree/v0.4.0) | `04.06.p0`, `Z_8_0406.bin` | `4deca457f0e3af1ea858e53db55ce4946cd99d916c663304e351047c049d2a63` | The provided **original C3.11 and generated 04.06 files were re-hashed directly in an isolated analysis container on 2026-11-23**, both exact matches against the author's v0.4.0 README. This is not a fresh Windows/WSL or hardware execution. |
| Nikon C3.11 | [upstream v0.5.0](https://github.com/XinmiaoZhao/NikonFirmwarePatch/tree/v0.5.0) | `04.07.p0`, `Z_8_0407.bin` | `e084a43bfb37650ead5a55f2a0c582b4b0ff82513aca95da0d99144d3d576533` | Exact upstream v0.5.0 README reference and previously recorded local build/verification results; **the image is not present here for a new direct hash calculation**. |
| Nikon C3.11 | [upstream v0.5.1](https://github.com/XinmiaoZhao/NikonFirmwarePatch/tree/v0.5.1) | `04.08.p0`, `Z_8_0408.bin` | `73a151f7ce1f033a5eaadb44cfcf5e37c74bc250addc7841061a3a606fc4bab5` | Exact upstream v0.5.1 README reference and prior local Python/Rust reproduction records. **Not directly re-hashed in this isolated analysis container**. |

**Important:** 04.06/04.07/04.08 are output version identities, not Nikon firmware source versions. All three examples start with stock **C3.11**, *not* C3.10 or C3.12. Use the matching upstream release's own reviewed prebuilt units and feature names. SHA identity proves reproducibility of bytes, **not physical safety**.

For independently checking an exact example, the upstream `patch.py` creates the package and upstream `verify.py` separately inspects the container, patch windows, preserved bytes and checksums. Reference commands are [v0.4.0 README](https://github.com/XinmiaoZhao/NikonFirmwarePatch/blob/v0.4.0/README.md#english), [v0.5.0 README](https://github.com/XinmiaoZhao/NikonFirmwarePatch/blob/v0.5.0/README.md#english), [v0.5.1 README](https://github.com/XinmiaoZhao/NikonFirmwarePatch/blob/v0.5.1/README.md#english). The [Rust builder](../README.md) has a distinct independent verifier, with pinned SHA-256, per-window old/new checks, untouched-region detection, Nikon wrapper and CRC16 readback; original code/patches remain attributed under GPL-3.0.

## 2. Apple Clang source reproducibility: actual GitHub Actions evidence

A GitHub Actions **macos-26-arm64** job, `run_id=45056866745`, completed **successfully on 2026-11-22** using `/Applications/Xcode_26.5.0.app`, `Apple clang version 21.0.0 (clang-2100.1.1.101)`.

Recorded job-log summary:

```text
COMPILER Xcode_26.5.0_EXACT CODE_HASH 20 FULL_RECIPE 20 / 20
COMPILER Xcode_26.5_EXACT_ALIAS CODE_HASH 20 FULL_RECIPE 20 / 20
```

The job fetched **only the public source**, compiled **20/20 v0.5.1 source patch units**, and matched their reviewed code hashes and recipe metadata. It uploaded **text-only evidence**, not Nikon firmware. A separate macOS run `45056317207` also completed successfully on exact Clang 2100.1.1.101; the alternative Xcode 27.2 Clang 2100.4.1.1 agreed on the ASM units but not the three C units. The associated Actions repository is **private**; these run IDs are provenance references, **not publicly inspectable evidence to an uninvited external reviewer**. Exact upstream release pins and public reproducibility instructions are the preferred external proof.

Cross-check distinctions:
- **Python:** upstream package builder + upstream independent `verify.py`; additional local diff/relocation checkers produced source-version-specific reports.
- **Rust:** independent native builder/verifier with pinned stock input, patch preimage/postimage checks and all untouched regions; prior local audit reports included zero unexpected changed bytes. The exact counts of patch windows depend on version and verifier profile and must not be conflated.
- **Claude/AGY:** prior local orchestration included Claude reviews, Python/Rust and AGY adversarial activities. AGY availability and direct review of the **new DNG v2** are **not confirmed**. Historical reports are not substituted for a newly executed independent test.
- **Current session:** direct file SHA-256 confirmation of C3.11 and 04.06, GitHub contents/log audit and isolated container synthetic DNG tests. **The owner's WSL was not rerun in this session.**

## 3. Why our DNG16 and HDR output differ

The experimental Night RAW stack is not the Nikon Z8 ADC's precision, nor a NEF firmware feature. A 14-bit CFA pixel averaged over N equal-exposure frames is stored in an unsigned 16-bit CFA DNG as `round(4 * mean)`: this retains quarter-DN units. **A 16-frame mean also has 1/16-DN increments, so this DNG16 representation is bounded but not lossless relative to the exact mean**. More importantly, exposure-bracket HDR cannot faithfully be represented by pretending its scene-linear range fits into the same fixed integer white level.

Our **[separate public DRAFT DNG v2 research PR](https://github.com/icxnc1/nikon-z8-rust-builder/pull/2)** therefore implements a strict DNG16 writer for ordinary equal-exposure stacking and an experimental **IEEE float32 CFA TIFF/DNG 1.4** reference for exposure-normalized HDR. It checks source SHA-256 and supplied crop/CFA/acquisition metadata, preserves negative values and >1.0 HDR scene-linear values in a synthetic test, and uses atomic finalization in the float32 prototype. The calibrated NEF metadata extractor, alignment/deghost, bad-pixel/dark correction, real-scene measurements, LibRaw/Adobe/NX Studio compatibility, camera RTOS integration and safety **are still open**.

Actual executed DNG v2 tests in the isolated container: strict GCC/Clang compilation; AArch64 freestanding compilation (**not execution**); UBSan; 18 invalid-input negative cases; simulated read/write errors; **330,400 C↔NumPy pixels matched**; independent `tifffile` readback (29 tags); synthetic HDR float32 TIFF readback and 2.7× reference-white value preservation. Source code and tests are in PR #2. No claim of in-camera Night RAW or full HDR implementation is made.

## 4. Collaboration boundary and user safety

The upstream author's project remains Python. This independent Rust application is **not reviewed, supported or maintained by XinmiaoZhao**. Please report builder-specific problems **in [this repository's Issues](https://github.com/icxnc1/nikon-z8-rust-builder/issues)**. A report claiming that an output is upstream-equivalent must include matching firmware version/groups and full SHA-256. No beginner-level flashing help is offered; see [WARNING.md](../WARNING.md). For real Z8 hardware, no experimental Night DNG or C3.12 firmware is approved for flashing.

Standards reference: [Adobe Digital Negative specification and SDK](https://helpx.adobe.com/camera-raw/desktop/dng-and-file-formats/digital-negative.html).
