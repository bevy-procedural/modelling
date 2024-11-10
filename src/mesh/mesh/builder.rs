use crate::mesh::{DefaultEdgePayload, DefaultFacePayload};

use super::{MeshBasics, MeshType};

// TODO: Simplify the builder and move it to `operations`.

/// Some basic operations to build meshes.
pub trait MeshPathBuilder<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Same as `add_isolated_edge` but with default edge payloads
    fn add_isolated_edge_default(&mut self, a: T::VP, b: T::VP) -> (T::V, T::V)
    where
        T::EP: DefaultEdgePayload;

    /// Generate a path from the finite iterator of positions and return the first and
    /// last edge resp. the arcs/halfedges pointing to the first and last vertex.
    fn insert_path(&mut self, vp: impl IntoIterator<Item = T::VP>) -> (T::E, T::E)
    where
        T::EP: DefaultEdgePayload;

    /// Same as `insert_path` but closes the path by connecting the last vertex with the first one.
    /// Also, returns only the first edge (outside the loop when constructed ccw).
    fn insert_loop(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E
    where
        T::EP: DefaultEdgePayload;
}

// TODO: We need a half-edge independent way of inserting vertices and edges! Most difficult part: how to handle edge payloads?

/// Some basic operations to build meshes.
pub trait MeshBuilder<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Same as `add_vertex_via_vertex` but with default edge payloads
    fn add_vertex_via_vertex_default(&mut self, v: T::V, vp: T::VP) -> (T::V, T::E, T::E)
    where
        T::EP: DefaultEdgePayload;

    /// Same as `add_vertex_via_edge` but with default edge payloads
    fn add_vertex_via_edge_default(
        &mut self,
        input: T::E,
        output: T::E,
        vp: T::VP,
    ) -> (T::V, T::E, T::E)
    where
        T::EP: DefaultEdgePayload;

    /// Removes the provided face.
    fn remove_face(&mut self, f: T::F) -> T::FP;

    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    fn close_hole(&mut self, e: T::E, fp: T::FP, curved: bool) -> T::F;

    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    fn close_hole_default(&mut self, e: T::E) -> T::F
    where
        T::FP: DefaultFacePayload,
    {
        self.close_hole(e, Default::default(), false)
    }

    /// Same as `close_face_vertices` but with default edge and face payloads
    fn close_face_vertices_default(
        &mut self,
        prev: T::V,
        from: T::V,
        to: T::V,
        curved: bool,
    ) -> (T::F, T::E, T::E)
    where
        T::EP: DefaultEdgePayload,
        T::FP: DefaultFacePayload;
}

// TODO: These need to be simplified

