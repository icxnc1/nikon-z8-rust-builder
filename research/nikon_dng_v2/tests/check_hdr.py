# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib, json, pathlib, tempfile, sys
import numpy as np
import tifffile
sys.path.insert(0,str(pathlib.Path(__file__).resolve().parents[1]))
from hdr_float_reference import fuse,write_dng_float32
from metadata_gate import check_manifest
r=pathlib.Path(sys.argv[1])
cal={'color_matrix1':[11000,0,0,0,10000,0,0,0,9500],
     'as_shot_neutral':[0.54,1.0,0.7], 'illuminant1':21,
     'source':'SYNTHETIC TEST PROFILE; not Nikon measured','model':'Nikon Z 8'}
scene=np.array([[-0.004, 0.0, 0.03, 0.4], [0.7, 1.0, 2.0, 2.7]],dtype=np.float64)
exposures=[250000,1000000,4000000]
white=16383;black=1008
raws=[]; files=[]
for i,t in enumerate(exposures):
    raw=np.rint(scene*(t/1000000)*(white-black)+black).clip(0,white).astype('<u2')
    raws.append(raw)
    path=r/f'input{i}.raw'; path.write_bytes(raw.tobytes());files.append(path)
raws=np.stack(raws)
metadata={'calibration':cal,'frames':[]}
for i,p in enumerate(files):
    metadata['frames'].append(dict(path=p.name,sha256=hashlib.sha256(p.read_bytes()).hexdigest(),
                width=4,height=2,cfa=[0,1,1,2],origin=[0,0],black=[black]*4,
                white=white,iso=800,aperture_x1000=4000,exposure_us=exposures[i]))
assert check_manifest(metadata,kind='hdr',base_dir=r)['frame_count']==3
try:
    check_manifest(metadata,kind='stack',base_dir=r)
    raise AssertionError('mixed exposure stack unexpectedly passed')
except ValueError: pass
m=metadata.copy(); m={'calibration':cal,'frames':[dict(x) for x in metadata['frames']]}
m['frames'][1]['sha256']='0'*64
try:
    check_manifest(m,kind='hdr',base_dir=r)
    raise AssertionError('tampered SHA passed')
except ValueError: pass
m={'calibration':cal,'frames':[dict(x) for x in metadata['frames']]}
m['frames'][1]['origin']=[1,0]
try:
    check_manifest(m,kind='hdr',base_dir=r)
    raise AssertionError('crop phase mismatch passed')
except ValueError: pass
out,sat=fuse(raws,exposures,[black]*3,[white]*3,1000000)
assert out.max()>2.5 and out.min()<0 and sat.sum()==0, (out,sat)
# Pure scalar oracle, independent of vectorized fuse implementation.
for y in range(2):
  for x in range(4):
    ss=ww=0.
    for k,t in enumerate(exposures):
      raw=int(raws[k,y,x]);ratio=t/1000000
      if raw<white-2:
        w=ratio**2;ss+=w*(raw-black)/(white-black)/ratio;ww+=w
    exp=ss/ww
    assert abs(float(out[y,x])-exp)<1e-6
path=r/'hdr_float.dng';write_dng_float32(path,out,cal)
with tifffile.TiffFile(path) as tif:
    pg=tif.pages[0]
    assert pg.photometric==32803 and pg.dtype==np.dtype('float32')
    assert pg.tags[339].value==3
    assert tuple(pg.tags[50706].value)==(1,4,0,0)
    assert pg.tags[50778].value==21
    assert np.array_equal(pg.asarray(),out)
print('HDR_SYNTHETIC PASS',{'max_radiance':float(out.max()),'negative_preserved':bool(out.min()<0),
    'float_tiff_roundtrip':True,'tag_count':len(pg.tags),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
