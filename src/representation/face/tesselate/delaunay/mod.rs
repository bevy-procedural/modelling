use super::{Face, Mesh, Payload};
use crate::{
    math::{Scalar, Vector, Vector2D, Vector3D},
    representation::IndexType,
};
use std::collections::VecDeque;
mod dual;
use spade::{
    handles::{
        FixedDirectedEdgeHandle,
        VoronoiVertex::{self, Inner, Outer},
    },
    AngleLimit, ConstrainedDelaunayTriangulation, FloatTriangulation as _, HasPosition,
    InsertionError, Point2, RefinementParameters, Triangulation as _,
};

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Flips edges until the delaunay-condition is met.
    /// This is quite slow in the worst case O(n^3) but usually much better than the naive version.
    /// Assumes local indices
    #[deprecated(since = "0.1.0", note = "please use `delaunator` instead")]
    pub fn delaunayfy<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        first: usize,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        let eps = P::S::EPS; // TODO: Numerical instability... This is so close to zero we have to include equal to zero (or slightly smaller). This just doesn't work!
        let mut this_is_a_hack_eps = P::S::ZERO;

        let vs: Vec<(P::Vec2, V)> = self.vertices_2d::<V, P>(mesh).collect();
        let mut flips = 0;
        let max_flips = vs.len() * vs.len();
        let min_flips = vs.len() * ((vs.len() as f32).sqrt() as usize);

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
                    .is_inside_circumcircle(v21, v22, v23, this_is_a_hack_eps)
                {
                    dual.flip(&neigh, indices);
                    neigh.flip_indices(indices);

                    flips += 1;
                    if flips > max_flips {
                        // TODO:
                        println!("WARNING: Delaunay might not terminate if numerical instabilities are too bad. Aborting.");
                        return;
                    }
                    // After a few iterations, increase the eps to the "real" eps
                    if flips == min_flips {
                        this_is_a_hack_eps = eps;
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

    /// Converts the face into a triangle list using the delaunay triangulation.
    pub fn delaunay_triangulation<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));
        debug_assert!(!self.has_self_intersections(mesh));

        //let n = indices.len();
        //self.sweep_line(mesh, indices);
        //self.delaunayfy(mesh, indices, n);

        // using Delaunator because it is faster than spade for < 100_000 vertices
        // https://github.com/Stoeoef/spade/tree/master/delaunay_compare
        /*let vs: Vec<delaunator::Point> = self
            .vertices_2d::<V, P>(mesh)
            .map(|(vec2, _)| delaunator::Point {
                x: vec2.x().to_f64(),
                y: vec2.y().to_f64(),
            })
            .collect();
        let triangulation = delaunator::triangulate(&vs);
        indices.extend(triangulation.triangles.iter().map(|i| V::new(*i)));*/

        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::new();
        //TODO: faster: ConstrainedDelaunayTriangulation::bulk_load()
        let mut last = None;
        let mut first = None;
        for (i2, (vec2, _)) in self.vertices_2d::<V, P>(mesh).enumerate() {
            let i = cdt
                .insert(Point2::new(vec2.x().to_f64(), vec2.y().to_f64()))
                .unwrap();
            assert!(i.index() == i2);
            if let Some(j) = last {
                assert!(cdt.add_constraint(j, i));
            } else {
                first = Some(i);
            }
            last = Some(i);
        }
        assert!(cdt.add_constraint(last.unwrap(), first.unwrap()));

        let i2v = self
            .vertices_2d::<V, P>(mesh)
            .map(|(_, i)| i)
            .collect::<Vec<_>>();
        let i0 = indices.len();
        cdt.inner_faces().for_each(|face| {
            let [p0, p1, p2] = face.vertices();
            let v0 = V::new(p0.index());
            let v1 = V::new(p1.index());
            let v2 = V::new(p2.index());

            //if self.is_inside(mesh, i2v[p0.index()], i2v[p1.index()], i2v[p2.index()]) {
            if self.is_inside(
                mesh,
                V::new(i0 + p0.index()),
                V::new(i0 + p1.index()),
                V::new(i0 + p2.index()),
            ) {
                indices.extend(&[v0, v1, v2]);
            }
        });
    }
}
