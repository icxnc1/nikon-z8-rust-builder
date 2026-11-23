# Nikon Z8 Night RAW / Night HDR — active research, evidence and engineering plan

**Status (2026-11-23): WORK IN PROGRESS, not a camera firmware feature.** The public [Rust firmware builder](README.md) supports pinned Nikon Z8 **original C3.11-based v0.5.1** feature recipes, but **does NOT add Night RAW or Night HDR** to a Nikon camera. No Night firmware candidate is approved for installation. `FLASH_APPROVED=NO`. The [separate Rust offline RAW processor](docs/CLAUDE_REAL_RAW_EVIDENCE_20261123.md) remains private and currently lacks a vendored build dependency; it is **not** bundled as a finished application.

## What we are trying to build — and why

**Night RAW — single-exposure multi-frame accumulation.** Capture a configurable number of separate 14-bit sensor RAW frames at **the same ISO, shutter time, aperture and framing**, initially **4, 8 or 16 frames**, preserve black-level offsets and negative read-noise residuals, reject unstable/hot/clipped samples, measure alignment and motion, then combine scene-linear Bayer samples before demosaic. The proposed deliverable is a better noise-controlled linear image (first on PC, with a DNG/TIFF export), **not** an invented 16/24-bit Nikon sensor mode. Later research may explore camera-resident streaming and NEF output **only if** its writer semantics, memory/latency and safety are independently established.

- **Purpose:** reveal real shadow texture and natural colour in static night architecture, distant landscapes, dim interiors, documentary work, static high-ISO scenes and potentially astrophotography *with correctly aligned stars*. Preserve a plausible black point, avoid patterned purple/green shadows, excessive smoothing and fabricated detail.
- **Ideal limit:** with independent zero-mean random noise and perfectly registered static subjects, averaging `N` frames reduces noise *standard deviation* by `sqrt(N)`. For `N=16`, this is **4×** or **~2 stops of noise-standard-deviation reduction** relative to one frame of the same exposure. It requires **16× capture time** plus readout, and fixed-pattern noise, thermal drift, missed alignment, subject motion and clipping reduce the benefit. It is **not** a free 2-stop expansion of the Z8 sensor's intrinsic dynamic range.

**Night HDR — exposure-bracket fusion.** Combine at least **−2 EV / 0 EV / +2 EV** RAW captures by varying *shutter duration*, keeping ISO/aperture/focus/white balance stable, with exposure-ratio verification, saturation masks, scene-linear normalization, phase-aware alignment and motion/deghost controls. Shorter exposures preserve lamps, headlights and bright windows; longer exposures can improve dark façades, street scenes and architectural interiors. A simple **bracket-only** or **single-frame fallback** is required if registration or motion cannot be safely handled. Research alternatives include short highlight-preserving **0/−2 EV** and shadow-biased **0/+2 EV** two-frame modes. These are proposals, **not implemented menu options**.

## Evidence: real Nikon Z8 files, synthetic calculations and firmware boundaries

| Evidence class | Measured or verified | What it does NOT prove |
|---|---|---|
| **REAL DARK NEF** | Private series `12026NIK`: 16 Nikon Z8 NEF files plus 16 paired JPEGs; **ISO 3200, 4 s, f/4, LENR OFF, High ISO NR Off**. Dark-frame spatial sigma **21.1415 → 5.8516 DN** at 16-frame average (ratio **3.61×**, ~1.85 stops on that noise estimator). | Better detail on a night scene, improved Bayer colour accuracy, real HDR, or any execution in camera |
| **REAL RAW / OFFLINE ORACLE** | Independent `rawpy` cross-check on **1,048,576 pixels (1024×1024 ROI)** from 16 distinct NEFs: **zero pixel-sum mismatches**. ROI robust green sigma **19.5518 → 5.2818 DN** (ratio 3.70×). | Full-frame equivalence, independently confirmed scene registration, a valid NEF output or Nikon firmware integration |
| **OTHER REAL DARK / LENR** | `100TESTE` comprises **103 NEF and 71 JPEG**. A short-exposure ISO12800 12-frame stack had sigma **17.42 DN** with nominal black correction and **24.42 DN** after subtracting an 11-frame master dark: the latter was **worse**. | Proof that LENR is globally ineffective, or that master dark subtraction should always be applied |
| **SYNTHETIC HDR** | Rust/Python differential tests of a Bayer bracket-fusion algorithm showed an approximate **+1.62 EV** deep-shadow noise metric improvement over one 0 EV input in a defined synthetic scene. | Actual HDR performance on a real −2/0/+2 EV Nikon Z8 bracket |
| **QEMU / firmware hooks** | AArch64 tests can execute selected guest code with **stubbed external Nikon functions**. A C3.11-based v0.5.1 experimental image can be rebuilt and checksummed. | A fully booted/emulated Nikon pipeline, a resident Night capture hook, real autofocus/video safety, or a flash-safe Night firmware |

