use crate::{
    math::{HasPosition, IndexType, Scalar, Vector},
    mesh::{
        CurvedEdge, CurvedEdgeType, DefaultEdgePayload, EdgeBasics, FaceBasics, MeshType,
        VertexBasics,
    },
};
use itertools::Itertools;
use std::collections::HashSet;

use super::{EuclideanMeshType, IndexIsomorphism, MeshBuilder, MeshEquivalenceDifference};

fn equal_up_to_rotation<T: Clone + PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[0] {
            continue;
        }
        if a.iter().cycle().skip(i).take(a.len()).eq(b.iter()) {
            return true;
        }
    }
    false
}

/// Some basic operations to retrieve information about the mesh.
pub trait MeshBasics<T: MeshType<Mesh = Self>>: Default + std::fmt::Debug + Clone {
    /// Returns whether the vertex exists and is not deleted
    fn has_vertex(&self, index: T::V) -> bool;

    /// Returns a reference to the requested vertex
    fn vertex(&self, index: T::V) -> &T::Vertex;

    /// Returns a reference to the requested edge
    fn edge(&self, index: T::E) -> &T::Edge;

    /// Returns a reference to the requested face
    fn face(&self, index: T::F) -> &T::Face;

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

    /// Get a mutable reference to the payload of the mesh
    fn payload_mut(&mut self) -> &mut T::MP;

    /// Since the vertex payloads in the `Deletable` can be sparse,
    /// we need to compact the vertices when converting them to a dense vector.
    /// This function returns the cloned compact vertices and maps the indices to the new compact buffer.
    fn dense_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP>;

