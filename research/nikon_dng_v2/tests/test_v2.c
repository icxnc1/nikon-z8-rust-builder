/* SPDX-License-Identifier: GPL-3.0-or-later */
#include "../night_dng_v2.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct { FILE *f; int fail_row; int fail_write_after; int writes; } ctx;
static int read_row(void *c,uint32_t fr,uint32_t y,uint16_t *out,uint32_t width){
    ctx *t=(ctx*)c;
    if(t->fail_row>=0 && (int)y==t->fail_row)return 1;
    for(uint32_t x=0;x<width;x++)out[x]=(uint16_t)(1008u+fr*3u+y*19u+x*7u);
    return 0;
}
static int write_at(void *c,uint32_t off,const void *p,uint32_t size){
    ctx *t=(ctx*)c;
    if(t->fail_write_after>=0 && t->writes>=t->fail_write_after)return 1;
    if(fseek(t->f,(long)off,SEEK_SET))return 1;
    if(fwrite(p,1,size,t->f)!=size)return 1;
    t->writes++;
    return 0;
}
static ns2_params good(unsigned frames,unsigned mode){
    ns2_params p={0};p.frames=frames;p.width=48;p.height=11;p.mode=mode;
    for(unsigned i=0;i<4;i++)p.black[i]=1008;
    p.white=16383;p.cfa[0]=0;p.cfa[1]=1;p.cfa[2]=1;p.cfa[3]=2;
    p.color_matrix[0]=11000;p.color_matrix[4]=10000;p.color_matrix[8]=9500;
    p.neutral[0]=540084;p.neutral[1]=1000000;p.neutral[2]=700410;
    p.illuminant1=21;strcpy(p.model,"NIKON Z 8");return p;
}
static void output(const char *path,unsigned frames,unsigned mode){
    ns2_params p=good(frames,mode);
    assert(ns2_validate(&p)==NS2_OK);
    size_t size=ns2_scratch_bytes(&p);
    uint16_t *scratch=calloc(1,size);assert(scratch);
    ctx t={0};t.fail_row=-1;t.fail_write_after=-1;
    t.f=fopen(path,"wb+");assert(t.f);
    assert(ns2_write_dng(&p,read_row,&t,write_at,&t,scratch)==NS2_OK);
    assert(t.writes==(int)p.height+1);
    fclose(t.f);free(scratch);
}
int main(int argc,char **argv){
    assert(argc==2);
    char p1[4096],p2[4096];snprintf(p1,sizeof p1,"%s/mean.dng",argv[1]);snprintf(p2,sizeof p2,"%s/trim.dng",argv[1]);
    output(p1,16,NS2_MEAN);output(p2,16,NS2_TRIM);
    int fails=0; ns2_params p=good(16,NS2_MEAN);
    #define NEG(MUTATE) do{ns2_params a=p;MUTATE; if(ns2_validate(&a)!=NS2_BAD_ARGS){fails++;}}while(0)
    NEG(a.frames=1);
    NEG(a.frames=33);
    NEG(a.mode=7);
    NEG(a.mode=NS2_TRIM;a.frames=4);
    NEG(a.width=0);
    NEG(a.height=0);
    NEG(a.width=UINT32_MAX;a.height=UINT32_MAX);
    NEG(a.cfa[0]=3);
    NEG(a.cfa[0]=1);
    NEG(a.black[0]=16383);
    NEG(a.white=0);
    NEG(a.white=16384);
    NEG(a.neutral[0]=0);
    NEG(a.illuminant1=0);
    NEG(a.illuminant1=65535);
    NEG(a.model[0]=0);
    NEG(memset(a.model,'x',sizeof(a.model)));
    NEG(memset(a.color_matrix,0,sizeof(a.color_matrix)));
    assert(fails==0);
    size_t size=ns2_scratch_bytes(&p);uint16_t *scratch=calloc(1,size);assert(scratch);
    ctx t={0};t.f=tmpfile();assert(t.f);t.fail_row=1;t.fail_write_after=-1;
    assert(ns2_write_dng(&p,read_row,&t,write_at,&t,scratch)==NS2_READ_ERROR);
    fclose(t.f);
    t.f=tmpfile();assert(t.f);t.fail_row=-1;t.fail_write_after=1;t.writes=0;
    assert(ns2_write_dng(&p,read_row,&t,write_at,&t,scratch)==NS2_WRITE_ERROR);
    fclose(t.f);free(scratch);
    printf("C_TESTS PASS negative=%u read_fail=1 write_fail=1 header=%u\n",18u,ns2_header_bytes());
    return 0;
}
