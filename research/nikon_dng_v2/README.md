# Nikon Z8 Night DNG v2 — independent experimental hardening

**23 November 2026. Offline/synthetic validation only. FLASH_APPROVED=NO.**

This additive prototype does NOT replace or alter NikonFirmwarePatch, firmware,
NEF decoding or the camera runtime. In particular, it is not a working in-camera
Night RAW or HDR feature. `night_dng_v2.c` is a freestanding callback-based writer.
`hdr_float_reference.py` is a separate **Python/tifffile reference**, not an
ARM firmware pipeline.

## Design

- **DNG16 CFA:** equal-exposure, pre-validated Bayer frames, 2–32 images;
  trimmed mean for N>=5; raw values scaled by four to preserve quarter-DN
  fractions, with consistent BlackLevel/WhiteLevel scaled by four. 16-frame
  means can have 1/16-DN fractions: 16-bit x4 therefore **is not lossless**
  with respect to the full 16-frame average; float32 is the candidate when
  preserving those extra fractions matters.
- **DNG metadata:** the writer requires caller-supplied, validated CFA phase,
  black/white, ColorMatrix1, AsShotNeutral, CalibrationIlluminant1 and camera
  model. It never invents a calibrated profile. `metadata_gate.py` verifies
  local manifest SHA-256 and metadata consistency but **does not authenticate
  arbitrary manifest claims or read Nikon maker notes**. A vetted Nikon NEF
  metadata extractor is still required for real-camera use.
- **HDR:** float32 CFA DNG is supported by DNG 1.4 at the format level;
  `hdr_float_reference.py` demonstrates per-exposure black subtraction,
  shutter-ratio normalization, clipping mask, signed residuals, values >1.0,
  and atomic output only after a TIFF readback. No deghost, star alignment,
  bad-pixel correction, highlight reconstruction or Adobe/NX Studio compatibility
  is claimed. All-saturated pixels carry lower bounds and a mask, not invented
  highlight detail. Integer DNG16 output is not used as the HDR container.
- **Safety:** C output is callback-based and may be partial on read/write
  error. Integration MUST write to a private temporary file, validate, then
  atomically rename; on failure delete it. Python reference already follows
  this pattern. No real-camera I/O is implemented.

## Reproduce locally (Python numpy + tifffile, GCC and Clang)

```
mkdir -p /tmp/night-dng-test
cc -std=c11 -Wall -Wextra -Werror -pedantic -O2 night_dng_v2.c tests/test_v2.c -o /tmp/ns2test
/tmp/ns2test /tmp/night-dng-test
python3 tests/check_v2.py /tmp/night-dng-test
cc -std=c11 -fPIC -shared -O2 night_dng_v2.c -o tests/libnight.so
python3 tests/diff_fuzz.py
python3 tests/check_hdr.py /tmp/night-dng-new-directory  # directory must exist and be fresh
clang --target=aarch64-none-elf -ffreestanding -nostdlib -O2 -c night_dng_v2.c -o /tmp/night_dng_v2_arm64.o
```

## Recorded bounded evidence (container environment, not user's WSL)

- GCC 64-bit and Clang strict compilation: PASS.
- Clang AArch64 freestanding object: PASS; ELF AArch64, **not executed in an emulator**.
- UBSan+bounds C harness: 18 invalid-input negative cases PASS; injected read and write failures detected.
- C vs independent Python/NumPy scalar oracle: **330,400 pixel comparisons, zero mismatches** for N=2..32, mean and trim, synthetic samples including boundary values.
- `tifffile` independent DNG16 structural and image readback: **mean + trim, 528 pixels each, 29 tags**, zero mismatches.
- HDR synthetic: input exposures 0.25×, 1×, 4×; scene radiance up to 2.7× reference white survives in IEEE float32 output, negative residual survives. Full TIFF readback equals written pixels. Tampered manifest SHA, crop mismatch and unequal-exposure integer stacking are rejected.
- **Not tested:** actual NEF samples, LibRaw/rawpy, camera firmware, Adobe Camera Raw, Lightroom, NX Studio, GH Actions macOS or actual EXPEED7 timings. Claude/AGY review not yet received.

Read this package's source rather than assuming that a generated DNG is
scientifically calibrated or that it has been validated on the Nikon Z8.