    /// Returns an iterator over all non-deleted vertices
    fn vertices<'a>(&'a self) -> impl Iterator<Item = &'a T::Vertex>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted vertice's ids
    fn vertex_ids<'a>(&'a self) -> impl Iterator<Item = T::V>
    where
        T: 'a,
        T::Face: 'a,
    {
        self.vertices().map(|v| v.id())
    }

    /// Returns an mutable iterator over all non-deleted vertices
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T: 'a;

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
    fn shared_edge(&self, v: T::V, w: T::V) -> Option<T::Edge>;

    /// Returns the (half)edge id from v to w. Panics if the edge does not exist.
    fn shared_edge_id(&self, v: T::V, w: T::V) -> Option<T::E>;

    /// Returns the face shared by the two vertices or `None`.
    /// 
    /// TODO: Currently cannot distinguish between holes and "the outside"
    fn shared_face(&self, v0: T::V, v1: T::V) -> Option<T::F>;

    /// Converts the mesh to a mesh without curved edges
    fn flatten_curved_edges<const D: usize>(&mut self, tol: T::S) -> &mut Self
    where
        T::Edge: CurvedEdge<D, T>,
        T::EP: DefaultEdgePayload,
        T: EuclideanMeshType<D>,
        T::VP: HasPosition<D, T::Vec>,
        T::Mesh: MeshBuilder<T>,
    {
        // TODO: assert that T::EP::default() is a linear edge

        // Convert curved edges
        for e in self.edge_ids().collect::<Vec<_>>().iter() {
            let edge = self.edge(*e);
            if edge.curve_type() != CurvedEdgeType::Linear {
                let vs = edge.flatten_casteljau(tol, self);
                if vs.len() == 0 {
                    continue;
                }
                self.insert_vertices_into_edge(
                    *e,
                    vs.iter()
                        .map(|v| (T::EP::default(), T::EP::default(), T::VP::from_pos(*v))),
                );
                self.edge_mut(*e).set_curve_type(CurvedEdgeType::Linear);
            }
        }

        self
    }

    /// Returns whether the mesh has curved edges
    fn has_curved_edges<const D: usize>(&self) -> bool
    where
        T::Edge: CurvedEdge<D, T>,
        T: EuclideanMeshType<D>,
        T::VP: HasPosition<D, T::Vec>,
    {
        self.edges()
            .any(|e| e.curve_type() != CurvedEdgeType::Linear)
    }


    /// Given two graphs and a vertex id isomorphism, check whether
    /// - the graphs have the same number of vertices, edges, and faces,
    /// - the corresponding vertices are adjacent in one mesh iff they are adjacent in the other mesh,
    /// - the faces have the same vertices up to rotation of the vertex list,
    /// - the three comparison functions hold for all pairs of corresponding vertices, edges, and faces.
    /// 
    /// Can take up to O(n^2) time.
    /// 
    /// TODO: Split this into is_vertex/edge/face_isomorphic and make each of them return the induced isomorphism
    fn is_isomorphic<
        T2: MeshType,
        F1: Fn(&T::Vertex, &T2::Vertex) -> bool,
        F2: Fn(&T::Edge, &T2::Edge) -> bool,
        F3: Fn(&T::Face, &T2::Face) -> bool,
    >(
        &self,
        other: &T2::Mesh,
        iso: &IndexIsomorphism<T::V, T2::V>,
        compare_vertex: F1,
        compare_edge: F2,
        compare_face: F3,
    ) -> MeshEquivalenceDifference<T, T2> {
        // TODO: how is this related to https://hackmd.io/@bo-JY945TOmvepQ1tAWy6w/SyuaFtay6
        
        if self.num_vertices() != other.num_vertices() {
            return MeshEquivalenceDifference::DifferentNumberOfVertices;
        }
        if self.num_edges() != other.num_edges() {
            return MeshEquivalenceDifference::DifferentNumberOfEdges;
        }
        if self.num_faces() != other.num_faces() {
            return MeshEquivalenceDifference::DifferentNumberOfFaces;
        }

        let mut edge_iso = IndexIsomorphism::<T::E, T2::E>::new();
        let mut face_iso = IndexIsomorphism::<T::F, T2::F>::new();

        for v in self.vertices() {
            let other_v: &T2::Vertex = other.vertex(*iso.get(v.id()).unwrap());
            if !compare_vertex(v, other_v) {
                return MeshEquivalenceDifference::DifferentVertices(v.id(), other_v.id());
            }

            // Is there a corresponding edge?
            for e in v.edges_out(self) {
                if edge_iso.has(e.id()) {
                    continue;
                }

                let Some(other_e) =
                    other.shared_edge(other_v.id(), *iso.get(e.target(self).id()).unwrap())
                else {
                    return MeshEquivalenceDifference::NoCorrespondingEdge(e.id());
                };

                edge_iso.insert(e.id(), other_e.id());

                if !compare_edge(&e, &other_e) {
                    return MeshEquivalenceDifference::DifferentEdges(e.id(), other_e.id());
                }

                let other_faces = other_e
                    .face_ids(other)
                    .map(|f| (f, other.face(f).vertex_ids(other).collect_vec()))
                    .collect_vec();

                for face in e.face_ids(self) {
                    if face_iso.has(face) {
                        continue;
                    }

                    let vs = self
                        .face(face)
                        .vertex_ids(self)
                        .map(|id| *iso.get(id).unwrap())
                        .collect_vec();
                    for (other_f, other_vs) in other_faces.iter() {
                        // faces are the same if they have the same vertices
                        if equal_up_to_rotation(&vs, other_vs) {
                            face_iso.insert(face, *other_f);

                            if !compare_face(self.face(face), other.face(*other_f)) {
                                return MeshEquivalenceDifference::DifferentFaces(face, *other_f);
                            }
                            break;
                        }
                    }

                    if !face_iso.has(face) {
                        return MeshEquivalenceDifference::NoCorrespondingFace(face);
                    }
                }
            }
        }

        MeshEquivalenceDifference::Equivalent
    }

    /// `is_isomorphic` for the vertex isomorphism based on the given similarity metric.
    /// Ignoring all payloads.
    fn is_isomorphic_by<T2: MeshType, F: Fn(&T::Vertex, &T2::Vertex) -> bool>(
        &self,
        other: &T2::Mesh,
        f: F,
    ) -> MeshEquivalenceDifference<T, T2> {
        let Ok(iso) = self.find_payload_isomorphism::<T2, F>(other, f) else {
            return MeshEquivalenceDifference::NoCorrespondingVertex;
        };
        self.is_isomorphic(other, &iso, |_, _| true, |_, _| true, |_, _| true)
    }

    /// Returns whether the two meshes are equal, i.e., when treating vertices, edges,
    /// and faces with the same ids as corresponding to each other has the same connectivity.
    /// Does care about the order and direction in which edges are connected to a vertex,
    /// but not where the edge wheel starts.
    fn is_trivially_isomorphic_ex<
        F1: Fn(&T::Vertex, &T::Vertex) -> bool,
        F2: Fn(&T::Edge, &T::Edge) -> bool,
        F3: Fn(&T::Face, &T::Face) -> bool,
    >(
        &self,
        other: &Self,
        compare_vertex: F1,
        compare_edge: F2,
        compare_face: F3,
    ) -> MeshEquivalenceDifference<T, T> {
        if self.num_vertices() != other.num_vertices() {
            return MeshEquivalenceDifference::DifferentNumberOfVertices;
        }
        if self.num_edges() != other.num_edges() {
            return MeshEquivalenceDifference::DifferentNumberOfEdges;
        }
        if self.num_faces() != other.num_faces() {
            return MeshEquivalenceDifference::DifferentNumberOfFaces;
        }

        for v in self.vertices() {
            let other_v = other.vertex(v.id());
            if v.id() != other_v.id() || !compare_vertex(v, other_v) {
                return MeshEquivalenceDifference::DifferentVertices(v.id(), other_v.id());
            }
        }

        for e in self.edges() {
            let other_e = other.edge(e.id());
            if e.id() != other_e.id()
                || e.origin(self).id() != other_e.origin(self).id()
                || e.target(self).id() != other_e.target(self).id()
                || !compare_edge(e, other_e)
            {
                return MeshEquivalenceDifference::DifferentEdges(e.id(), other_e.id());
            }
        }

        for f in self.faces() {
            let other_f = other.face(f.id());
            if f.id() != other_f.id()
                || f.num_vertices(self) != other_f.num_vertices(self)
                || !equal_up_to_rotation(
                    &f.edge_ids(self).collect_vec(),
                    &other_f.edge_ids(self).collect_vec(),
                )
                || !compare_face(f, other_f)
            {
                return MeshEquivalenceDifference::DifferentFaces(f.id(), other_f.id());
            }
        }

        MeshEquivalenceDifference::Equivalent
    }

    /// `is_equivalent_ex` ignoring all payloads and using the trivial isomorphism.
    fn is_trivially_isomorphic(&self, other: &Self) -> MeshEquivalenceDifference<T, T> {
        self.is_trivially_isomorphic_ex(other, |_, _| true, |_, _| true, |_, _| true)
    }

    /// Whether the graphs have the same connectivity, ids, and vertex positions.
    /// We use the trivial isomorphism for vertices and edges, i.e., the same vertices
    /// should have the same ids and the same edges should have the same ids.
    /// Ignores other payloads. A special case of `is_equivalent`.
    fn is_trivially_isomorphic_pos<const D: usize, S: Scalar, Vec: Vector<S, D>>(
        &self,
        other: &Self,
        eps: S,
    ) -> MeshEquivalenceDifference<T, T>
    where
        T::VP: HasPosition<D, Vec, S = S>,
    {
        self.is_trivially_isomorphic_ex(
            other,
            |a, b| a.pos().is_about(&b.pos(), eps),
            |_, _| true,
            |_, _| true,
        )
    }

    /// Will construct an isomorphism between the vertices of two meshes
    /// based on a similarity metric (e.g., vertex position distance) of the vertices.
    /// This will not work if the vertices are not unique and isomorphic w.r.t. the metric.
    /// Runs in O(n^2) time.
    fn find_payload_isomorphism<T2: MeshType, F: Fn(&T::Vertex, &T2::Vertex) -> bool>(
        &self,
        other: &T2::Mesh,
        are_the_same: F,
    ) -> Result<IndexIsomorphism<T::V, T2::V>, String> {
        let mut res = IndexIsomorphism::new();
        let mut used = HashSet::new();
        for v in self.vertices() {
            let Some(best_match) = other.vertices().find(|v2| are_the_same(v, v2)) else {
                return Err(format!("No match found for vertex {:?}", v.id()));
            };
            if res.has(v.id()) {
                return Err(format!("Vertex {:?} is not unique", v.id()));
            }
            res.insert(v.id(), best_match.id());
            if used.contains(&best_match.id()) {
                return Err(format!("Vertex {:?} is not unique", best_match.id()));
            }
            used.insert(best_match.id());
        }
        Ok(res)
    }

    /// Whether the graphs have the same connectivity and vertex positions.
    /// The vertex positions are used to canonicalize the vertex ids. Based on
    /// this, we can quickly check if the graphs are isomorphic. This won't work
    /// if some vertex positions have a smaller or equal distance than `eps`.
    fn is_isomorphic_pos<const D: usize, S: Scalar, Vec: Vector<S, D>>(
        &self,
        other: &Self,
        eps: S,
    ) -> MeshEquivalenceDifference<T, T>
    where
        T::VP: HasPosition<D, Vec, S = S>,
    {
        self.is_trivially_isomorphic_ex(
            other,
            |a, b| a.pos().is_about(&b.pos(), eps),
            |_, _| true,
            |_, _| true,
        )
    }

    /// Same as `is_trivially_isomorphic_pos`, but also checks control points of curved edges.
    fn is_trivially_isomorphic_pos_curved<const D: usize>(
        &self,
        other: &Self,
        eps: T::S,
    ) -> MeshEquivalenceDifference<T, T>
    where
        T::VP: HasPosition<D, T::Vec, S = T::S>,
        T::Edge: CurvedEdge<D, T>,
        T: EuclideanMeshType<D>,
    {
        self.is_trivially_isomorphic_ex(
            other,
            |a, b| a.pos().is_about(&b.pos(), eps),
            |a, b| a.curve_type().is_about(&b.curve_type(), eps),
            |_, _| true,
        )
    }
}
