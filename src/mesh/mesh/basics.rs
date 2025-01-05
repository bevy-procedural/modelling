use crate::{
    math::IndexType,
    mesh::{EdgeBasics, FaceBasics, MeshType, VertexBasics},
};
use std::collections::HashSet;

/// Some basic operations to retrieve information about the mesh.
pub trait MeshBasics<T: MeshType<Mesh = Self>>: Default + std::fmt::Debug + Clone {
    /// Returns whether the vertex exists and is not deleted
    fn has_vertex(&self, index: T::V) -> bool;

    /// Returns whether the edge exists and is not deleted
    fn has_edge(&self, index: T::E) -> bool;

    /// Returns whether the face exists and is not deleted
    fn has_face(&self, index: T::F) -> bool;

    /// Returns a reference to the requested vertex
    fn vertex(&self, index: T::V) -> &T::Vertex;

    /// Returns a reference to the requested vertex or `None` if it does not exist.
    fn get_vertex(&self, index: T::V) -> Option<&T::Vertex>;

    /// Returns a reference to the requested edge
    fn edge<'a>(&'a self, index: T::E) -> &'a T::Edge;

    /// Returns a reference to the requested edge or `None` if it does not exist.
    fn get_edge<'a>(&'a self, index: T::E) -> Option<&'a T::Edge>;

    /// Returns a reference to the requested face
    fn face(&self, index: T::F) -> &T::Face;

    /// Returns a reference to the requested face or `None` if it does not exist.
    fn get_face(&self, index: T::F) -> Option<&T::Face>;

    /// Returns a mutable reference to the requested vertex
    fn vertex_mut(&mut self, index: T::V) -> &mut T::Vertex;

    /// Returns a mutable reference to the requested edge
    fn edge_mut<'a>(&'a mut self, index: T::E) -> &'a mut T::Edge;

    /// Returns a mutable reference to the requested face
    fn face_mut(&mut self, index: T::F) -> &mut T::Face;

    /// Whether the mesh is open, i.e., has boundary edges
    fn is_open(&self) -> bool;

    /// Returns the maximum vertex index in the mesh
    fn max_vertex_index(&self) -> usize;

    /// Returns the number of vertices in the mesh
    fn num_vertices(&self) -> usize;

    /// Returns the number of edges in the mesh
    fn num_edges(&self) -> usize;

    /// Returns the number of faces in the mesh
    fn num_faces(&self) -> usize;

    /// Clears the mesh (deletes all vertices, edges, and faces)
    fn clear(&mut self) -> &mut Self;

    /// Get the payload of the mesh
    fn payload(&self) -> &T::MP;

    /// Set the payload of the mesh
    fn set_payload(&mut self, payload: T::MP) -> &mut Self;

    /// Get a mutable reference to the payload of the mesh
    fn payload_mut(&mut self) -> &mut T::MP;

    /// Returns the edge payload.
    ///
    /// On half-edge meshes, the payload is shared between the two half-edges.
    /// Since this means the payload is not necessarily stored in the edge,
    /// we consider the edge payload a property of the mesh.
    fn edge_payload<'a>(&'a self, edge: &'a T::Edge) -> &'a T::EP;

    /// Returns a mutable reference to the edge payload.
    ///
    /// On half-edge meshes, the payload is shared between the two half-edges.
    /// Since this means the payload is not necessarily stored in the edge,
    /// we consider the edge payload a property of the mesh.
    ///
    /// Notice that the given edge is not modified.
    fn edge_payload_mut<'a>(&'a mut self, edge: &'a T::Edge) -> &'a mut T::EP;

    /// Returns an iterator over all non-deleted vertices
    fn vertices<'a>(&'a self) -> impl Iterator<Item = &'a T::Vertex>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted vertice's ids
    fn vertex_ids<'a>(&'a self) -> impl Iterator<Item = T::V>
    where
        T: 'a,
    {
        self.vertices().map(|v| v.id())
    }

    /// Returns an mutable iterator over all non-deleted vertices
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted edge's ids
    fn edge_ids<'a>(&'a self) -> impl Iterator<Item = T::E>
    where
        T: 'a,
        T::Face: 'a,
    {
        self.edges().map(|e| e.id())
    }

    /// Returns an mutable iterator over all non-deleted vertices
    fn edges_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Edge>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted faces
    fn faces<'a>(&'a self) -> impl Iterator<Item = &'a T::Face>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted face's ids
    fn face_ids<'a>(&'a self) -> impl Iterator<Item = T::F>
    where
        T: 'a,
        T::Face: 'a,
    {
        self.faces().map(|f| f.id())
    }

    /// Returns an mutable iterator over all non-deleted vertices
    fn faces_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Face>
    where
        T: 'a;

    /// Returns the id of the (half)edge from `v` to `w` or `None` if they are not neighbors.
    fn shared_edge(&self, v: T::V, w: T::V) -> Option<&T::Edge>;

    /// Returns the (half)edge id from v to w. Panics if the edge does not exist.
    fn shared_edge_id(&self, v: T::V, w: T::V) -> Option<T::E>;

    /// Returns whether the vertex ids are consecutive, i.e., 0, 1, 2, 3, ...
    fn has_consecutive_vertex_ids(&self) -> bool {
        let mut last_id: usize = 0;
        for v in self.vertices() {
            if v.id() != IndexType::new(last_id) {
                return false;
            }
            last_id += 1;
        }
        true
    }

    /// Since the vertex payloads in the `Deletable` can be sparse,
    /// we need to compact the vertices when converting them to a dense vector.
    /// This function returns the cloned compact vertices and maps the indices to the new compact buffer.
    fn dense_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP>;

    /// Returns the face shared by the two vertices or `None`.
    ///
    /// TODO: Currently cannot distinguish between holes and "the outside"
    fn shared_face(&self, v0: T::V, v1: T::V) -> Option<T::F>;

    /// Returns whether the mesh is connected, i.e., has only one connected component.
    fn is_connected(&self) -> bool {
        let mut seen = HashSet::<T::V>::new();
        let mut stack = Vec::<T::V>::new();
        if let Some(v) = self.vertices().next() {
            stack.push(v.id());
            seen.insert(v.id());
        } else {
            return true;
        }
        while let Some(v) = stack.pop() {
            for e in self.vertex(v).edges_out(self) {
                let w = e.target(self).id();
                if !seen.contains(&w) {
                    stack.push(w);
                    seen.insert(w);
                }
            }
        }
        seen.len() == self.num_vertices()
    }

    /// Returns the connected components of the mesh.
    fn connected_components(&self) -> Vec<Self> {
        // TODO: test
        let mut components = Vec::<Self>::new();
        let mut seen = HashSet::<T::V>::new();
        for v in self.vertices() {
            if seen.contains(&v.id()) {
                continue;
            }
            let mut stack = Vec::<T::V>::new();
            stack.push(v.id());
            seen.insert(v.id());
            let mut component = Vec::<T::V>::new();
            while let Some(v) = stack.pop() {
                component.push(v);
                for e in self.vertex(v).edges_out(self) {
                    let w = e.target(self).id();
                    if !seen.contains(&w) {
                        stack.push(w);
                        seen.insert(w);
                    }
                }
            }
            // PERF: `submesh` is slow because it doesn't know that we want the whole connected component so it has to iterate over much more vertices.
            components.push(self.submesh(component.into_iter()));
        }
        components
    }

    /// Returns the subgraph induced by the given vertices.
    fn submesh<Iter: Iterator<Item = T::V>>(&self, vertices: Iter) -> Self {
        // TODO: test
        todo!("{:?}", vertices.collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_mesh_connected() {
        let mut mesh = Mesh3d64::default();
        assert!(mesh.is_connected());
        mesh.insert_regular_polygon(1.0, 10);
        assert!(mesh.is_connected());
        let v = mesh.insert_vertex(VertexPayloadPNU::from_pos(Vec3::default()));
        assert!(!mesh.is_connected());
        mesh.remove_vertex(v);
        assert!(mesh.is_connected());
        mesh.insert_regular_polygon(1.0, 10);
        assert!(!mesh.is_connected());

        let mut mesh = Mesh3d64::cube(1.0);
        assert!(mesh.is_connected());
        mesh.insert_regular_polygon(1.0, 3);
        assert!(!mesh.is_connected());
        mesh.insert_regular_polygon(1.0, 3);
        assert!(!mesh.is_connected());
    }
}
