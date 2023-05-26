// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use sppark_msm::*;

#[test]
fn msm_correctness() {
    let test_npow = std::env::var("TEST_NPOW").unwrap_or("15".to_string());
    let npoints_npow = i32::from_str(&test_npow).unwrap();

    let (points, scalars) =
        util::generate_points_scalars(1usize << npoints_npow);

    let _ = multi_scalar_mult_halo2(points.as_slice(), scalars.as_slice());
}
