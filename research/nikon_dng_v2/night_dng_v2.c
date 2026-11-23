/* SPDX-License-Identifier: GPL-3.0-or-later */
#include "night_dng_v2.h"
#include <limits.h>

#define NS2_NTAGS 29u
#define NS2_IFD 8u
#define NS2_EXTRA (NS2_IFD + 2u + NS2_NTAGS*12u + 4u)
#define NS2_EXTRA_CAP 288u
#define NS2_HEADER ((NS2_EXTRA + NS2_EXTRA_CAP + 63u) & ~63u)
#define NS2_DESC_LEN 44u
#define NS2_SOFT_LEN 17u
_Static_assert(NS2_HEADER < 2048, "header budget");
_Static_assert(NS2_EXTRA + NS2_EXTRA_CAP <= NS2_HEADER, "header overlap");

uint32_t ns2_header_bytes(void) { return NS2_HEADER; }
size_t ns2_scratch_bytes(const ns2_params *p) {
    if (!p || p->frames < 2 || p->frames > NS2_MAX_FRAMES || !p->width) return 0;
    uint64_t n = ((uint64_t)p->frames + 1u) * p->width * 2u;
    return n > SIZE_MAX ? 0 : (size_t)n;
}
static int illum_valid(uint16_t v) {
    switch(v) {
        case 1:case 2:case 3:case 4:case 9:case 10:case 11:case 12:case 13:
        case 14:case 15:case 17:case 18:case 19:case 20:case 21:case 22:
        case 23:case 24:case 255:return 1;
        default:return 0;
    }
}
static int bayer(const uint8_t *a) {
    unsigned r=0,g=0,b=0;
    for (unsigned i=0;i<4;i++) { if(a[i]==0)r++;else if(a[i]==1)g++;else if(a[i]==2)b++;else return 0; }
    return r==1 && g==2 && b==1;
}
int ns2_validate(const ns2_params *p) {
    if (!p || p->frames < 2 || p->frames > NS2_MAX_FRAMES || !p->width || !p->height ||
        p->mode > NS2_TRIM || (p->mode == NS2_TRIM && p->frames < 5) ||
        !bayer(p->cfa) || !p->white || p->white > 16383 || !illum_valid(p->illuminant1) ||
        !ns2_scratch_bytes(p)) return NS2_BAD_ARGS;
    uint64_t pixels=(uint64_t)p->width*p->height;
    if (pixels > (UINT32_MAX-NS2_HEADER)/2u) return NS2_BAD_ARGS;
    for(unsigned i=0;i<4;i++) if (p->black[i]>=p->white) return NS2_BAD_ARGS;
    for(unsigned i=0;i<3;i++) if (!p->neutral[i]) return NS2_BAD_ARGS;
    int nz=0; for(unsigned i=0;i<9;i++) { if(p->color_matrix[i]) nz=1; }
    if(!nz || !p->model[0]) return NS2_BAD_ARGS;
    for(unsigned i=0;i<32;i++) { if(p->model[i]==0) return NS2_OK; }
    return NS2_BAD_ARGS;
}
void ns2_stack_row(const ns2_params *p, const uint16_t *src, uint16_t *out) {
    uint32_t n=p->frames, w=p->width, div=(p->mode==NS2_TRIM ? n-2:n);
    for(uint32_t x=0;x<w;x++) {
        uint32_t sum=0,min=UINT16_MAX,max=0;
        for(uint32_t f=0;f<n;f++) {
            uint32_t v=src[(size_t)f*w+x]; sum+=v;
            if(v<min)min=v;
            if(v>max)max=v;
        }
        if(p->mode==NS2_TRIM)sum-=min+max;
        uint32_t v=(sum*NS2_SCALE+div/2)/div;
        out[x]=(uint16_t)(v>UINT16_MAX ? UINT16_MAX:v);
    }
}
static void u16(uint8_t *b,uint32_t o,uint32_t v){b[o]=(uint8_t)v;b[o+1]=(uint8_t)(v>>8);}
static void u32(uint8_t *b,uint32_t o,uint32_t v){u16(b,o,v);u16(b,o+2,v>>16);}
typedef struct { uint8_t *b; uint32_t n,ext; } writer;
static void tag(writer *t,uint32_t id,uint32_t type,uint32_t cnt,uint32_t val){
    uint32_t off=NS2_IFD+2+t->n*12u;
    u16(t->b,off,id);u16(t->b,off+2,type);u32(t->b,off+4,cnt);u32(t->b,off+8,val);t->n++;
}
static uint32_t blob(writer *t,const void *src,uint32_t n){
    uint32_t off=t->ext;
    const uint8_t *s=(const uint8_t*)src;
    for(uint32_t i=0;i<n;i++)t->b[off+i]=s[i];
    t->ext+=(n+1u)&~1u;
    return off;
}
static void header(const ns2_params *p,uint8_t *b){
    for(uint32_t i=0;i<NS2_HEADER;i++)b[i]=0;
    b[0]='I';b[1]='I';u16(b,2,42);u32(b,4,NS2_IFD);
    writer t={b,0,NS2_EXTRA};
    uint32_t model=blob(&t,p->model,32);
    int32_t matrix[18];for(unsigned i=0;i<9;i++){matrix[2*i]=p->color_matrix[i];matrix[2*i+1]=10000;}
    uint32_t cm=blob(&t,matrix,sizeof matrix);
    uint32_t neutral[6];for(unsigned i=0;i<3;i++){neutral[2*i]=p->neutral[i];neutral[2*i+1]=1000000u;}
    uint32_t an=blob(&t,neutral,sizeof neutral);
    uint16_t black[4];for(unsigned i=0;i<4;i++)black[i]=(uint16_t)(p->black[i]*4u);
    uint32_t bl=blob(&t,black,sizeof black);
    static const char sw[NS2_SOFT_LEN]="z8 night stack 1";
    uint32_t sw_at=blob(&t,sw,sizeof sw);
    static const char desc_mean[NS2_DESC_LEN]="DERIVED CFA; mean of multiple RAW frames";
    static const char desc_trim[NS2_DESC_LEN]="DERIVED CFA; trimmed multiple RAW frames";
    uint32_t desc=blob(&t,p->mode==NS2_TRIM?desc_trim:desc_mean,NS2_DESC_LEN);
    uint32_t active[4]={0,0,p->height,p->width};
    uint32_t act=blob(&t,active,sizeof active);
    /* TIFF/DNG IFD tags strictly sorted. 2 bytes * width * height, single strip. */
    tag(&t,254,4,1,0);
    tag(&t,256,4,1,p->width);
    tag(&t,257,4,1,p->height);
    tag(&t,258,3,1,16);
    tag(&t,259,3,1,1);
    tag(&t,262,3,1,32803);
    tag(&t,270,2,NS2_DESC_LEN,desc);
    {static const char make[6]="NIKON";tag(&t,271,2,6,blob(&t,make,6));}
    {static const char model2[10]="NIKON Z 8";tag(&t,272,2,10,blob(&t,model2,10));}
    tag(&t,273,4,1,NS2_HEADER);
    tag(&t,274,3,1,1);
    tag(&t,277,3,1,1);
    tag(&t,278,4,1,p->height);
    tag(&t,279,4,1,(uint32_t)((uint64_t)p->width*p->height*2u));
    tag(&t,284,3,1,1);
    tag(&t,305,2,NS2_SOFT_LEN,sw_at);
    tag(&t,339,3,1,1); /* explicit unsigned integer TIFF SampleFormat */
    tag(&t,33421,3,2,0x00020002u);
    tag(&t,33422,1,4,(uint32_t)p->cfa[0]|((uint32_t)p->cfa[1]<<8)|((uint32_t)p->cfa[2]<<16)|((uint32_t)p->cfa[3]<<24));
    tag(&t,50706,1,4,0x00000401u); /* DNGVersion 1.4 */
    tag(&t,50707,1,4,0x00000401u); /* DNGBackwardVersion 1.4 */
    tag(&t,50708,2,32,model);
    tag(&t,50713,3,2,0x00020002u);
    tag(&t,50714,3,4,bl);
    tag(&t,50717,3,1,(uint32_t)p->white*4u);
    tag(&t,50721,10,9,cm);
    tag(&t,50728,5,3,an);
    tag(&t,50778,3,1,p->illuminant1);
    tag(&t,50829,4,4,act);
    u16(b,NS2_IFD,t.n);u32(b,NS2_IFD+2+t.n*12,0);
}
int ns2_write_dng(const ns2_params *p, ns2_read_row_fn rd,void *rctx,ns2_write_at_fn wr,void *wctx,uint16_t *scratch){
    if(ns2_validate(p)!=NS2_OK || !rd || !wr || !scratch)return NS2_BAD_ARGS;
    uint8_t h[NS2_HEADER];header(p,h);
    if(wr(wctx,0,h,NS2_HEADER))return NS2_WRITE_ERROR;
    uint16_t *out=scratch+(size_t)p->frames*p->width;
    for(uint32_t y=0;y<p->height;y++){
        for(uint32_t f=0;f<p->frames;f++){
            if(rd(rctx,f,y,scratch+(size_t)f*p->width,p->width))return NS2_READ_ERROR;
        }
        for(uint32_t f=0;f<p->frames;f++) {
            for(uint32_t x=0;x<p->width;x++) {
                if(scratch[(size_t)f*p->width+x]>p->white) return NS2_READ_ERROR;
            }
        }
        ns2_stack_row(p,scratch,out);
        uint32_t off=NS2_HEADER+(uint32_t)((uint64_t)y*p->width*2u);
        if(wr(wctx,off,out,p->width*2u))return NS2_WRITE_ERROR;
    }
    return NS2_OK;
}
