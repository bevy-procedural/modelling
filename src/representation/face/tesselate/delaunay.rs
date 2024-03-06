use super::{Face, Mesh, Payload};
use crate::{
    math::{Vector2D, Vector3D},
    representation::IndexType,
};
use std::collections::HashMap;

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Flips edges until the delaunay-condition is met. This is quite slow: O(n^3).
    pub fn delaunayfy<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        local_indices: bool,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        let vs: Vec<(P::Vec2, V)> = self.vertices_2d::<V, P>(mesh).collect();
        assert!(vs.len() == self.num_vertices(mesh));
        let mut vsh: HashMap<V, P::Vec2> = HashMap::new();
        if local_indices {
            for (i, (v, p)) in vs.iter().enumerate() {
                vsh.insert(V::new(i), *v);
            }
        } else {
            for (v, p) in vs {
                vsh.insert(p, v);
            }
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
                                if vsh[&f].is_inside_circumcircle(vsh[&a], vsh[&b], vsh[&c]) {
                                    //if vsh[&a].distance(&vsh[&b]) > vsh[&c].distance(&vsh[&f]) {
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

    /*/// Converts the face into a triangle list using the delaunay triangulation.
    pub fn delaunay_triangulation<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        _indices: &mut Vec<V>,
    ) {
        assert!(self.may_be_curved() || self.is_planar2(mesh));
        // TODO: or at least some other O(n log n) algorithm: https://en.wikipedia.org/wiki/Delaunay_triangulation
    }*/
}
