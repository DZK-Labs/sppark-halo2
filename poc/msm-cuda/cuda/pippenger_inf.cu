// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#include <cuda.h>
#include <ff/alt_bn128.hpp>
#include <ec/jacobian_t.hpp>
#include <ec/xyzz_t.hpp>

typedef jacobian_t<fp_t> point_t;
typedef xyzz_t<fp_t> bucket_t;
typedef bucket_t::affine_inf_t affine_t;
typedef fr_t scalar_t;

#include <msm/pippenger.cuh>

#ifndef __CUDA_ARCH__
extern "C"
RustError mult_pippenger_inf(point_t* out, const affine_t points[], size_t npoints,
                                           const scalar_t scalars[],
					   size_t ffi_affine_sz)
{
    return mult_pippenger<bucket_t>(out, points, npoints, scalars, false, ffi_affine_sz);
}
#endif
