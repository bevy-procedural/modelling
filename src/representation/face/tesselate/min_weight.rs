use super::{Face, Mesh, Payload};
use crate::{
    math::{Vector, Vector3D},
    representation::IndexType,
};
use std::collections::HashMap;

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    fn shorten<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>, indices: &mut Vec<V>)
    where
        P::Vec: Vector3D<P::S>,
    {
        // TODO: This shortens edges producing invalid meshess!
        let vs: Vec<(P::Vec2, V)> = self.vertices_2d::<V, P>(mesh).collect();
        assert!(vs.len() == self.vertices(mesh).count());
        let mut vsh: HashMap<V, P::Vec2> = HashMap::new();
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
    pub fn min_weight_triangulation_stoch<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        // TODO: O(n^3) algorithm http://www.ist.tugraz.at/_attach/Publish/Eaa19/Chapter_04_MWT_handout.pdf
        let mut best_indices = Vec::new();
        let mut best_dist: P::S = std::f32::INFINITY.into();

        for _ in 1..100 {
            let mut local_indices = Vec::new();
            self.ear_clipping_randomized(mesh, &mut local_indices);

            // self.shorten(mesh, &mut local_indices);

            let mut dist = 0.0.into();

            for i in (0..local_indices.len()).step_by(3) {
                let a = mesh.vertex(local_indices[i]).vertex();
                let b = mesh.vertex(local_indices[i + 1]).vertex();
                let c = mesh.vertex(local_indices[i + 2]).vertex();
                dist += a.distance(b) + b.distance(c) + c.distance(a);
            }

            if dist < best_dist {
                best_dist = dist;
                best_indices = local_indices;
            }
        }
        indices.extend(best_indices);
    }
}
