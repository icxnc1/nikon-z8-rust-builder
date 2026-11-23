# SPDX-License-Identifier: GPL-3.0-or-later
"""Independent TIFF/DNG metadata and full-data oracle; no reliance on C writer internals."""
import pathlib, sys
import numpy as np
import tifffile
root = pathlib.Path(sys.argv[1])
for mode in ('mean','trim'):
    path=root / f'{mode}.dng'
    with tifffile.TiffFile(path) as t:
        page=t.pages[0]
        tags=page.tags
        assert page.shape==(11,48) and page.dtype==np.dtype('uint16')
        assert page.photometric==32803, page.photometric
        assert tuple(tags[50706].value)==(1,4,0,0)
        assert tuple(tags[50707].value)==(1,4,0,0)
        assert tags[50717].value==65532
        assert tuple(tags[50714].value)==(4032,)*4
        assert tags[50778].value==21
        assert tags[305].count==17 and str(tags[305].value)=='z8 night stack 1'
        assert 'DERIVED CFA' in str(tags[270].value)
        assert len(tags)==29,len(tags)
        assert tuple(tags[50829].value)==(0,0,11,48)
        values = np.stack([np.fromfunction(lambda y,x:1008+f*3+y*19+x*7,(11,48),dtype=np.int64)
                           for f in range(16)]).astype(np.uint32)
        if mode=='mean':
            expected=(values.sum(axis=0)*4+8)//16
        else:
            expected=((values.sum(axis=0)-values.min(axis=0)-values.max(axis=0))*4+7)//14
        got=page.asarray()
        assert np.array_equal(got,expected.astype('<u2'))
        assert path.stat().st_size==page.dataoffsets[0]+page.databytecounts[0]
        print(f'PY_ORACLE PASS {mode} pixels={got.size} all_tags={len(tags)}')
