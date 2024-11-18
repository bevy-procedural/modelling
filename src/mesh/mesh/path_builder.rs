use crate::{
    math::{HasPosition, IndexType},
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
    mesh: &'a mut T::Mesh,
    start_vertex: T::V,
    current_vertex: T::V,
    start_edges: Option<(T::E, T::E)>,
    current_edges: Option<(T::E, T::E)>,
}

/// Some basic operations to build meshes.
impl<'a, T: MeshType> PathBuilder<'a, T>
where
    T::Mesh: MeshBuilder<T> + 'a,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
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
                mesh,
                start_vertex: v,
                current_vertex: v,
                start_edges: None,
                current_edges: None,
            }
        }
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
            mesh,
            start_vertex,
            current_vertex: start_vertex,
            start_edges,
            current_edges: start_edges,
        }
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
        let Some((current_inner, current_outer)) = self.current_edges() else {
            // The current vertex doesn't have any edges yet.
            assert!(self.start_edges().is_none());
            assert!(self.start_vertex() == self.current_vertex());
            println!("start_vertex: {:?}", self.start_vertex());
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

        println!("start_inner: {:?}", start_inner);

        self.mesh().close_hole(start_inner, fp, false)
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
        let v = self.mesh().add_vertex(T::VP::from_pos(pos));
        self.line_to(v);
        self
    }

    /// Draws a straight line from the current vertex to a new vertex with the given payload.
    #[inline(always)]
    pub fn line_ex(&mut self, vp: T::VP, ep0: T::EP, ep1: T::EP) -> &mut Self
    where
        T::Edge: HalfEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
    {
        let v = self.mesh().add_vertex(vp);
        self.line_to_ex(v, ep0, ep1);
        self
    }

    /// Draws a quadratic bezier curve from the current vertex to a new vertex with the given payload.
    #[inline(always)]
    pub fn quadratic_bezier(&mut self, control: T::Vec, end: T::Vec) -> &mut Self
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
        T::Edge: HalfEdge<T> + CurvedEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        let v = self.mesh().add_vertex(T::VP::from_pos(end));
        self.quadratic_bezier_to(control, v);
        self
    }

    /// Draws a cubic bezier curve from the current vertex to a new vertex with the given payload.
    #[inline(always)]
    pub fn cubic_bezier(&mut self, control1: T::Vec, control2: T::Vec, end: T::Vec) -> &mut Self
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
        T::Edge: HalfEdge<T> + CurvedEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        let v = self.mesh().add_vertex(T::VP::from_pos(end));
        self.cubic_bezier_to(control1, control2, v);
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
        // TODO: Avoid these requirements! Also, only take the position. Make a "line_ex" version that takes payloads.

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

        // TODO: It seems to constructs the face in the wrong direction (flipped)
        if let Some((_inside, _)) = self.current_edges {
            println!("{:?}", self.mesh);
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
    pub fn quadratic_bezier_to(&mut self, control: T::Vec, end: T::V) -> &mut Self
    where
        T::Edge: HalfEdge<T> + CurvedEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        self.line_to(end);
        let (edge, _twin) = self.current_edges().unwrap();
        self.mesh()
            .edge_mut(edge)
            .set_curve_type(CurvedEdgeType::QuadraticBezier(control));
        self
    }

    /// Draws a cubic bezier curve from the current vertex to the given vertex.
    /// The vertex must have no edges at all or must only be adjacent to one "outside".
    pub fn cubic_bezier_to(&mut self, control1: T::Vec, control2: T::Vec, end: T::V) -> &mut Self
    where
        T::Edge: HalfEdge<T> + CurvedEdge<T>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
    {
        self.line_to(end);
        let (edge, _twin) = self.current_edges().unwrap();
        self.mesh()
            .edge_mut(edge)
            .set_curve_type(CurvedEdgeType::CubicBezier(control1, control2));
        self
    }
}
