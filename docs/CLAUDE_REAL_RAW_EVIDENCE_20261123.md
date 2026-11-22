# Claude / Nikon Z8 real RAW evidence reconciliation — 2026-11-23

**Status:** sanitized summary of private real-camera source data and Claude source reviews. These observations are not independently reproducible from this public repository. **FLASH_APPROVED=NO.** No original NEF/JPEG, unredacted metadata, source firmware, patch payload or camera memory dumps are included.

## Real photographs and settings

Private evidence: development/z8_deep_raw/samples/DECIZII_100TESTE_20261122.md, INVENTORY_100TESTE.json, ANALYSIS_100TESTE.json and CURATION_100TESTE.json.

- Series **100TESTE** contains **103 NEF and 71 JPEG**. The 103 NEF were curated into **61 dark**, **26 long-exposure-NR** and **16 high-ISO-NR** groups. All **174 private files** were checked for SHA-256 identity between originals and local curated copies.
- Ambient room temperature **23–24 °C** is user-reported; no direct sensor temperature was measured.
- The dataset inventory does **not** establish NEF compression (compression=null). Apparent ISO400/500 differences come from single-image groups and do not establish a dual-gain transition.
- An ISO12800, 1/125-second dataset had single-frame spatial sigma **57.82 DN**, sigma after a 12-frame average with nominal black correction **17.42 DN**, and sigma after subtracting an 11-frame master dark **24.42 DN**. **Blind master-dark subtraction was worse in this short-exposure case.**

An additional private series **12026NIK** contains **16 NEF plus 16 paired JPEG**. Its camera inventory records **Nikon Z8, ISO 3200, 4 seconds, f/4, Long Exposure NR OFF, High ISO NR Off, RAW quality**. These are **dark/noise characterization images**, not real bracketed HDR scenery. The actual NEF compression tag was not established.

Private ANALYSIS_12026NIK.json and HOT_DETAIL_12026NIK.json report:
- Single-frame spatial sigma: **21.1415 DN**.
- Sixteen-frame average spatial sigma: **5.8516 DN** versus theoretical independent-noise prediction **5.2854 DN**.
- Ratio: **3.61×**, approximately **1.85 stops of noise-standard-deviation reduction** in dark frames; this is **not** proven scene dynamic-range gain.
- Median temporal noise sigma: **20.35 DN**; spatial pixel FPN sigma: **3.58 DN**; stable hot pixels above 50 DN: **6,018**.
- A separate 8 + 8-frame experiment found noise sigma **8.12 → 10.79 DN** after subtracting another 8-frame master dark. This again warns against unconditional master subtraction.

## Real 16-frame signed RAW arithmetic verification

Private source: development/Z8_0408_WORK/evidence/NIGHT16_REAL_VERIFY.json; independent verification implementation: development/Z8_0408_WORK/tools/verify_night16_real.py.

The local offline calculation took **16 distinct real Nikon Z8 NEFs**, ISO3200 at 4s. An independent rawpy oracle checked the signed-Q8 Bayer accumulation in one **1024×1024 ROI**: **1,048,576 pixels compared, zero mismatches**. Within that ROI, robust green noise sigma was **19.5518 → 5.2818 DN** (ratio 3.70×, about 1.89 stops on that noise statistic).

Crucial boundaries: the generated output is a **TIFF/int32 mosaic, not a Nikon NEF**; no actual motion alignment was proven; **no master dark was used**; no in-camera Night RAW functionality exists in this evidence. A 16-frame offline arithmetic cross-check is not proof of firmware integration or camera safety.

## Separate Rust offline RAW processor

Private project development/z8_deep_raw contains separate Rust modules for NEF decode, per-CFA black correction, wavelet denoising before demosaic, RCD demosaic, float32 DNG, hot-pixel/master-dark processing and Bayer-domain HDR. This **is not the same application as the public Nikon firmware patch builder**.

Its documentation records an older local **45,705,600-sample NEF decode comparison** with patched rawler 0.8.0 versus LibRaw/rawpy and a local rawler identity-curve correction avoiding an approximately **−0.75 DN** negative bias in the affected code path (private vendor/PATCHES.md and tests/evidence/XCHECK_DECODE.json).

Its HDR gains of **+1.62 EV** in deep shadows are from **synthetic scene/noise data**, not measured real-scene HDR. Full-resolution three-frame HDR smoke performance was reported at **4.7s merge / 12.9s total / 1.47 GB peak RSS**, on **2 PC x86 vCPUs**, not Nikon EXPEED7.

**Build blocker found on 2026-11-23:** its Cargo.toml pins a patched dependency at vendor/rawler, yet **vendor/rawler is missing**. Do not claim that the separate RAW processor currently rebuilds or is a published executable until its LGPL-licensed dependency and patch are restored and tests rerun. The two applications also use different licenses and need a license review before any code merger.

## Claude's adversarial review: open engineering risks

Private reports: development/night_raw_stack/CLAUDE_NIGHT_REVIEW.txt, development/nikon_autopilot/docs/reviews/R11_CLAUDE_READONLY.txt, development/nikon_collab/THERMAL_AUDIT_2026-11-20/claude_thermal.txt.

1. **CFA crop origin and odd-pixel offsets** can break phase-specific black corrections. Do not assume Bayer phase from array parity without verified crop coordinates.
2. Registration and motion-free booleans were sometimes **declared by callers**, not measured. Moving subjects and subpixel shifts require an independent registration/deghost oracle.
3. Naive averaging is insufficient for saturated lamps, permanent hot pixels and fixed-pattern noise. Use explicit saturation masks and robust rejection/fallback.
4. Interrupted reads/writes need fail-closed behavior, temporary files and atomic final commit; never allow a partially written Night result to appear valid.
5. Sixteen fully decoded Z8 16-bit-per-sample arrays would use roughly **1.46 GB** before intermediates. A practical in-camera path needs a real bounded-memory decoder/streaming proof.
6. Several prior positive test strings were hardcoded instead of computed checks, making the corresponding assertions weaker than independent evidence.
7. No absolute Nikon sensor temperature/headroom was established. Thermal controls must honor native OEM warning/cutout and should not invent thresholds or bypass protections.

These are **Claude read-only source-review findings**, not physical Nikon validation.

## Integration priorities

- Keep all private NEF/JPEG and sensitive EXIF metadata out of public GitHub. Only sanitized aggregate measurements are authorized here.
- Restore and retest the separate Rust RAW processor's patched rawler vendor before offering its source/build as a validated second application.
- Acquire real multi-ISO and 4–30-second dark groups, flat fields, and real −2/0/+2 EV bracketed Night HDR scenes, with measured temperature and motion.
- Compare real-scene output against original NEFs, Nikon NX Studio / other raw processors, including ghosts, saturation, color errors, shadow artifacts, temporal drift and long-run thermals.
- Keep evidence classes distinct: **real dark-frame offline processing; synthetic HDR; QEMU with simulated OEM; Claude source review; actual camera firmware behavior**.
- No Night RAW/HDR camera firmware release is claimed. No flashing is authorized by this report.

[Research roadmap](../RESEARCH.md) | [NikonFirmwarePatch upstream](https://github.com/XinmiaoZhao/NikonFirmwarePatch) | [Public Rust Builder](../README.md)
