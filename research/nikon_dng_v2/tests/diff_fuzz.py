# SPDX-License-Identifier: GPL-3.0-or-later
"""Independent Python scalar differential against C v2 Bayer stacking row."""
import ctypes as C, pathlib, sys, os
import numpy as np
root=pathlib.Path(__file__).resolve().parents[1]
class NS2(C.Structure):
    _fields_=[('frames',C.c_uint32),('width',C.c_uint32),('height',C.c_uint32),('mode',C.c_uint32),
              ('black',C.c_uint16*4),('white',C.c_uint16),('cfa',C.c_uint8*4),
              ('color_matrix',C.c_int32*9),('neutral',C.c_uint32*3),('illuminant1',C.c_uint16),('model',C.c_char*32)]
lib=C.CDLL(os.environ.get('NS2_LIB',str(root/'tests/libnight.so')))
lib.ns2_stack_row.argtypes=[C.POINTER(NS2),C.POINTER(C.c_uint16),C.POINTER(C.c_uint16)]
rng=np.random.default_rng(1253)
checks=0
for n in range(2,33):
  for width in (1,3,4,17,255):
    for mode in (0,1):
      if mode==1 and n<5: continue
      for batch in range(20):
        arr=rng.integers(0,16384,size=(n,width),dtype=np.uint16)
        if batch%4==0:arr[0,:]=16383
        if batch%7==0:arr[-1,:]=0
        p=NS2();p.frames=n;p.width=width;p.mode=mode
        out=np.zeros(width,dtype=np.uint16)
        lib.ns2_stack_row(C.byref(p),arr.ctypes.data_as(C.POINTER(C.c_uint16)),
                          out.ctypes.data_as(C.POINTER(C.c_uint16)))
        values=arr.astype(np.uint64)
        sums=values.sum(axis=0)
        if mode: sums-=values.min(axis=0)+values.max(axis=0)
        div=n-2 if mode else n
        ref=((sums*4+div//2)//div).clip(0,65535).astype(np.uint16)
        if not np.array_equal(out,ref):raise AssertionError((n,width,mode,batch,arr,out,ref))
        checks+=width
print('C_PY_DIFF PASS cases=',checks,'zero_mismatches=1')
