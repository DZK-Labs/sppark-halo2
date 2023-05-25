// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use halo2curves::bn256::{Fr, G1Affine, G1};

sppark::cuda_error!();

pub mod util;

#[cfg_attr(feature = "quiet", allow(improper_ctypes))]
extern "C" {
    fn mult_pippenger_inf(
        out: *mut G1,
        points_with_infinity: *const G1Affine,
        npoints: usize,
        scalars: *const Fr,
        ffi_affine_sz: usize,
    ) -> cuda::Error;
}

pub fn multi_scalar_mult_halo2(
    points: &[G1Affine],
    scalars: &[Fr],
) -> G1 {
    let npoints = points.len();
    if npoints != scalars.len() {
        panic!("length mismatch")
    }

    let mut ret = G1::default();
    let err = unsafe {
        mult_pippenger_inf(
            &mut ret as *mut _ as *mut G1,
            points as *const _ as *const G1Affine,
            npoints,
            scalars as *const _ as *const Fr,
            std::mem::size_of::<G1Affine>(),
        )
    };
    if err.code != 0 {
        panic!("{}", String::from(err));
    }

    ret
}
