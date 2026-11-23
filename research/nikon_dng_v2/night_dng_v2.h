/* SPDX-License-Identifier: GPL-3.0-or-later */
#ifndef NIGHT_DNG_V2_H
#define NIGHT_DNG_V2_H
#include <stddef.h>
#include <stdint.h>
#define NS2_MAX_FRAMES 32u
#define NS2_SCALE 4u
#define NS2_MEAN 0u
#define NS2_TRIM 1u
#define NS2_OK 0
#define NS2_BAD_ARGS 1
#define NS2_READ_ERROR 2
#define NS2_WRITE_ERROR 3
/* Caller MUST derive and cross-validate metadata from original RAW frames.
 * DNG ColorMatrix1: signed rational XYZ(D50/D65 according to supplied illuminant)
 * to camera, with fixed 10000 denominator; do NOT invent its calibration.
 * CFA is phase for the exact decoded array origin, not an assumed pattern. */
typedef struct {
    uint32_t frames, width, height, mode;
    uint16_t black[4], white;
    uint8_t cfa[4];
    int32_t color_matrix[9];
    uint32_t neutral[3];     /* positive rational numerators, denominator 1000000 */
    uint16_t illuminant1;   /* DNG CalibrationIlluminant1, e.g. 21 = D65 */
    char model[32];         /* NUL-terminated; nonempty */
} ns2_params;
typedef int (*ns2_read_row_fn)(void *, uint32_t, uint32_t, uint16_t *, uint32_t);
typedef int (*ns2_write_at_fn)(void *, uint32_t, const void *, uint32_t);
uint32_t ns2_header_bytes(void);
size_t ns2_scratch_bytes(const ns2_params *p);
int ns2_validate(const ns2_params *p);
int ns2_write_dng(const ns2_params *p, ns2_read_row_fn rd, void *read_ctx,
                  ns2_write_at_fn wr, void *write_ctx, uint16_t *scratch);
void ns2_stack_row(const ns2_params *p, const uint16_t *src, uint16_t *out);
#endif
