// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use halo2curves::bn256::{Fr, G1Affine};
use halo2curves::ff::Field;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

pub fn generate_points_scalars(
    len: usize,
) -> (Vec<G1Affine>, Vec<Fr>) {
    let rand_gen: usize = 1 << 11;
    let mut rng = ChaCha20Rng::from_entropy();

    let mut points = (0..rand_gen)
                .map(|_| G1Affine::random(&mut rng))
                .collect::<Vec<_>>();

    // Sprinkle in some infinity points
    points[3] = G1Affine::default();

    let scalars = (0..len)
        .map(|_| Fr::random(&mut rng))
        .collect::<Vec<_>>();

    while points.len() < len {
        points.append(&mut points.clone());
    }

    points.truncate(len);

    (points, scalars)
}