Evidence details and source provenance: [Claude and real RAW cross-audit](docs/CLAUDE_REAL_RAW_EVIDENCE_20261123.md). The quoted tests are local results based on **private inputs**, not independently repeatable from GitHub without consent to release representative samples.

### Important corrections from a fresh Claude adversarial review

- The separate `HOT_DETAIL_12026NIK.json` contains **6,018** under a *stable-hot aggregate metric* (`stable_hot_gt50dn`), **but only 115** pixels are over 50 DN in the **minimum of all 16 frames** (`hot_gt50_in_min_of_16`). These are **different definitions of persistence**. The overlap/repeatability of candidate hot pixels between two halves is about **25%**. A safe defect map must define its temporal criterion, not claim all 6,018 are persistently hot across all 16 exposures.
- The **1,024×1,024 ROI is only ~2.3% of a full 8,280×5,520 RAW array**. The independent verifier's standalone JSON does not itself list all 16 filenames and SHA-256 identities, so direct one-to-one correspondence with the `12026NIK` inventory must be rechecked against the generator's complete private report. Shared ISO/time settings alone are insufficient provenance.
- A historical decoder comparison against a **separate public 14-bit lossless Z8 sample** does not establish lossless-compression mode for private series `100TESTE` and `12026NIK` (`compression=null` in their inventories). No specific lossless decoder patch is claimed to apply to them without a verified compression tag.
- Some tests reported fixed success strings; independent, computed guard/assertion checks are required before raising any release gate.

## Engineering approach and test gates

**Stage 1 — offline reference on real data (partly complete):** retain every NEF read-only with private SHA-256; establish per-CFA black/white levels, active crop origin, noise vs ISO/exposure, hot-pixel consistency, sensor drift and nonzero Bayer residuals. Process on PC with a bounded Rust pipeline, independently cross-checking raw samples with LibRaw/rawpy or another decoder. Do not assume optical-black margins exist in NEF. Restore and test the separate `z8_deep_raw` Rust crate's missing `vendor/rawler` first.

**Stage 2 — image-quality validation (OPEN):** test matched *real* illuminated scenes with identical framing/exposure, static detail plus controlled moving cars/pedestrians; compare single 0 EV vs 4/8/16-frame output with actual image/noise, SNR, colour, star/line detail, ghost rate, clipping, FPN and time budget. Include a real −2/0/+2 EV HDR bracket; synthetic HDR metrics are not a replacement. Evaluate against untouched original NEFs and conventional editors. Require explicit fallback to ordinary single-frame RAW.

**Stage 3 — bounded in-camera feasibility (OPEN):** 16 fully decoded 8280×5520 uint16 frames require about **1.46 GB** before workspaces. Determine whether a real firmware decode/encoder path can stream frames and write valid metadata with bounded RAM, read ordering, card I/O, cancellation, thermal limits and atomic file write. A row is 16,560 bytes at uint16, but sequential Nikon compressed decoding and storage behavior must be proven; row memory arithmetic alone is insufficient. Verify correct Bayer phase for odd crop offsets and reject moving/saturated samples with measured criteria.

