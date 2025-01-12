use crate::{
    math::IndexType,
    mesh::{
        CursorData, EdgeBasics, EdgeCursor, EdgeCursorBasics, EdgeCursorMut, FaceBasics,
        FaceCursor, FaceCursorMut, MeshType, VertexBasics, VertexCursor, VertexCursorBasics,
        VertexCursorMut,
    },
};
use std::collections::HashSet;

/// Some basic operations to retrieve information about the mesh.
pub trait MeshBasics<T: MeshType<Mesh = Self>>: Default + std::fmt::Debug + Clone {
    //======================= Vertex Operations =======================//

    /// Returns whether the vertex exists and is not deleted
    #[must_use]
    fn has_vertex(&self, index: T::V) -> bool;

    /// Returns a reference to the requested vertex or `None` if it does not exist.
    #[must_use]
    fn get_vertex(&self, index: T::V) -> Option<&T::Vertex>;

    /// Returns a reference to the requested vertex or panics if is not found.
    #[inline]
    #[must_use]
    fn vertex_ref(&self, index: T::V) -> &T::Vertex {
        self.get_vertex(index)
            .expect(format!("Vertex {} not found", index).as_str())
    }

    /// Returns an immutable vertex cursor to the requested vertex. Doesn't panic, even if the vertex does not exist.
    #[inline]
    #[must_use]
    fn vertex(&self, index: T::V) -> VertexCursor<'_, T> {
        VertexCursor::new(self, index)
    }

    /// Returns a mutable reference to the requested vertex or `None` if it does not exist.
    #[must_use]
    fn get_vertex_mut(&mut self, index: T::V) -> Option<&mut T::Vertex>;

    /// Returns a mutable reference to the requested vertex or panics if is not found.
    #[inline]
    #[must_use]
    fn vertex_ref_mut(&mut self, index: T::V) -> &mut T::Vertex {
        self.get_vertex_mut(index)
            .expect(format!("Vertex {} not found", index).as_str())
    }

    /// Returns a mutable vertex cursor to the requested vertex. Doesn't panic, even if the vertex does not exist.
    #[inline]
    #[must_use]
    fn vertex_mut(&mut self, index: T::V) -> VertexCursorMut<'_, T> {
        VertexCursorMut::new(self, index)
    }

    /// Returns the number of vertices in the mesh
    #[must_use]
    fn num_vertices(&self) -> usize;

    /// Returns an iterator over all non-deleted vertices
    #[must_use]
    fn vertex_refs<'a>(&'a self) -> impl Iterator<Item = &'a T::Vertex>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted vertice's ids
    #[inline]
    #[must_use]
    fn vertex_ids<'a>(&'a self) -> impl Iterator<Item = T::V>
    where
        T: 'a,
    {
        self.vertex_refs().map(|v| v.id())
    }

    /// Returns an iterator of cursors for each non-deleted vertex
    #[inline]
    #[must_use]
    fn vertices<'a>(&'a self) -> impl Iterator<Item = VertexCursor<'a, T>>
    where
        T: 'a,
    {
        self.vertex_ids().map(move |id| VertexCursor::new(self, id))
    }

    /// Returns an mutable iterator over all non-deleted vertices
    #[must_use]
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T: 'a;

    /// Iterates all outgoing (half)edges (resp. all edges in outwards-direction
    /// if undirected) incident to this vertex (clockwise)
    /// Returns an empty iterator if the vertex does not exist.
    #[must_use]
    fn vertex_edges_out(&self, v: T::V) -> impl Iterator<Item = T::E>;

    /// Iterates all ingoing (half)edges (resp. all edges in outwards-direction
    /// if undirected) incident to this vertex (clockwise)
    /// Returns an empty iterator if the vertex does not exist.
    #[must_use]
    fn vertex_edges_in(&self, v: T::V) -> impl Iterator<Item = T::E>;

    /// Iterates all neighbors of the vertex.
    /// Returns an empty iterator if the vertex does not exist.
    #[must_use]
    fn vertex_neighbors(&self, v: T::V) -> impl Iterator<Item = T::V>;

    /// Iterates all faces adjacent to the vertex.
    /// Returns an empty iterator if the vertex does not exist.
    #[must_use]
    fn vertex_faces(&self, v: T::V) -> impl Iterator<Item = T::F>;

    //======================= Edge Operations =======================//

    /// Returns whether the edge exists and is not deleted
    #[must_use]
    fn has_edge(&self, index: T::E) -> bool;

    /// Returns a reference to the requested edge or `None` if it does not exist.
    #[must_use]
    fn get_edge<'a>(&'a self, index: T::E) -> Option<&'a T::Edge>;

    /// Returns a reference to the requested edge or panics if is not found.
    #[inline]
    #[must_use]
    fn edge_ref<'a>(&'a self, index: T::E) -> &'a T::Edge {
        self.get_edge(index)
            .expect(format!("Edge {} not found", index).as_str())
    }

    /// Returns an immutable edge cursor to the requested edge. Doesn't panic, even if the edge does not exist.
    #[inline]
    #[must_use]
    fn edge(&self, index: T::E) -> EdgeCursor<'_, T> {
        EdgeCursor::new(self, index)
    }

    /// Returns a mutable reference to the requested edge or `None` if it does not exist.
    #[must_use]
    fn get_edge_mut<'a>(&'a mut self, index: T::E) -> Option<&'a mut T::Edge>;

    /// Returns a mutable reference to the requested edge or panics if is not found.
    #[inline]
    #[must_use]
    fn edge_ref_mut<'a>(&'a mut self, index: T::E) -> &'a mut T::Edge {
        self.get_edge_mut(index)
            .expect(format!("Edge {} not found", index).as_str())
    }

    /// Returns a mutable edge cursor to the requested edge. Doesn't panic, even if the edge does not exist.
    #[inline]
    #[must_use]
    fn edge_mut(&mut self, index: T::E) -> EdgeCursorMut<'_, T> {
        EdgeCursorMut::new(self, index)
    }

    /// Returns the number of edges in the mesh
    #[must_use]
    fn num_edges(&self) -> usize;

    /// Returns the edge payload.
    ///
    /// On half-edge meshes, the payload is shared between the two half-edges.
    /// Since this means the payload is not necessarily stored in the edge,
    /// we consider the edge payload a property of the mesh.
    ///
    /// Panics if the edge is deleted or does not exist.
    #[must_use]
    fn edge_payload<'a>(&'a self, edge: T::E) -> &'a T::EP;

    /// Returns a mutable reference to the edge payload.
    ///
    /// On half-edge meshes, the payload is shared between the two half-edges.
    /// Since this means the payload is not necessarily stored in the edge,
    /// we consider the edge payload a property of the mesh.
    ///
    /// Notice that the given edge is not modified.
    ///
    /// Panics if the edge is deleted or does not exist.
    #[must_use]
    fn edge_payload_mut<'a>(&'a mut self, edge: T::E) -> &'a mut T::EP;

    /// Returns an iterator over all non-deleted edges.
    /// For halfedge graphs, this will enumerate only one halfedge per edge.
    #[must_use]
    fn edge_refs<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Returns an edge cursor for each non-deleted edge.
    /// For halfedge graphs, this will enumerate only one halfedge per edge.
    #[inline]
    #[must_use]
    fn edges<'a>(&'a self) -> impl Iterator<Item = EdgeCursor<'a, T>>
    where
        T: 'a,
    {
        self.edge_ids().map(move |id| EdgeCursor::new(self, id))
    }

    /// Returns an iterator over all non-deleted edges' ids.
    /// For halfedge graphs, this will enumerate only one halfedge per edge.
    #[inline]
    #[must_use]
    fn edge_ids<'a>(&'a self) -> impl Iterator<Item = T::E> + 'a
    where
        T: 'a,
    {
        self.edge_refs().map(|e| e.id())
    }

    /// Returns an mutable iterator over all non-deleted vertices
    #[must_use]
    fn edges_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Edge>
    where
        T: 'a;

    /// Returns a (half)edge from `v` to `w`.
    /// Returns `None` if there is no edge between `v` and `w`.
    /// If there are multiple edges between `v` and `w`, any of them may be returned.
    #[must_use]
    fn shared_edge(&self, v: T::V, w: T::V) -> Option<&T::Edge>;

    /// Returns the id of a (half)edge from v to w.
    /// Returns `None` if there is no edge between `v` and `w`.
    /// If there are multiple edges between `v` and `w`, any of them may be returned.
    #[must_use]
    fn shared_edge_id(&self, v: T::V, w: T::V) -> Option<T::E>;

    /// Returns all (half)edges from v to w.
    #[must_use]
    fn shared_edges<'a>(&'a self, v: T::V, w: T::V) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Returns the ids of all (half)edges from v to w.
    #[must_use]
    fn shared_edge_ids(&self, v: T::V, w: T::V) -> impl Iterator<Item = T::E>;

    //======================= Face Operations =======================//

    /// Returns whether the face exists and is not deleted
    #[must_use]
    fn has_face(&self, index: T::F) -> bool;

    /// Returns a reference to the requested face or `None` if it does not exist.
    #[must_use]
    fn get_face(&self, index: T::F) -> Option<&T::Face>;

    /// Returns a reference to the requested face or panics if is not found.
    #[inline]
    #[must_use]
    fn face_ref(&self, index: T::F) -> &T::Face {
        self.get_face(index)
            .expect(format!("Face {} not found", index).as_str())
    }

    /// Returns an immutable face cursor to the requested face. Doesn't panic, even if the face does not exist.
    #[inline]
    #[must_use]
    fn face(&self, index: T::F) -> FaceCursor<'_, T> {
        FaceCursor::new(self, index)
    }

    /// Returns a reference to the requested face or `None` if it does not exist.
    #[must_use]
    fn get_face_mut(&mut self, index: T::F) -> Option<&mut T::Face>;

    /// Returns a mutable reference to the requested face or panics if is not found.
    #[inline]
    #[must_use]
    fn face_ref_mut(&mut self, index: T::F) -> &mut T::Face {
        self.get_face_mut(index)
            .expect(format!("Face {} not found", index).as_str())
    }

    /// Returns a mutable face cursor to the requested face. Doesn't panic, even if the face does not exist.
    #[inline]
    #[must_use]
    fn face_mut(&mut self, index: T::F) -> FaceCursorMut<'_, T> {
        FaceCursorMut::new(self, index)
    }

    /// Returns the number of faces in the mesh
    #[must_use]
    fn num_faces(&self) -> usize;

    /// Returns an iterator over all non-deleted faces
    #[must_use]
    fn face_refs<'a>(&'a self) -> impl Iterator<Item = &'a T::Face>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted face's ids
    #[inline]
    #[must_use]
    fn face_ids<'a>(&'a self) -> impl Iterator<Item = T::F>
    where
        T: 'a,
    {
        self.face_refs().map(|f| f.id())
    }

    /// Returns an iterator of cursors for each non-deleted face
    #[inline]
    #[must_use]
    fn faces<'a>(&'a self) -> impl Iterator<Item = FaceCursor<'a, T>>
    where
        T: 'a,
    {
        self.face_ids().map(move |id| FaceCursor::new(self, id))
    }

    /// If the mesh has exactly one face, returns a cursor to that face.
    /// Otherwise, returns a void cursor.
    #[inline]
    #[must_use]
    fn the_face(&self) -> FaceCursor<'_, T> {
        let mut fs = self.face_ids();
        let Some(f) = fs.next() else {
            return FaceCursor::new_void(self);
        };
        if fs.next().is_some() {
            FaceCursor::new_void(self)
        } else {
            FaceCursor::new(self, f)
        }
    }

    /// Returns an mutable iterator over all non-deleted vertices
    #[must_use]
    fn faces_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Face>
    where
        T: 'a;

    /// Returns the face shared by the two vertices or `None`.
    ///
    /// TODO: Currently cannot distinguish between holes and "the outside"
    #[must_use]
    fn shared_face(&self, v0: T::V, v1: T::V) -> Option<T::F>;

    //======================= Mesh Operations =======================//

    /// Whether the mesh is open, i.e., has boundary edges
    #[must_use]
    fn is_open(&self) -> bool;

    /// Returns the maximum vertex index in the mesh
    #[must_use]
    fn max_vertex_index(&self) -> usize;

    /// Clears the mesh (deletes all vertices, edges, and faces)
    fn clear(&mut self) -> &mut Self;

    /// Get the payload of the mesh
    #[must_use]
    fn payload(&self) -> &T::MP;

    /// Set the payload of the mesh
    fn set_payload(&mut self, payload: T::MP) -> &mut Self;

    /// Get a mutable reference to the payload of the mesh
    #[must_use]
    fn payload_mut(&mut self) -> &mut T::MP;

    /// Returns whether the vertex ids are consecutive, i.e., 0, 1, 2, 3, ...
    #[must_use]
    fn has_consecutive_vertex_ids(&self) -> bool {
        // TODO: We can check this more efficiently using the deleted vertices
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
    #[must_use]
    fn dense_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP>;

    /// Returns whether the mesh is connected, i.e., has only one connected component.
    #[must_use]
    fn is_connected(&self) -> bool {
        let mut seen = HashSet::<T::V>::new();
        let mut stack = Vec::<T::V>::new();
        if let Some(v) = self.vertex_refs().next() {
            stack.push(v.id());
            seen.insert(v.id());
        } else {
            return true;
        }
        while let Some(v) = stack.pop() {
            for e in self.vertex(v).edges_out() {
                let w = e.target_id();
                if !seen.contains(&w) {
                    stack.push(w);
                    seen.insert(w);
                }
            }
        }
        seen.len() == self.num_vertices()
    }

    /// Returns the connected components of the mesh.
    #[must_use]
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
                for e in self.vertex(v).edges_out() {
                    let w = e.target_id();
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
    #[must_use]
    fn submesh(&self, vertices: impl IntoIterator<Item = T::V>) -> Self {
        // TODO: test
        todo!("{:?}", vertices.into_iter().collect::<Vec<_>>())
    }

    /// Determines whether the mesh has holes, i.e., true boundary edges.
    #[must_use]
    fn has_holes(&self) -> bool {
        for e in self.edges() {
            if e.is_boundary() {
                return true;
            }
        }
        false
    }

    /// Determines whether the mesh is a collection of open 2-manifolds.
    /// The mesh might be empty or consist of multiple disconnected components where each one is an open 2-manifolds.
    ///
    /// This implies:
    /// - Each edge has between 1 and 2 incident faces.
    ///   This doesn't imply that the edge multiplicity is 1, since parallel edges are allowed
    ///   and will be summarized as one edge, i.e., you can have two edges with one face
    ///   each and a third one with no faces at all.
    /// - Each vertex has exactly one (half)disk of faces around it.
    ///
    /// This function is only well-defined for well-formed meshes.
    #[must_use]
    fn is_open_2manifold(&self) -> bool {
        for e in self.edges() {
            if !e.is_manifold() {
                return false;
            }
        }
        for v in self.vertices() {
            if !v.is_manifold() {
                return false;
            }
        }

        true
    }

    /// Determines whether the mesh is a collection of closed 2-manifolds.
    /// See [MeshBasics::is_open_2manifold] and [MeshBasics::has_holes] for details.
    #[must_use]
    fn is_2manifold(&self) -> bool {
        self.is_open_2manifold() && !self.has_holes()
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
