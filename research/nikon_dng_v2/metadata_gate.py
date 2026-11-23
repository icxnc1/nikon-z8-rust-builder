# SPDX-License-Identifier: GPL-3.0-or-later
"""Fail-closed metadata preflight for same-exposure Bayer stacking / unequal-exposure HDR.

This validates a separately produced, source-pinned manifest. It does NOT extract
Nikon maker notes or independently authenticate arbitrary supplied metadata.
Require a separately audited extraction/decoder before accepting real NEFs.
"""
import argparse
import hashlib
import json
import pathlib


def _digest(path):
    h = hashlib.sha256()
    with pathlib.Path(path).open('rb') as f:
        for b in iter(lambda: f.read(1 << 20), b''):
            h.update(b)
    return h.hexdigest()


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def check_manifest(manifest, *, kind, base_dir=pathlib.Path('.')):
    require(kind in ('stack', 'hdr'), 'unknown mode')
    require(isinstance(manifest, dict), 'invalid manifest')
    frames = manifest.get('frames')
    cal = manifest.get('calibration')
    require(isinstance(frames, list) and 2 <= len(frames) <= 32, 'frames must be 2..32')
    require(isinstance(cal, dict) and cal.get('source') and cal.get('model'), 'provenance required')
    cm = cal.get('color_matrix1')
    require(isinstance(cm, list) and len(cm) == 9 and all(isinstance(v, int) and abs(v) <= 1000000 for v in cm)
            and any(cm), 'calibrated ColorMatrix1 required')
    require(isinstance(cal.get('illuminant1'), int) and cal['illuminant1'] > 0,
            'CalibrationIlluminant1 required')
    neutral = cal.get('as_shot_neutral')
    require(isinstance(neutral, list) and len(neutral) == 3 and all(isinstance(n, (int, float)) and
            0 < n < 20 for n in neutral), 'AsShotNeutral required')
    common = None
    exposures = []
    seen = set()
    for i, frame in enumerate(frames):
        require(isinstance(frame, dict), f'frame {i}: must be object')
        require(isinstance(frame.get('path'), str) and frame['path'], f'frame {i}: path required')
        path = base_dir / frame['path']
        require(path.is_file(), f'frame {i}: source missing')
        sha = frame.get('sha256')
        require(isinstance(sha, str) and len(sha) == 64 and _digest(path) == sha.lower(),
                f'frame {i}: SHA-256 mismatch')
        require(path.resolve() not in seen, f'frame {i}: duplicate file')
        seen.add(path.resolve())
        shape = (frame.get('width'), frame.get('height'), tuple(frame.get('cfa', [])),
                 tuple(frame.get('origin', [])), tuple(frame.get('black', [])), frame.get('white'),
                 frame.get('iso'), frame.get('aperture_x1000'))
        w, h, cfa, origin, black, white, iso, aperture = shape
        require(isinstance(w, int) and isinstance(h, int) and 0 < w <= 100000 and 0 < h <= 100000,
                f'frame {i}: invalid dimensions')
        require(len(cfa) == 4 and sorted(cfa) == [0, 1, 1, 2], f'frame {i}: invalid CFA')
        require(len(origin) == 2 and all(isinstance(v, int) and v >= 0 for v in origin),
                f'frame {i}: crop origin unavailable')
        require(len(black) == 4 and all(isinstance(v, int) and 0 <= v < 16384 for v in black),
                f'frame {i}: black level unavailable')
        require(isinstance(white, int) and max(black) < white <= 16383,
                f'frame {i}: white level unavailable')
        require(isinstance(iso, int) and iso > 0 and isinstance(aperture, int) and aperture > 0,
                f'frame {i}: ISO and aperture required')
        microseconds = frame.get('exposure_us')
        require(isinstance(microseconds, int) and microseconds > 0,
                f'frame {i}: exposure_us required')
        exposures.append(microseconds)
        if common is None:
            common = shape
        require(common == shape, f'frame {i}: phase, crop, CFA or acquisition metadata differs')
    if kind == 'stack':
        require(len(set(exposures)) == 1, 'stack requires equal exposure times')
    else:
        require(len(set(exposures)) > 1, 'HDR requires distinct exposure times')
    return dict(frame_count=len(frames), width=common[0], height=common[1],
                exposures_us=exposures, type=kind, calibration_source=cal['source'])


def main():
    a=argparse.ArgumentParser()
    a.add_argument('manifest', type=pathlib.Path)
    a.add_argument('--mode', required=True, choices=('stack','hdr'))
    args=a.parse_args()
    info=check_manifest(json.loads(args.manifest.read_text()), kind=args.mode,
                        base_dir=args.manifest.parent)
    print(json.dumps({'gate':'PASS', **info}, sort_keys=True))


if __name__ == '__main__':
    main()
