# SPDX-License-Identifier: GPL-3.0-or-later
"""EXPERIMENTAL float32 CFA DNG / synthetic HDR validation reference, NOT in-camera.

The result is scene-linear *relative* irradiance at a selected reference
exposure.  DNG reader compatibility is NOT established by TIFF parsing alone.
"""
import os
import pathlib
import tempfile
import numpy as np
import tifffile


def fuse(raws, exposures, black, white, reference_us, *, saturation_margin=2):
    """Return radiance float32 and a saturation lower-bound mask; no alignment.

    `raws`: shape (N,H,W) uint16, `exposures`: microseconds per frame.
    black and white: matching frame and CFA plane values already validated.
    Reject mismatched or unsafe exposure representations before arithmetic.
    """
    a=np.asarray(raws)
    exp=np.asarray(exposures, dtype=np.float64)
    if a.ndim!=3 or a.shape[0]<2 or a.shape[0]>32 or a.dtype!=np.uint16:
        raise ValueError('raws must be (N,H,W) uint16')
    if exp.shape!=(a.shape[0],) or not np.all(np.isfinite(exp)) or np.any(exp<=0):
        raise ValueError('positive exposures required')
    if not np.isfinite(reference_us) or reference_us<=0:
        raise ValueError('positive reference_us required')
    black=np.asarray(black, dtype=np.float64)
    white=np.asarray(white, dtype=np.float64)
    if black.shape!=(a.shape[0],) or white.shape!=(a.shape[0],):
        raise ValueError('per-frame black/white required')
    if not np.all(np.isfinite(black)) or not np.all(np.isfinite(white)) or np.any(white<=black):
        raise ValueError('invalid black/white')
    if np.any(a>white[:,None,None]):
        raise ValueError('source sample exceeds declared white')
    ratio=exp/float(reference_us)
    linear=(a.astype(np.float64)-black[:,None,None])/(white-black)[:,None,None]/ratio[:,None,None]
    good=(a.astype(np.float64)<(white-saturation_margin)[:,None,None]);
    weights=np.where(good,ratio[:,None,None]**2,0.0)
    denom=weights.sum(axis=0)
    numerator=(linear*weights).sum(axis=0)
    # If every frame is saturated, the value is an unresolved LOWER BOUND.
    # Prefer the smallest exposure, whose saturation threshold gives highest bound.
    lower=linear[np.argmin(exp)]
    output=np.where(denom>0,numerator/np.where(denom>0,denom,1),lower)
    return output.astype('<f4'), denom==0


def write_dng_float32(path, array, cal, *, white_radiance, cfa=(0,1,1,2)):
    """Atomic on success; prototype metadata is explicitly declared external."""
    path=pathlib.Path(path)
    a=np.asarray(array)
    if a.ndim!=2 or a.dtype!=np.dtype('<f4') or not np.all(np.isfinite(a)):
        raise ValueError('need finite 2D float32 mosaic')
    if not np.isfinite(white_radiance) or white_radiance < 1.0 or np.max(a) > white_radiance:
        raise ValueError('HDR white_radiance must represent the measurable clipping ceiling')
    if path.exists():
        raise FileExistsError(path)
    if sorted(cfa)!=[0,1,1,2]:
        raise ValueError('invalid CFA')
    cm=cal['color_matrix1']
    if len(cm)!=9 or not any(cm) or not cal.get('source'):
        raise ValueError('calibrated matrix provenance required')
    neutral=cal['as_shot_neutral']
    if len(neutral)!=3 or any(x<=0 for x in neutral):
        raise ValueError('invalid neutral')
    if not cal.get('illuminant1'):
        raise ValueError('illumination unknown')
    # These DNG tags cannot make a wrong camera matrix correct; caller must validate.
    extratags=[
        (33421,'H',2,(2,2),False),
        (33422,'B',4,tuple(cfa),False),
        (50706,'B',4,(1,4,0,0),False),
        (50707,'B',4,(1,4,0,0),False),
        (50708,'s',0,str(cal['model']),False),
        (50713,'H',2,(2,2),False),
        (50714,'f',1,0.0,False),
        (50717,'f',1,float(white_radiance),False),
        (50721,'2i',9,tuple((int(x),10000) for x in cm),False),
        (50728,'2I',3,tuple((int(round(float(x)*1000000)),1000000) for x in neutral),False),
        (50778,'H',1,int(cal['illuminant1']),False),
        (50829,'I',4,(0,0,a.shape[0],a.shape[1]),False),
    ]
    fd, temp=tempfile.mkstemp(prefix='.night-hdr-', suffix='.tmp', dir=path.parent)
    os.close(fd)
    try:
        tifffile.imwrite(temp, a, photometric=32803, compression=None, metadata=None,
                         description='DERIVED MULTI-EXPOSURE CFA; EXPERIMENTAL HDR, not sensor RAW',
                         extratags=extratags, software='Nikon Z8 HDR research')
        with open(temp,'rb') as f:
            os.fsync(f.fileno())
        # Independently read/inspect TIFF bytes prior to exposing output.
        with tifffile.TiffFile(temp) as t:
            p=t.pages[0]
            if p.dtype!=np.dtype('float32') or p.photometric!=32803 or not np.array_equal(p.asarray(),a):
                raise ValueError('HDR TIFF readback mismatch')
            if p.tags[339].value!=3:
                raise ValueError('not IEEE float TIFF')
        os.replace(temp,path)
    finally:
        if os.path.exists(temp):os.unlink(temp)
