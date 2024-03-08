use super::{Face, Mesh, Payload};
use crate::{
    math::{Scalar, Vector2D, Vector3D},
    representation::IndexType,
};
use std::collections::{HashMap, VecDeque};
mod dual;

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Flips edges until the delaunay-condition is met.
    /// This is quite slow in the worst case O(n^3) but usually much better than the naive version.
    /// Assumes local indices
    pub fn delaunayfy<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        first: usize,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        let eps = P::S::ZERO; // TODO: Numerical instability... This is so close to zero we have to include equal to zero (or slightly smaller). This just doesn't work!

        let vs: Vec<(P::Vec2, V)> = self.vertices_2d::<V, P>(mesh).collect();
        let mut flips = 0;
        let max_flips = vs.len() * vs.len();

        assert!(indices.len() - first == (self.num_vertices(mesh) - 2) * 3);
        assert!(indices[first..]
            .iter()
            .all(|i| i.index() < self.num_vertices(mesh) as usize));

        let mut dual = self.dual::<V>(indices, first);

        // dequeue to improve productivity (i.e., if it's stuck in a loop, it still circles through the whole queue before doing the loop again)
        let mut flip_list = VecDeque::from(dual.indices());

        loop {
            let d = if let Some(id) = flip_list.pop_front() {
                dual.vertex(&id)
            } else {
                break;
            };
            let s = d.start();
            let (v21, v22, v23) = (
                vs[indices[s + 0].index()].0,
                vs[indices[s + 1].index()].0,
                vs[indices[s + 2].index()].0,
            );
            for neighbor in d.neighbors_array().iter() {
                let neigh = dual.neighbor_rotated(&d.id(), indices, neighbor);

                if vs[indices[neigh.other_ns].index()]
                    .0
                    .is_inside_circumcircle(v21, v22, v23, eps)
                {
                    dual.flip(&neigh, indices);
                    neigh.flip_indices(indices);

                    flips += 1;
                    if flips > max_flips {
                        // TODO:
                        println!("WARNING: Delaunay might not terminate if numerical instabilities are too bad. Aborting.");
                        return;
                    }

                    // println!("{} <-> {}", neigh.s.index(), neigh.o.index());

                    // Push neighbors to fliplist
                    for n in dual.vertex(&neigh.s).neighbors_array() {
                        flip_list.push_back(n);
                    }
                    for n in dual.vertex(&neigh.o).neighbors_array() {
                        flip_list.push_back(n);
                    }

                    // don't look at more neighbors for now
                    break;
                }
            }
        }
    }

    /// Flips edges until the delaunay-condition is met. This is quite slow: O(n^3).
    pub fn delaunayfy_naive<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        local_indices: bool,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        let eps = P::S::EPS * P::S::from(10.0); // TODO
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
                                if vsh[&f].is_inside_circumcircle(vsh[&a], vsh[&b], vsh[&c], eps) {
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
