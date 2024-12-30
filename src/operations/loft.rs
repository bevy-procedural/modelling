use crate::{
    math::IndexType,
    mesh::{DefaultEdgePayload, DefaultFacePayload, HalfEdge, MeshBuilder, MeshTypeHalfEdge},
};

// TODO: Adjust this to not be halfedge-specific

/// A trait for lofting a mesh.
pub trait MeshLoft<T: MeshTypeHalfEdge<Mesh = Self>>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// This will walk counter-clockwise along the given boundary and add a "hem" made from triangles.
    /// The payloads are given using the iterator.
    ///
    /// `start` must be an edge on the boundary pointing to the first vertex to be connected with the hem.
    ///
    /// Returns the edge pointing from the first inserted vertex to the target of `start`.
    /// If the iterator is empty, return `start` instead.
    ///
    /// If `shift` is true, the first inserted triangle will be with the tip pointing to the target of `start`.
    /// Otherwise, the first triangle will include the edge `start`.
    /// This doesn't affect the number of triangles but shifts the "hem" by one.
    #[deprecated(note = "Use `crochet` instead")]
    fn loft_tri(&mut self, start: T::E, shift: bool, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        // TODO: a more efficient implementation could bulk-insert everything at once
        // TODO: assertions

        // output will walk forward around the boundary
        let mut output = start;

        let mut first = true;
        let mut iter = vp.into_iter();
        let mut pos = iter.next();
        let mut ret = start;

        if shift && pos.is_some() {
            let input = self.edge(output).prev_id();
            self.insert_vertex_e(input, pos.unwrap(), Default::default());
            first = false;
            ret = self.edge(output).prev_id();
            pos = iter.next();
        }

        while pos.is_some() {
            let input = self.edge(output).prev_id();
            self.insert_vertex_e(input, pos.unwrap(), Default::default());

            // the first one shouldn't connect to the previous
            if !first {
                self.close_face_ee_legacy(
                    self.edge(input).next_id(),
                    self.edge(input).prev_id(),
                    Default::default(),
                    Default::default(),
                );
            } else {
                ret = self.edge(output).prev_id();
            }

            let new_output = self.edge(output).next_id();

            // only continue if there are more vertices
            pos = iter.next();

            // the last one also shouldn't connect to the next
            if pos.is_some() || shift {
                self.close_face_ee_legacy(
                    output,
                    self.edge(output).prev(self).prev_id(),
                    Default::default(),
                    Default::default(),
                );
            }

            // advance output to the next edge on the boundary
            output = new_output;

            first = false;
        }

        ret
    }

    /// Like `loft_tri` but closes the "hem" with a face.
    /// Returns the edge pointing from the first inserted vertex to the second inserted vertex.
    #[deprecated(note = "Use `crochet` instead")]
    fn loft_tri_closed(&mut self, start: T::E, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let e = self.loft_tri(start, false, vp);
        let inside = self.edge(e).twin(self).prev_id();
        let outside = self.edge(inside).prev(self).prev_id();
        self.close_face_ee_legacy(inside, outside, Default::default(), Default::default());
        self.close_face_ee_legacy(
            self.edge(e).twin_id(),
            outside,
            Default::default(),
            Default::default(),
        );
        self.edge(outside).next(self).next_id()
    }

    /// Walks along the given boundary and "crochet" a "hem" made from polygon faces.
    /// Each face consists of `n` vertices from the iterator
    /// and `m` vertices from the boundary of the existing mesh.
    /// Hence, it will create polygon faces with `n+m` vertices each.
    ///
    /// If the iterator is long enough to go once around the boundary,
    /// the "hem" will be automatically closed if `autoclose` is true.
    ///
    /// If the boundary is too short to fit the next `n` new vertices, none of them will be inserted.
    /// e.g., if you have a triangle and insert with `m=2`, then at most one face will be created since
    /// two won't fit.
    ///
    /// It returns the first and last edge on the new boundary, with one edge distance to the boundary, e.g.,
    /// - if `n>=2`: the edge between the first inserted vertex and the second inserted
    ///   vertex and the edge between the last to the second to last. The direction will
    ///   be chosen such that it is the edge that is not part of the inserted face. If the
    ///   hem is autoclosed, the last edge will be the edge from the last inserted vertex
    ///   to the first instead, so the edges will actually be neighbors.
    /// - if `n=0`: the edge between the first boundary vertex to the `n`th if `m=0`.
    /// If there is no new boundary edge, it will return `start` instead.
    /// The function will return `None` if the boundary had unclear connectivity
    /// (which shouldn't happen on half-edge meshes).
    ///
    /// The distance to the boundary makes the behavior easier to understand when using `open=false` or `autoclose=true`.
    /// Think of the mesh constructing the new "true" boundary around the existing boundary and connecting
    /// it with diagonals to the old boundary to form smaller faces in the room between the new and old boundary.
    /// When the mesh is autoclosed, all diagonals are inner edges
    /// while with `open=true` all diagonals are boundary edges.
    /// The returned edges are always from the "true" boundary and never diagonals.
    ///
    /// The operation is guaranteed to not insert edges that are not incident to any face.
    ///
    /// Parameters:
    /// - `start`: the edge on the boundary pointing from the first vertex used in the hem to the second.
    ///    Iff `m=0`, the edge will be ignored and can be safely set to `IndexType::max()`.
    /// - `n`: the number of vertices in the iterator that will be inserted in each face.
    /// - `m`: the number of vertices from the boundary that will be used in each face.
    ///   Notice that `n+m` must be at least 3.
    /// - `backwards`: if true, the boundary is walked backwards instead of forwards.
    ///    The `target` of `start` will be used instead of the `origin` to build the first face.
    /// - `autoclose`: if true and the iterator is long enough, the hem will be automatically closed.
    /// - `open`: if true, the hem will be connected back to the inner boundary in every iteration.
    ///   Hence, the newly created boundary will be longer by `2` edges per created face.
    ///   `open` cannot be `true` if `autoclose` is `true`.
    /// - `vp`: the iterator of vertices to be inserted in each face.
    ///   - If the iterator is too long to fit everything along the boundary, all additional vertices will be ignored.
    ///     Hence, it is always safe to provide an infinite iterator.
    ///   - If `n=0`, the iterator is ignored.
    ///   - If the iterator is too short and doesn't end with a completed face, the last face will have less vertices.
    ///   - If the iterator is too short and there is exactly one vertex left and `open` is `true`, there would be a single edge not connected to a face,
    ///     which is considered invalid. Hence, in that specific situation the last vertex will be ignored!
    ///   - If the iterator is empty but `n>0`, the function will return `start` instead.
    ///   - If the iterator has only length `1` and `m <= 11`, the function will return `start` instead.
    ///
    ///
    /// Some examples to illustrate special cases (see the `loft` example):
    /// - `n=0, m>=3`: append faces by using `m` vertices from the boundary and ignoring the iterator.
    ///    The value of `open` doesn't matter in this case.
    /// - `n=1, m=1, open`: Create a star graph (not a star polygon).
    ///    i.e., the first vertex (depending on `backwards` the `origin` or `target` of `start`)
    ///    is connected to all vertices of the iterators without inserting any faces.
    ///    This parameter combination is currently not supported since it doesn't insert any faces.
    /// - `n=1, m=2, open`: append triangles by using two vertices from the boundary and one from the iterator each,
    ///    e.g., if you start with a regular polygon, you'll get a star where the tips are from the iterator.
    /// - `n=1, m=2, !open`: insert only one vertex from the iterator and create a windmill around it,
    ///    e.g., if you start with a regular polygon, the result will be a pyramid.
    /// - `n>=2, m=1, !open`: Use only one vertex from the boundary and create a fan around it.
    ///    When `backwards` is true, this will be the target of `start`, otherwise the origin.
    ///   `autoclose` doesn't have any effect here, since it could only come into effect if the mesh was a single isolated vertex.
    /// - `n>=2, m=1, !open`: Like above, but the triangles are disconnected.
    /// - `n=2, m=2, open`: append disconnected quads to the boundary
    /// - `n=2, m=2, !open`: append a quad loft to the boundary
    /// - `n=2, m=3, !open`: create a hem of pentagons. The "tip" of each pentagon points inwards.
    /// - `n>=3, m=0, open`: insert disconnected polygons with `n` vertices each. Ignores `start`.
    /// - `n>=3, m=0, !open`: Generate a polygon strip. Ignores `start`.
    ///    Each polygon will have `n` vertices from the iterator and will be
    ///    connected with it's first vertex to the previous' polygons last vertex.
    ///    When `autoclose` is true, the last polygon will be connected to the first.
    /// - `n=3, m=2, !open`: create a hem of pentagons. The "tip" of each pentagon points outwards.
    #[must_use]
    #[inline(always)]
    fn crochet(
        &mut self,
        start: T::E,
        n: usize,
        m: usize,
        backwards: bool,
        autoclose: bool,
        open: bool,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> Option<(T::E, T::E)> {
        assert!(n + m >= 3, "n+m must be at least 3");
        assert!(
            !autoclose || !open,
            "autoclose and open cannot be true at the same time"
        );

        // TODO
        assert!(n >= 2);
        // TODO
        assert!(m >= 2);
        // TODO
        assert!(backwards);
        // TODO
        assert!(!open);

        // PERF: Instead of insert_face, we could directly insert the face indices when creating the edges

        // insert the outer boundary
        let mut iter = vp.into_iter();
        let mut inner = self.edge(start).prev_id();
        let mut last = false;
        let mut last_inner = start;
        let current_inner = inner;
        let mut outer = IndexType::max();
        let mut first_edge = start;
        let mut last_edge = start;

        loop {
            // Skip the center edges
            for _ in 1..m {
                if inner == last_inner {
                    // We reached the start again - so we are done!
                    return Some((first_edge, last_edge));
                }
                inner = self.edge(inner).prev_id();
            }

            // insert first diagonal towards bow in the first iteration
            if outer == IndexType::max() {
                let (e, _) =
                    self.insert_vertex_e(current_inner, iter.next()?, Default::default())?;
                last_inner = self.edge(e).twin_id();
                outer = e;
            }

            // Insert next bow
            for i in 1..n {
                let Some(vp) = iter.next() else {
                    if i == 1 && (n >= 3 || !autoclose) {
                        // We are done - the iterator ended just after the last bow
                        return Some((first_edge, last_edge));
                    }
                    // We are done - the iterator ended in the middle of the bow. Close it!
                    last = true;
                    break;
                };
                let (e, _) = self.insert_vertex_e(outer, vp, Default::default())?;
                outer = e;

                let e_twin = self.edge(e).twin_id();
                if first_edge == start {
                    first_edge = e_twin;
                }
                last_edge = e_twin;
            }

            let autoclose_now = autoclose && last && inner == last_inner;
            if autoclose_now {
                // automatically close the shape
                inner = self.edge(inner).prev_id();
            }

            // Insert the diagonal between inner and outer and create a face
            outer = self.insert_edge_ee(inner, self.edge(outer).next_id(), Default::default())?;
            self.insert_face(self.edge(outer).twin_id(), Default::default())?;

            if autoclose_now {
                // the last face was closed; this counts as the last boundary!
                last_edge = outer;
            }

            // TODO: Why is this not the same as above?
            /*let (e, f) = self.close_face_ee(
                inner,
                self.edge(outer).next_id(),
                Default::default(),
                Default::default(),
            )?;
            outer = self.edge(e).twin_id();*/

            if last {
                return Some((first_edge, last_edge));
            }
        }
    }

    /// see `crochet(start, n, m, true, true, false, vp)`
    fn loft_polygon_back(
        &mut self,
        start: T::E,
        n: usize,
        m: usize,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> Option<(T::E, T::E)> {
        self.crochet(start, n, m, true, true, false, vp)
    }

    /// see `crochet(start, n, m, true, false, false, vp)`
    #[must_use]
    fn loft_polygon(
        &mut self,
        start: T::E,
        n: usize,
        m: usize,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> Option<(T::E, T::E)>
    where
        T::Mesh: MeshBuilder<T>,
    {
        self.crochet(start, n, m, true, false, false, vp)
    }
}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};
    use itertools::Itertools;
    use std::{collections::HashSet, hash::RandomState};

    struct LoftTestConfig {
        n: usize,
        m: usize,
        backwards: bool,
        autoclose: bool,
        open: bool,
        mesh_start: (Mesh3d64, usize),
        vp: Vec<VertexPayloadPNU<f64, 3>>,

        // the following are expected results
        return_none: bool,
        area_in_appended_faces: Option<f64>,
        num_appended_edges: usize,
        num_boundary_edges: usize,
        num_appended_faces: usize,
        num_inserted_vertices: usize,
        num_inner_edges: usize,
        num_true_boundary: usize,
        num_diagonals: usize,
        connected: bool,
    }

    fn run_crochet_test(config: LoftTestConfig) {
        let mut mesh = config.mesh_start.0.clone();
        let res = mesh.crochet(
            config.mesh_start.1,
            config.n,
            config.m,
            config.backwards,
            config.autoclose,
            config.open,
            config.vp,
        );
        assert_eq!(res.is_none(), config.return_none);
        assert_eq!(mesh.check(), Ok(()));

        println!("{:?}", mesh);

        let old_vertices: HashSet<usize, RandomState> =
            HashSet::from_iter(config.mesh_start.0.vertex_ids());
        let new_vertices: HashSet<usize, RandomState> = HashSet::from_iter(mesh.vertex_ids());
        assert!(old_vertices.is_subset(&new_vertices));
        let inserted_vertices: HashSet<usize, RandomState> =
            HashSet::from_iter(new_vertices.symmetric_difference(&old_vertices).cloned());
        assert_eq!(inserted_vertices.len(), config.num_inserted_vertices);

        let old_halfedges: HashSet<usize, RandomState> =
            HashSet::from_iter(config.mesh_start.0.edge_ids());
        let new_halfedges: HashSet<usize, RandomState> = HashSet::from_iter(mesh.edge_ids());
        assert!(old_halfedges.is_subset(&new_halfedges));
        let inserted_halfedges: HashSet<usize, RandomState> =
            HashSet::from_iter(new_halfedges.symmetric_difference(&old_halfedges).cloned());
        assert_eq!(inserted_halfedges.len(), 2 * config.num_appended_edges);

        let old_faces: HashSet<usize, RandomState> =
            HashSet::from_iter(config.mesh_start.0.face_ids());
        let new_faces: HashSet<usize, RandomState> = HashSet::from_iter(mesh.face_ids());
        assert!(old_faces.is_subset(&new_faces));
        let inserted_faces: HashSet<usize, RandomState> =
            HashSet::from_iter(new_faces.symmetric_difference(&old_faces).cloned());
        assert_eq!(inserted_faces.len(), config.num_appended_faces);

        let boundary_edges = mesh
            .edges()
            .filter(|e| e.is_boundary_self())
            .map(|e| e.id())
            .collect_vec();
        assert_eq!(boundary_edges.len(), config.num_boundary_edges);

        let inner_edges = mesh
            .twin_edges()
            .filter(|(e, _)| !e.is_boundary(&mesh))
            .map(|(e, ee)| (e.id(), ee.id()))
            .collect_vec();
        assert_eq!(inner_edges.len(), config.num_inner_edges);

        let old_boundary: HashSet<usize, RandomState> = HashSet::from_iter(
            config
                .mesh_start
                .0
                .edges()
                .filter(|e| e.is_boundary_self())
                .map(|e| e.id())
                .collect_vec(),
        );
        let old_boundary_vertices: HashSet<usize, RandomState> =
            HashSet::from_iter(old_boundary.iter().map(|e| mesh.edge(*e).origin_id()));
        let diagonals: HashSet<usize, RandomState> = HashSet::from_iter(
            inserted_halfedges
                .iter()
                .filter(|e| old_boundary_vertices.contains(&mesh.edge(**e).origin_id()))
                .cloned(),
        );
        assert_eq!(diagonals.len(), config.num_diagonals);
        let true_boundary: HashSet<usize, RandomState> = HashSet::from_iter(
            boundary_edges
                .iter()
                .filter(|e| {
                    !(diagonals.contains(e) || diagonals.contains(&mesh.edge(**e).twin_id()))
                        && inserted_halfedges.contains(e)
                })
                .cloned(),
        );
        assert_eq!(true_boundary.len(), config.num_true_boundary);
        assert_eq!(
            true_boundary.len() + diagonals.len(),
            inserted_halfedges.len() / 2
        );

        for face in inserted_faces {
            assert_eq!(mesh.face(face).vertices(&mesh).count(), config.n + config.m);

            if let Some(a) = config.area_in_appended_faces {
                let poly = mesh.face(face).as_polygon(&mesh);
                assert!(
                    poly.signed_area().is_about(a, 1e-6),
                    "face {}: {} != {}, {:?}",
                    face,
                    poly.signed_area(),
                    a,
                    mesh.face(face).vertex_ids(&mesh).collect_vec()
                );
            };
        }

        assert_eq!(mesh.is_connected(), config.connected);

        if !config.return_none {
            let (first_edge, last_edge) = res.unwrap();
            assert!(boundary_edges.contains(&first_edge));
            assert!(boundary_edges.contains(&last_edge));
            if true_boundary.len() > 0 {
                assert!(true_boundary.contains(&first_edge));
                assert!(true_boundary.contains(&last_edge));
            }
            assert!(mesh.edge(first_edge).is_boundary_self());
            assert!(mesh.edge(last_edge).is_boundary_self());
        }
    }

    fn regular_polygon(n: usize) -> (Mesh3d64, usize) {
        let mut mesh = Mesh3d64::default();
        let e = mesh.insert_regular_polygon(1.0, n);
        (mesh, e)
    }
    fn wedge_area(n: usize) -> f64 {
        (regular_polygon_area(2.0, n) - regular_polygon_area(1.0, n)) / (n as f64)
    }

    #[test]
    fn test_crochet_2_2() {
        for n in [3, 4, 6, 7, 20] {
            for c in [
                LoftTestConfig {
                    n: 2,
                    m: 2,
                    backwards: true,
                    autoclose: false,
                    open: false,
                    mesh_start: regular_polygon(n),
                    vp: circle_iter::<3, MeshType3d64PNU>(n, 2.0, 0.0).collect_vec(),
                    return_none: false,
                    area_in_appended_faces: Some(wedge_area(n)),
                    num_appended_edges: 2 * (n - 1) + 1,
                    num_appended_faces: n - 1,
                    num_inserted_vertices: n,
                    num_boundary_edges: n + 2,
                    num_inner_edges: n - 1 + n - 2,
                    num_diagonals: n,
                    num_true_boundary: n - 1,
                    connected: true,
                },
                LoftTestConfig {
                    n: 2,
                    m: 2,
                    backwards: true,
                    autoclose: true,
                    open: false,
                    mesh_start: regular_polygon(n),
                    vp: circle_iter::<3, MeshType3d64PNU>(n, 2.0, 0.0).collect_vec(),
                    return_none: false,
                    area_in_appended_faces: Some(wedge_area(n)),
                    num_appended_edges: 2 * n,
                    num_appended_faces: n,
                    num_inserted_vertices: n,
                    num_boundary_edges: n,
                    num_inner_edges: 2 * n,
                    num_diagonals: n,
                    num_true_boundary: n,
                    connected: true,
                },
            ] {
                run_crochet_test(c);
            }
        }
    }
}
