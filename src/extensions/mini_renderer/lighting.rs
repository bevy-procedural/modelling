use crate::{extensions::nalgebra::*, prelude::*};
use std::collections::HashMap;

pub(super) fn lighting(
    light_direction: Vec3<f32>,
    diffuse_color: Vec3<f32>,
    ambient_color: Vec3<f32>,
    normal: Vec3<f32>,
) -> Vec3<f32> {
    let intensity = normal.dot(&light_direction).max(0.0);
    intensity * diffuse_color + ambient_color //.clamp(0.0, 1.0)
}

pub(super) fn calculate_vertex_coords<T: MeshType3D, F: Fn(T::S) -> T::Trans>(
    mesh: &T::Mesh,
    steps: usize,
    perspective: &T::Trans,
    f: F,
) -> HashMap<T::V, Vec<(T::Vec, T::Vec)>>
where
    T::S: ScalarPlus,
{
    let mut vertex_coords = HashMap::<T::V, Vec<(T::Vec, T::Vec)>>::new();
    let mut transforms = Vec::new();
    if steps == 0 {
        transforms.push(f(T::S::ZERO));
    } else {
        for i in 0..2 * steps {
            let t: T::S =
                T::S::from_usize(i) / T::S::from_usize(2 * steps - 1) * T::S::TWO - T::S::ONE; // t ∈ [-1, 1]
            assert!(t >= -T::S::ONE && t <= T::S::ONE);
            transforms.push(f(t));
        }
    }
    for v in mesh.vertices() {
        if steps == 0 {
            let p = transforms[0].apply_point(v.pos());
            vertex_coords.insert(v.id(), vec![(p, perspective.apply_point(p))]);
        } else {
            let mut vs = Vec::new();
            vs.reserve(2 * steps);
            for i in 0..2 * steps {
                let p = transforms[i].apply_point(v.pos());
                vs.push((p, perspective.apply_point(p)));
            }
            vertex_coords.insert(v.id(), vs);
        }
    }

    vertex_coords
}
