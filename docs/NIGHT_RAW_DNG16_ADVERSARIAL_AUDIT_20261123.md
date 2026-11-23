# Nikon Z8 Night RAW — 16-bit CFA DNG validation update (2026-11-23)

**Research status: LOCAL_DETERMINISTIC / emulator-only; FLASH_APPROVED=NO.**

This is a source-and-evidence research update, not a firmware release. The [public Nikon Z8 Rust builder](https://github.com/icxnc1/nikon-z8-rust-builder) is still a tool-only C3.11-based v0.5.1 builder; **Night RAW is not an installable Nikon Z8 feature**. No Nikon firmware or personally captured NEFs are redistributed.

## Verified progress

An experimental row-streaming Night RAW prototype combines Bayer-domain samples from compatible, same-framing Nikon Z8 NEFs and writes an uncompressed **16-bit integer CFA DNG**. Its output represents the 14-bit source values scaled by four, retaining up to two fractional bits of an integer multi-frame average. This does not change the sensor's 14-bit acquisition precision.

**Tests rerun on 23 November 2026:**

| Check | Result | Limits |
| --- | --- | --- |
| C11 adversarial DNG-writer test executable | **PASS** (reported 16 checks) | Parameter, I/O, TIFF tag, integer bounds |
| AArch64 freestanding Clang compilation | **PASS** | Compiles source; does not establish camera compatibility |
| Two real Z8 NEFs to 8280 × 5520 16-bit CFA DNG | **PASS** | Offline host only |
| LibRaw/rawpy full-frame readback versus per-pixel mean | **0 mismatches of 45,705,600 pixels** | Two frames, matching Bayer geometry |
| DNG black/white tags | **4032 / 65532** | Correct for these particular test samples |
| Output SHA-256 | `f5419c0b03f3caf0aa56d5988c892171e1f7857ebf6661170e7677eb8527ccdc` | Test DNG remains local and private |

Previous experiments also exercised 16-frame dark-image processing and independent AArch64/Unicorn code execution. They are **not** evidence of a working in-camera 16-exposure Night RAW sequence.

## Adversarial hardening implemented locally

- Reject invalid frame counts (outside 2–32), invalid dimensions/CFA codes, inconsistent black/white levels and integer overflow in scratch-memory and TIFF image offsets.
- Reject NEF input sets with differing dimensions or Bayer phase; validate guest-side frame count and essential pointers before accessing fixed arrays.
- Reject unsupported modes and failed input allocations/reads/writes; remove incomplete host DNG output on processing or close failure.
- Correct TIFF **Software tag 305** from 16 bytes to **17 bytes** (include required terminating NUL); add a tag-structure test.
- Recompile and rerun the complete two-real-NEF pixel comparison after the TIFF correction.

These fixes and tests are **local research changes**, not yet merged into the private integration PR or included in a public binary. Public results are limited to this documented evidence; samples and private source code are not supplied, so external reproduction needs independent test inputs.

## Still blocked — no release or camera flash

1. **Source-specific DNG metadata:** prototype `BlackLevel`, `WhiteLevel`, `ColorMatrix1` and `AsShotNeutral` are still fixed or approximate. Parse and validate NEF metadata, calibration illuminant, active crop/CFA phase, exposure/ISO and per-frame consistency.
2. **Real N-frame capture:** the C3.12 R13 emulator experiment validates a seeded `k=2` one-frame transformation, not persistence and averaging across 16 independently captured exposures.
3. **Image quality / HDR:** real illuminated 4/8/16-frame scenes, alignment, motion/ghost rejection, clipping, per-ISO noise calibration and real -2/0/+2 EV brackets are not yet validated.
4. **Camera resources:** firmware menu integration, asynchronous card I/O, allocation lifetimes, cancellation, thermal limits and hardware safety remain unproven.
5. **External readers:** validate full DNG colour metadata and compatibility with an additional DNG parser/editor, not only LibRaw.

**Independent reviewers:** Copilot was invoked but reviewed an older Night RAW pathway. AGY was invoked repeatedly, but permission/timeout errors produced no usable review (**UNAVAILABLE**, not PASS). Claude received two adversarial review reports in the private integration PR. No model agreement or green synthetic test is a hardware safety proof.

## Upstream and collaboration

We follow [XinmiaoZhao/NikonFirmwarePatch](https://github.com/XinmiaoZhao/NikonFirmwarePatch), latest verified v0.5.1 (C3.11 input), and discuss engineering questions in [upstream issue #63](https://github.com/XinmiaoZhao/NikonFirmwarePatch/issues/63). C3.12 support in upstream is not announced. Further work is tracked in this repository's [research roadmap](https://github.com/icxnc1/nikon-z8-rust-builder/blob/main/RESEARCH.md).

**No original/modified Nikon firmware, proprietary patch payload, memory dump, private photo, API credential or flash-ready image is included.**
