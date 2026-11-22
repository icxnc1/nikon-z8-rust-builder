# Nikon Z8 research proposal: Night RAW and Night HDR

**Important: research goals, NOT completed Z8 firmware functions.** The existing Rust builder and QEMU emulator hooks are enabling infrastructure only.

## Night RAW — improve low-light detail while preserving RAW latitude

Goal: capture and align a sequence of RAW frames (candidate mode: 16 frames). Merge scene-linear sensor data before final color transformation. Handle per-frame black level, clipping, bad pixels, frame alignment, motion masks and metadata. Favor bounded row/strip processing to avoid excessive camera memory use.

Under the idealized assumption of independent zero-mean noise and a static scene, combining N equal-exposure frames reduces noise standard deviation by sqrt(N); for **16 frames this is a 4x SNR ratio, equivalent to about 2 stops of noise-performance improvement**. Real gains may be smaller because of correlated noise, motion, spatial alignment and sensor constraints. No 16-frame Night RAW mode has been proven to execute in-camera.

Intended situations: night cityscapes, low-light static architecture, dim interiors, tripod landscape photography, long-term low-light documentation and stationary subjects at high ISO. Stars require star registration/tracking. For moving people/vehicles, multi-frame motion artifacts require reliable deghosting or fallback to standard single-frame RAW.

## Night HDR — preserve bright lights and dark shadows

Goal: acquire and merge exposure-bracketed RAW frames, for example **-2 EV, 0 EV, +2 EV**, maintaining highlight detail from short exposure, shadow information from long exposure and midtones from normal exposure. Employ exposure alignment, saturation masks, moving-subject deghosting, accurate linear color handling and preserve useful RAW/EXIF data.

Intended situations: dark streets with headlights and lamps, night architecture and illuminated monuments, window/interior contrasts, city skylines, tunnel entrances/exits, concerts/stages with bright spotlights and very dark backgrounds. Motion and hand shake are major constraints. Aim for operator-selectable modes including bracket-only fallback, rather than automatic aggressive HDR.

## Technical commitments

- Do not claim native **16-bit/24-bit ADC or native RAW data** merely by increasing container bit depth; the sensor/ADC must be characterized.
- Keep normal autofocus, N-RAW 3:2, Nikon menu behavior, thermal limits, card I/O and safety constraints unchanged unless independently validated.
- Separate firmware C3.10/C3.11/C3.12 binary profiles; addresses/relocations are version-specific. Current validated native Rust recipe set targets **original C3.11 only**.
- Build reproducible offline tests (synthetic & real RAW) with explicit SNR/DR, hot-pixel, clipping, color, motion and ghost metrics; Python reference implementation may independently cross-check Rust.
- Distinguish finite QEMU instruction tests, mocked OEM dependencies, source-level reasoning and physically verified camera behavior.
- No unattended flashing. No firmware release based only on CRC/SHA, emulator stubs or agreement between AI tools.

Feedback from the upstream author on feasibility, binary compatibility, memory, exposure controls and Night RAW/HDR integration is welcome.
