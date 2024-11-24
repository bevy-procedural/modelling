use crate::{
    math::{HasPosition, IndexType, TransformTrait, Transformable},
    mesh::{
        CurvedEdge, CurvedEdgeType, DefaultEdgePayload, EdgeBasics, HalfEdge, MeshBasics,
        MeshBuilder, MeshHalfEdgeBuilder, MeshType, VertexBasics,
    },
};

/// Some basic operations to build a single face.
pub struct PathBuilder<'a, T: MeshType>
where
    T::Mesh: MeshBuilder<T> + 'a,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Whether the path is closed
    closed: bool,

    /// Whether the path is closed and has a face
    has_face: bool,

    mesh: &'a mut T::Mesh,
    start_vertex: T::V,
    current_vertex: T::V,
    start_edges: Option<(T::E, T::E)>,
    current_edges: Option<(T::E, T::E)>,

    transform: T::Trans,
}

/// Some basic operations to build meshes.
impl<'a, T: MeshType> PathBuilder<'a, T>
where
    T::Mesh: MeshBuilder<T> + 'a,
    T::VP: HasPosition<T::Vec, S = T::S>
        + Transformable<Trans = T::Trans, Rot = T::Rot, Vec = T::Vec, S = T::S>,
{
    /// Create a new empty MeshPathBuilder.
    pub fn new(mesh: &'a mut T::Mesh) -> Self {
        Self {
            closed: false,
            has_face: false,
            mesh,
            // TODO: make sure to carefully handle cases where the start_vertex is undefined
            start_vertex: IndexType::max(),
            current_vertex: IndexType::max(),
            start_edges: None,
            current_edges: None,
            transform: T::Trans::identity(),
        }
    }

    /// Create a new MeshPathBuilder starting a new connected component.
    pub fn start(mesh: &'a mut T::Mesh, pos: T::Vec) -> Self
    where
        T::Edge: HalfEdge<T>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        Self::start_ex(mesh, T::VP::from_pos(pos))
    }

    /// Create a new MeshPathBuilder starting a new connected component.
    pub fn start_ex(mesh: &'a mut T::Mesh, vp: T::VP) -> Self
    where
        T::Edge: HalfEdge<T>,
    {
        // TODO: implement this without requiring `HalfEdge`

        let v = mesh.add_vertex(vp);
        Self::start_at(mesh, v)
    }

    /// Create a new MeshPathBuilder starting at the given vertex.
    pub fn start_at(mesh: &'a mut T::Mesh, v: T::V) -> Self
    where
        T::Edge: HalfEdge<T>,
    {
        // TODO: implement this without requiring `HalfEdge`

        if let Some(edge) = mesh.vertex(v).edge(mesh) {
            Self::start_at_edge(mesh, edge.prev_id())
        } else {
            Self {
                closed: false,
                has_face: false,
                mesh,
                start_vertex: v,
                current_vertex: v,
                start_edges: None,
                current_edges: None,
                transform: T::Trans::identity(),
            }
        }
    }

    /// Apply a transformation to the transformation applied to new vertices.
    fn transform(&mut self, t: &T::Trans) -> &mut Self {
        self.transform = self.transform * *t;
        self
    }

    /// Reset the transformation applied to new vertices.
    fn reset_transform(&mut self) -> &mut Self {
        self.transform = T::Trans::identity();
        self
    }

    /// Rotate the transformation applied to new vertices.
    fn rotate(&mut self, _r: T::Rot) -> &mut Self {
        todo!()
    }

    /// Translate the transformation applied to new vertices.
    fn translate(&mut self, v: T::Vec) -> &mut Self {
        self.transform = self.transform.with_translation(v);
        self
    }

    /// Scale the transformation applied to new vertices.
    fn scale(&mut self, v: T::Vec) -> &mut Self {
        self.transform = self.transform.with_scale(v);
        self
    }

    /// Return the current transformation applied to new vertices.
    fn get_transform(&self) -> T::Trans {
        self.transform
    }

    /// Create a new MeshPathBuilder starting at the target of the given
    /// (half)edge which will be on the outside of the created face.
    pub fn start_at_edge(mesh: &'a mut T::Mesh, e: T::E) -> Self
    where
        T::Edge: HalfEdge<T>,
    {
        // TODO: implement this without requiring `HalfEdge`

        let edge = mesh.edge(e);
        let start_edges = Some((e, edge.prev_id()));
        let start_vertex = edge.target_id(mesh);
        Self {
            closed: false,
            has_face: false,
            mesh,
            start_vertex,
            current_vertex: start_vertex,
            start_edges,
            current_edges: start_edges,
            transform: T::Trans::identity(),
        }
    }

    /// Returns whether the path is closed or empty. Doesn't check whether the path has a face.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Returns whether the path is closed and has a face.
    pub fn has_face(&self) -> bool {
        assert!(!self.has_face || self.is_closed());
        self.has_face
    }

    /// Returns the current vertex.
    pub fn current_vertex(&self) -> T::V {
        self.current_vertex
    }

    /// Returns the current edges (or `None` if the current vertex doesn't have any edges).
    pub fn current_edges(&self) -> Option<(T::E, T::E)> {
        self.current_edges
    }

    /// Returns the starting vertex.
    pub fn start_vertex(&self) -> T::V {
        self.start_vertex
    }

    /// Returns the starting edges  (or `None` if the current vertex doesn't have any edges).
    pub fn start_edges(&self) -> Option<(T::E, T::E)> {
        self.start_edges
    }

    /// Returns the mesh.
    pub fn mesh(&mut self) -> &mut T::Mesh {
        self.mesh
    }

    /// Fills the path with a face.
    ///
    /// If the path is not yet closed (this may include vertices that
    /// have not been created using this `PathBuilder`; i.e., it will consider the path
    /// "not closed" when the last inserted edge finds it twin before it findes the start),
    /// draws a straight line to the starting vertex.
    ///
    /// Returns the new face. If the path contains only one vertex, this will return `IndexType::max()`.
    pub fn close(&mut self, fp: T::FP) -> T::F
    where
        T::Edge: HalfEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        self.has_face = true;

        if self.is_closed() {
            let start_inner = self.start_edges().expect("The path is empty.").0;
            return self.mesh().close_hole(start_inner, fp, false);
        }

        self.closed = true;

        let Some((current_inner, current_outer)) = self.current_edges() else {
            // The current vertex doesn't have any edges yet.
            assert!(self.start_edges().is_none());
            assert!(self.start_vertex() == self.current_vertex());
            return IndexType::max();
        };
        let Some((start_inner, _start_outer)) = self.start_edges() else {
            // Shouldn't happen since we checked for this case in the previous block.
            assert!(false);
            return IndexType::max();
        };

        let end_of_path = self
            .mesh()
            .edge(current_inner)
            .clone()
            .edges_face(self.mesh())
            .find(|e| e.id() == current_outer || e.id() == start_inner)
            .expect("The path is malformed.");

        if end_of_path.id() == current_outer {
            // The path is open and needs to be closed.
            //self.mesh().close
            self.line_to(self.start_vertex());
        }

        // TODO: is this necessary or not? Generally, is the correction above correct? Or is the winding in the opposite direction?
        /*debug_assert!(self
        .mesh()
        .edge(current_inner)
        .clone()
        .edges_face(self.mesh())
        .find(|e| e.id() == start_inner)
        .is_some());*/

        self.mesh().close_hole(start_inner, fp, false)
    }

    /// Add a vertex or return the index of the start vertex if the position is the same.
    #[inline(always)]
    pub fn add_vertex_autoclose(&mut self, v: T::Vec) -> T::V {
        assert!(!self.is_closed());

        let sv = self.start_vertex();
        if self.mesh().vertex(sv).pos() == v {
            return self.start_vertex;
        }

        self.add_transformed_pos(v)
    }

    /// Draws a straight line from the current vertex to a new vertex with the given position.
    #[inline(always)]
    pub fn line(&mut self, pos: T::Vec) -> &mut Self
    where
        T::Edge: HalfEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        let w = self.transform.apply(pos);
        let v = self.mesh().add_vertex(T::VP::from_pos(w));
        self.line_to(v);
        self
    }

    /// Moves to the given vertex.
    /// Assumes the path is currently empty or closed to begin a new path.
    pub fn move_to(&mut self, pos: T::V) -> &mut Self {
        todo!()
    }

    fn add_transformed_pos(&mut self, pos: T::Vec) -> T::V {
        let w = self.transform.apply(pos);
        self.mesh().add_vertex(T::VP::from_pos(w))
    }

    /// Creates a new vertex at the given position and moves to it.
    /// Assumes the path is currently empty or closed to begin a new path.
    pub fn move_to_new(&mut self, pos: T::Vec) -> &mut Self {
        assert!(self.start_vertex() == IndexType::max());
        assert!(self.start_edges().is_none());
        assert!(self.current_vertex() == IndexType::max());

        let v = self.add_transformed_pos(pos);
        self.start_vertex = v;
        self.current_vertex = v;
        self.closed = false;
        self
    }

    /// Draws a straight line from the current vertex to a new vertex with the given payload.
    #[inline(always)]
    pub fn line_ex(&mut self, vp: T::VP, ep0: T::EP, ep1: T::EP) -> &mut Self
    where
        T::Edge: HalfEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
    {
        let vp2 = vp.transformed(&self.transform);
        let v = self.mesh().add_vertex(vp2);
        self.line_to_ex(v, ep0, ep1);
        self
    }

    /// Draws a quadratic bezier curve from the current vertex to a new vertex with the given payload.
    #[inline(always)]
    pub fn quad(&mut self, control: T::Vec, end: T::Vec) -> &mut Self
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
        T::Edge: HalfEdge<T> + CurvedEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        let v = self.add_transformed_pos(end);
        self.quad_to(control, v);
        self
    }

    /// Draws a cubic bezier curve from the current vertex to a new vertex with the given payload.
    #[inline(always)]
    pub fn cubic(&mut self, control1: T::Vec, control2: T::Vec, end: T::Vec) -> &mut Self
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
        T::Edge: HalfEdge<T> + CurvedEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        let v = self.add_transformed_pos(end);
        self.cubic_to(control1, control2, v);
        self
    }

    /// Draws a straight line from the current vertex to the given vertex.
    /// The vertex must have no edges at all or must only be adjacent to one "outside".
    pub fn line_to(&mut self, v: T::V) -> &mut Self
    where
        T::Edge: HalfEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        self.line_to_ex(v, Default::default(), Default::default())
    }

    /// Draws a straight line from the current vertex to the given vertex.
    /// The vertex must have no edges at all or must only be adjacent to one "outside".
    pub fn line_to_ex(&mut self, v: T::V, ep0: T::EP, ep1: T::EP) -> &mut Self
    where
        T::Edge: HalfEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
    {
        // TODO: Avoid these requirements! Also, only take the position.

        assert!(!self.is_closed(), "The path is already closed.");
        if v == self.start_vertex() {
            // The path is closed.
            self.closed = true;
        }

        // TODO: It seems to constructs the face in the wrong direction (flipped)
        if let Some((_inside, _)) = self.current_edges {
            //let edges = self.mesh().insert_edge(inside, ep0, outside, ep1);
            let origin = self.current_vertex();
            let edges = self.mesh().insert_edge_between(origin, ep0, v, ep1);
            self.current_edges = Some(edges);
        } else {
            // The current vertex doesn't have any edges yet.
            assert!(self.start_edges().is_none());
            assert!(self.start_vertex() == self.current_vertex());
            let origin = self.current_vertex();
            let edges = self.mesh().insert_edge_between(origin, ep0, v, ep1);
            self.current_edges = Some(edges);
            self.start_edges = Some((edges.1, edges.0));
        }

        self.current_vertex = v;
        self
    }

    /// Draws a quadratic bezier curve from the current vertex to the given vertex.
    /// The vertex must have no edges at all or must only be adjacent to one "outside".
    pub fn quad_to(&mut self, control: T::Vec, end: T::V) -> &mut Self
    where
        T::Edge: HalfEdge<T> + CurvedEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        let ct = self.transform.apply(control);
        self.line_to(end);
        let (edge, _twin) = self.current_edges().unwrap();
        self.mesh()
            .edge_mut(edge)
            .set_curve_type(CurvedEdgeType::QuadraticBezier(ct));
        self
    }

    /// Draws a cubic bezier curve from the current vertex to the given vertex.
    /// The vertex must have no edges at all or must only be adjacent to one "outside".
    pub fn cubic_to(&mut self, control1: T::Vec, control2: T::Vec, end: T::V) -> &mut Self
    where
        T::Edge: HalfEdge<T> + CurvedEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        let ct1 = self.transform.apply(control1);
        let ct2 = self.transform.apply(control2);
        self.line_to(end);
        let (edge, _twin) = self.current_edges().unwrap();
        self.mesh()
            .edge_mut(edge)
            .set_curve_type(CurvedEdgeType::CubicBezier(ct1, ct2));
        self
    }
}