**Stage 4 — version-specific hardware integration (NOT DEMONSTRATED):** Nikon firmware C3.10/C3.11/C3.12 executable addresses and data layouts cannot be freely interchanged. Reconstruct true CFG/ABI and camera APIs with emulator evidence. Compare new code against Nikon OEM behavior without disabling protections. Tests on a real camera would require a separate, deliberate safety decision; **no firmware flash is authorized by this research plan**.

### Camera data acquisition protocol — for reproducible tests

Use **NEF RAW, 14-bit lossless-compressed when the Nikon menu offers it**, manual `M`, fixed ISO (Auto ISO OFF), fixed white balance (e.g. Daylight), Active D-Lighting OFF, tripod/remote or 2s self-timer, LENR OFF and High ISO NR Off by default. For darks, cap the lens and prevent light leak; for scene brackets, remove cap, switch to manual focus, turn VR off on tripod and keep aperture constant. Note room temperature, session and elapsed time. Confirm compression independently from actual metadata; camera menu selection alone is not decoded proof.

| Series / priority | Recommended frames | Current evidence |
|---|---|---|
| **A, read-noise baseline** | ISO64/100/200/400/500/800/1600/3200/6400/12800/25600 and explicitly supported extended ISOs; shutter **1/250 s and 1 s**; **at least 2 independent frames per ISO/time** | `100TESTE` has many **single-frame** low-ISO cells; these still need repeated captures. ISO 400 vs 500 must be tested with replicated dark/flat pairs before concluding anything about dual gain. |
| **A+, Night darks** | Capture **16 before and 16 after** the real scene at the *same ISO and exposure* (e.g. ISO3200/4s, or ISO800/15s), to detect thermal drift. Record actual local conditions; compare master-dark subtraction against no-master baseline. | Existing `12026NIK`: **16 real 4s/ISO3200 darks already collected**; they are a valuable baseline, not automatic evidence of calibration before/after a scene. |
| **B, Nikon Long Exposure NR** | ISO800/6400/25600, shutter **1s, 4s, 30s**, at least 2 LENR OFF + 2 LENR ON each (observe any additional dark-capture processing time). | Historical 4s and 30s ON/OFF pairs already exist; **1s LENR ON missing**. The Z8 uses an electronic shutter; do not call its dark reference a *mechanical shutter* exposure. |
| **C, High ISO NR** | ISO6400/25600 and one explicitly named high extended-ISO setting, 1/250 s, 2 High ISO NR OFF + 2 High each. | These categories have existing pairs; review EXIF and pixel statistics before requesting replacements. |
| **D, real Night scene and HDR** | Repeat a **fixed static scene** for single / 4 / 8 / 16 frames, and a separate **−2/0/+2 EV** bracket varying only shutter. Include controlled motion and saturated lamps. | **Not yet validated as an in-camera or real-scene output**. |

Nikon Z8 has **no mechanical shutter**, so any discussion of LENR must describe camera-controlled reference acquisition without assuming shutter curtains. The usable shadow gain must be reported against **equivalent total exposure time and motion risk**, not as newly created sensor photonic information.

## Collaborate on concrete technical questions

We request upstream feedback on: (1) Nikon C3.11 RAW-buffer lifetime and metadata-consistent encoder integration; (2) whether a bounded streaming path could support 4–16 frames without disabling OEM safety; (3) version-specific C3.12 relocations; (4) a lossless RAW/DNG deliverable that never mislabels synthetic/composited samples as single original Nikon sensor RAW; and (5) objective dark/hot/thermal camera testing.

Original upstream [XinmiaoZhao/NikonFirmwarePatch](https://github.com/XinmiaoZhao/NikonFirmwarePatch) and [collaboration Issue #63](https://github.com/XinmiaoZhao/NikonFirmwarePatch/issues/63). The released application remains **tool-only**, without Nikon firmware or secret patch payloads.
