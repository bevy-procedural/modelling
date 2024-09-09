use rand::Rng;

use crate::math::{Scalar, HasZero, Vector2D};

/// Generates a zigzag pattern with `n` vertices, which
/// is the worst case for the sweep line triangulation
pub fn generate_zigzag<Vec2: Vector2D>(n: usize) -> Vec<Vec2> {
    assert!(n % 2 == 1);
    (0..(2 * n))
        .map(|i| {
            let mut offset = Vec2::S::ZERO;
            let mut x = Vec2::S::from_usize(i);
            if i > n {
                offset = 1.0.into();
                x = Vec2::S::from_usize(2 * n - i);
            }

            if i % 2 == 0 {
                offset += 2.0.into();
            }

            Vec2::from_xy(x, offset)
        })
        .collect()
}

/// Generates a star with a random number of vertices between `min_vert` and `max_vert`.
/// The angles are fixed but the radii are random within the given range.
pub fn random_star<Vec2: Vector2D>(
    min_vert: usize,
    max_vert: usize,
    min_r: f32,
    max_r: f32,
) -> Vec<Vec2> {
    let mut vec2s = Vec::new();
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(min_vert..=max_vert);
    for i in 0..n {
        // TODO: which direction should the star be oriented?
        let phi = i as f32 / n as f32 * 2.0 * std::f32::consts::PI;
        let r = rng.gen_range(min_r..=max_r);
        let x = r * phi.cos();
        let y = r * phi.sin();
        vec2s.push(Vec2::from_xy(Vec2::S::from(x), Vec2::S::from(y)));
    }

    vec2s
}
