use crate::{
    math::{HasPosition, Vector, Vector3D},
    mesh::{Face, Face3d, MeshBasics, MeshType, Vertex},
};
use std::collections::HashMap;

use super::{ear_clipping, Triangulation};

fn shorten<T: MeshType>(face: &T::Face, mesh: &T::Mesh, indices: &mut Vec<T::V>)
where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    // TODO: This shortens edges producing invalid meshes!
    let vs: Vec<(T::Vec2, T::V)> = face.vertices_2d(mesh).collect();
    assert!(vs.len() == face.vertices(mesh).count());
    let mut vsh: HashMap<T::V, T::Vec2> = HashMap::new();
    for (v, p) in vs {
        vsh.insert(p, v);
    }

    if indices.len() < 3 {
        return;
    }

    for _ in 0..indices.len() {
        let mut changed = false;
        for i in (0..indices.len()).step_by(3) {
            for j in ((i + 3)..indices.len()).step_by(3) {
                for k in 0..3 {
                    let a = indices[i + (0 + k) % 3];
                    let b = indices[i + (1 + k) % 3];
                    let c = indices[i + (2 + k) % 3];
                    for l in 0..3 {
                        let d = indices[j + (0 + l) % 3];
                        let e = indices[j + (1 + l) % 3];
                        let f = indices[j + (2 + l) % 3];
                        if a == e && b == d {
                            if vsh[&a].distance(&vsh[&b]) > vsh[&c].distance(&vsh[&f]) {
                                indices[i + (0 + k) % 3] = c;
                                indices[i + (1 + k) % 3] = f;
                                indices[i + (2 + k) % 3] = b;

                                indices[j + (0 + l) % 3] = c;
                                indices[j + (1 + l) % 3] = a;
                                indices[j + (2 + l) % 3] = f;

                                changed = true;

                                break;
                            }
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
}

/// Use multiple runs of randomized ear-clipping to approximate the minimum weight triangulation
pub fn min_weight_triangulation_stoch<T: MeshType>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Vec<T::V>,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    // TODO: O(n^3) algorithm http://www.ist.tugraz.at/_attach/Publish/Eaa19/Chapter_04_MWT_handout.pdf
    let mut best_indices = Vec::new();
    let mut best_dist: T::S = std::f32::INFINITY.into();

    for _ in 1..100 {
        let mut tmp_indices = Vec::new();
        ear_clipping::<T>(face, mesh, &mut Triangulation::new(&mut tmp_indices), true);

        // self.shorten(mesh, &mut local_indices);

        let mut dist = 0.0.into();

        for i in (0..tmp_indices.len()).step_by(3) {
            let a = mesh.vertex(tmp_indices[i]).pos();
            let b = mesh.vertex(tmp_indices[i + 1]).pos();
            let c = mesh.vertex(tmp_indices[i + 2]).pos();
            dist += a.distance(&b) + b.distance(&c) + c.distance(&a);
        }

        if dist < best_dist {
            best_dist = dist;
            best_indices = tmp_indices;
        }
    }
    indices.extend(best_indices);
}