/// Some basic operations to build meshes with halfedges.
pub trait MeshHalfEdgeBuilder<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Inserts vertices a and b and adds an isolated edge between a and b.
    fn add_isolated_edge(&mut self, a: T::VP, epa: T::EP, b: T::VP, epb: T::EP) -> (T::V, T::V);

    /// Connects the vertices v0 and v1 with an edge and returns the edge id.
    /// This will not close any face, i.e., v0 and v1 must be in different connected components.
    /// Hence, they must be also on the boundary of each connected components.
    ///
    /// Returns the inserted pair of halfedges.
    fn insert_edge_between(
        &mut self,
        origin0: T::V,
        ep0: T::EP,
        origin1: T::V,
        ep1: T::EP,
    ) -> (T::E, T::E);

    /// Creates a new vertex based on `vp` and connects it to vertex `v` with a pair of halfedges.
    ///
    /// TODO: Docs
    fn add_vertex_via_vertex(
        &mut self,
        v: T::V,
        vp: T::VP,
        ep1: T::EP,
        ep2: T::EP,
    ) -> (T::V, T::E, T::E);

    /// Adds a vertex with the given payload via a new edge starting in input and ending in output
    ///
    /// TODO: Docs
    fn add_vertex_via_edge(
        &mut self,
        input: T::E,
        output: T::E,
        vp: T::VP,
        ep1: T::EP,
        ep2: T::EP,
    ) -> (T::V, T::E, T::E);

    // TODO: simplify other places using this function.
    /// Provided two halfedges that point to the start and end vertex of the new edge, insert that new edge.
    /// This will also update the neighbors of the new edge so the halfedge mesh is consistent.
    fn insert_edge(&mut self, inside: T::E, ep1: T::EP, outside: T::E, ep2: T::EP) -> (T::E, T::E);

    /// Inserts a single half edge with the given origin and target vertex.
    /// You can set next and prev to IndexType::max() to insert the id of the twin edge there.
    fn insert_halfedge_no_update_no_check(
        &mut self,
        e: T::E,
        origin: T::V,
        face: T::F,
        prev: T::E,
        twin: T::E,
        next: T::E,
        payload: T::EP,
    );

    /// Close the face by inserting a pair of halfedges, i.e.,
    /// connecting `inside` (targeting a vertex of the to-be-inserted edge) with the
    /// next halfedge to close the face and `outside` (targeting the other vertex)
    /// with the next halfedge to complete the outside.
    /// This works even with non-manifold vertices!
    ///
    /// Returns the new face and (first) the inside edge and (second) the outside edge.
    fn close_face(
        &mut self,
        inside: T::E,
        ep1: T::EP,
        outside: T::E,
        ep2: T::EP,
        fp: T::FP,
        curved: bool,
    ) -> (T::F, T::E, T::E);

    /// Close the face by connecting vertex `from` (coming from `prev`) with vertex `to`.
    /// Inserts a pair of halfedges between these two vertices.
    /// This will only work if the insertion is unambiguous without having to look at the vertex positions, i.e., this must be a manifold vertex!
    /// If `to` has more than one ingoing edge that can reach `from`, use `close_face` instead and provide the edges.
    fn close_face_vertices(
        &mut self,
        prev: T::V,
        ep1: T::EP,
        from: T::V,
        ep2: T::EP,
        to: T::V,
        fp: T::FP,
        curved: bool,
    ) -> (T::F, T::E, T::E);

    /// Same as `close_face` but with default edge and face payloads
    fn close_face_default(
        &mut self,
        inside: T::E,
        outside: T::E,
        curved: bool,
    ) -> (T::F, T::E, T::E)
    where
        T::EP: DefaultEdgePayload,
        T::FP: DefaultFacePayload;
}

/// Some basic operations to build meshes with halfedges.
/// 
/// TODO: These are kinda edgy. Avoid exposing them in the public API.
pub trait HalfEdgeSemiBuilder<T: MeshType> {
    /// Provided two edges that point to the start and end vertex of the new edge, insert that new edge.
    /// This will also update the neighbors of the new edge so the halfedge mesh is consistent.
    ///
    /// It will not run any checks to see if the insertion is valid!
    /// This allows you to run this method on invalid meshes given that the `next` of the two edges is valid.
    fn insert_edge_no_check(
        &mut self,
        inside: T::E,
        ep1: T::EP,
        outside: T::E,
        ep2: T::EP,
    ) -> (T::E, T::E);

    /// like `insert_edge_no_update_no_check` but with additional checks
    fn insert_edge_no_update(
        &mut self,
        _: (T::E, T::E, T::V, T::F, T::EP),
        _: (T::E, T::E, T::V, T::F, T::EP),
    ) -> (T::E, T::E);

    /// Will allocate two edges and return them as a tuple.
    /// You can set next and prev to IndexType::max() to insert the id of the twin edge there.
    /// This will not update the neighbors! After this operation, the mesh is in an inconsistent state.
    ///
    /// You have to make sure that the vertices will not be deleted afterwards and that there is no halfedge between them yet.
    /// Also, you have to update the neighbors yourself. After this operation, the mesh is in an inconsistent state.
    fn insert_edge_no_update_no_check(
        &mut self,
        _: (T::E, T::E, T::V, T::F, T::EP),
        _: (T::E, T::E, T::V, T::F, T::EP),
    ) -> (T::E, T::E);

    /// Will insert a new vertex inside this halfedge.
    /// After this, the mesh will be invalid since the twin is not updated!
    fn subdivide_unsafe(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> T::E;

    /// Call this on the twin of an halfedge where `subdivide_unsafe` was called
    /// and it will apply the same subdivision on this halfedge making the mesh valid again.
    /// Returns the id of the new edge. If the twin was not subdivided, it will return `None`.
    fn subdivide_unsafe_try_fixup(&mut self, e: T::E, ep: T::EP) -> Option<T::E>;
}
