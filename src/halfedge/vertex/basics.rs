use super::{HalfEdgeImplMeshType, HalfEdgeVertexImpl};
use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, MeshBasics, MeshType, VertexBasics},
};

impl<T: MeshType> VertexBasics<T> for HalfEdgeVertexImpl<T>
where
    T: HalfEdgeImplMeshType,
{
    /// Returns the index of the vertex
    #[inline]
    fn id(&self) -> T::V {
        self.id
    }

    fn is_isolated(&self, _mesh: &T::Mesh) -> bool {
        self.edge == IndexType::max()
    }

    /// Returns the payload of the vertex
    #[inline]
    fn payload(&self) -> &T::VP {
        &self.payload
    }

    /// Returns a mutable reference to the payload of the vertex
    #[inline]
    fn payload_mut(&mut self) -> &mut T::VP {
        &mut self.payload
    }

    /// Returns whether the vertex is a boundary vertex
    #[inline]
    fn is_boundary(&self, mesh: &T::Mesh) -> bool {
        self.edges_out(mesh)
            .any(|e| mesh.edge(e).load_or(false, |v| v.is_boundary()))
    }

    /*
    /// Returns whether the vertex is manifold
    #[inline]
    fn is_manifold(&self) -> bool {
        self.next == IndexType::max()
    }*/

    /// Returns whether the vertex has only one edge incident to it
    #[inline]
    fn has_only_one_edge(&self, mesh: &T::Mesh) -> bool {
        // self.edges(mesh).count() == 1
        if let Some(e) = self.edge(mesh) {
            e.prev_id() == e.twin_id()
        } else {
            false
        }
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline]
    fn edge_id(&self, _mesh: &T::Mesh) -> T::E {
        self.edge
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline]
    fn edge<'a>(&'a self, mesh: &'a T::Mesh) -> Option<&'a T::Edge> {
        // PERF: avoid clone
        if self.edge == IndexType::max() {
            None
        } else {
            Some(mesh.edge_ref(self.edge))
        }
    }

    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    #[inline]
    fn neighbors<'a>(&self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Vertex>
    where
        T: 'a,
    {
        self.edges_out(mesh)
            .map(|e| mesh.vertex_ref(mesh.edge(e).unwrap().target_id()))
    }

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    #[inline]
    fn faces<'a>(&self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Face>
    where
        T: 'a,
    {
        self.edges_out(mesh)
            .filter_map(|e| mesh.get_face(mesh.edge(e).load()?.face_id()))
    }

    fn is_manifold(&self, mesh: &T::Mesh) -> bool {
        // TODO: If there is a "non-manifold vertex wheel", i.e., you cannot reach all out_edges by going to the next edge sibling, this fails
        let e0 = self.edge_id(mesh);
        if e0 == IndexType::max() {
            return false;
        }
        let mut e = e0;
        // TODO: unwrap
        let mut last_state = mesh.edge(e).unwrap().has_face();
        let mut state_changes = 0;
        loop {
            let edge = mesh.edge(e).unwrap();
            debug_assert_eq!(edge.fork().unwrap().origin_id(), self.id());

            // go to the next sibling sib
            let sibling = edge.next_sibling().unwrap();
            e = sibling.id();

            if sibling.has_face() != last_state {
                state_changes += 1;
                last_state = !last_state;

                if state_changes > 2 {
                    return false;
                }
            }

            if e == e0 {
                // went a full round
                debug_assert!(state_changes % 2 == 0);
                return (state_changes == 0 && last_state) || state_changes == 2;
            }
        }
    }
}
