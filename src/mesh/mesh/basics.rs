use crate::{
    math::{HasPosition, IndexType, Scalar, Vector},
    mesh::{
        CurvedEdge, CurvedEdgeType, DefaultEdgePayload, EdgeBasics, FaceBasics, MeshType,
        VertexBasics,
    },
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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

    /// Set the payload of the mesh
    fn set_payload(&mut self, payload: T::MP) -> &mut Self;

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
                self.edge_mut(*e).set_curve_type(CurvedEdgeType::Linear);
                if vs.len() == 0 {
                    continue;
                }
                self.insert_vertices_into_edge(
                    *e,
                    vs.iter()
                        .map(|v| (T::EP::default(), T::EP::default(), T::VP::from_pos(*v))),
                );
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

    /// Finds an edge isomorphism (if there is one) given a vertex isomorphism.
    ///
    /// Runs in O(e*d) where e is the number of edges and d is the maximum number of edges per vertex.
    fn find_edge_isomorphism<T2: MeshType, F: Fn(&T::Edge, &T2::Edge) -> bool>(
        &self,
        other: &T2::Mesh,
        iso: &IndexIsomorphism<T::V, T2::V>,
        compare_edge: F,
    ) -> Result<IndexIsomorphism<T::E, T2::E>, MeshEquivalenceDifference<T, T2>> {
        if self.num_edges() != other.num_edges() {
            return Err(MeshEquivalenceDifference::DifferentNumberOfEdges);
        }

        let mut edge_iso = IndexIsomorphism::<T::E, T2::E>::new();

        for v in self.vertices() {
            let other_v: &T2::Vertex = other.vertex(*iso.get(v.id()).unwrap());

            // Is there a corresponding edge?
            for e in v.edges_out(self) {
                if edge_iso.has(e.id()) {
                    continue;
                }

                let Some(other_e) =
                    other.shared_edge(other_v.id(), *iso.get(e.target(self).id()).unwrap())
                else {
                    return Err(MeshEquivalenceDifference::NoCorrespondingEdge(e.id()));
                };

                edge_iso.insert(e.id(), other_e.id());

                if !compare_edge(&e, &other_e) {
                    return Err(MeshEquivalenceDifference::DifferentEdges(
                        e.id(),
                        other_e.id(),
                    ));
                }
            }
        }

        Ok(edge_iso)
    }

    /// Finds a face isomorphism (if there is one) given an edge isomorphism.
    /// If `ignore_order` is true, the order of the edges in the face is ignored.
    ///
    /// Runs in O(e * fe) where
    ///  - e is the number of edges,
    ///  - fe is the maximum number of faces per edge, and
    fn find_face_isomorphism<T2: MeshType, F: Fn(&T::Face, &T2::Face) -> bool>(
        &self,
        other: &T2::Mesh,
        iso: &IndexIsomorphism<T::E, T2::E>,
        compare_face: F,
        ignore_order: bool,
    ) -> Result<IndexIsomorphism<T::F, T2::F>, MeshEquivalenceDifference<T, T2>> {
        if self.num_faces() != other.num_faces() {
            return Err(MeshEquivalenceDifference::DifferentNumberOfFaces);
        }

        let mut face_iso = IndexIsomorphism::<T::F, T2::F>::new();

        for face in self.face_ids() {
            assert!(!face_iso.has(face));

            // faces are the same if they have the same edges

            let es = self
                .face(face)
                .edge_ids(self)
                .map(|id| *iso.get(id).unwrap())
                .collect_vec();

            assert!(!es.is_empty());

            // check which face occurs in all corresponding edges.
            // This runs in O(e*fe) since all edges are checked once per incident face
            let mut other_faces: HashMap<T2::F, usize> =
                other.edge(es[0]).face_ids(&other).map(|f| (f, 1)).collect();
            for e in es.iter().skip(1) {
                for other_face in other.edge(*e).face_ids(&other) {
                    if let Some(count) = other_faces.get_mut(&other_face) {
                        *count += 1;
                    }
                }
            }

            // when the counts are equal, those are faces on the same edge set
            let num_edges = self.face(face).num_edges(self);
            let matches = other_faces
                .iter()
                .filter(|(_, count)| **count == num_edges)
                .map(|(f, _)| *f)
                .collect_vec();

            // when ignoring order, having a match is enough
            if ignore_order {
                if matches.is_empty() {
                    return Err(MeshEquivalenceDifference::NoCorrespondingFace(face));
                }
                if !compare_face(self.face(face), other.face(matches[0])) {
                    return Err(MeshEquivalenceDifference::DifferentFaces(face, matches[0]));
                }
                face_iso.insert(face, matches[0]);
                // we don't have to check for duplicate matches since otherwise the number faces wouldn't be equal
                continue;
            }

            // when caring about order, we have to check whether the sets can be rotated to match.
            // This runs in O(f*fe*ef) since each edge is compared to at most ef other edges 
            // where ef is the maximum number of edges per face. This simplifies to O(e*fe).
            for other_face in matches {
                let other_es = other.face(other_face).edge_ids(&other).collect_vec();
                if equal_up_to_rotation(&es, &other_es) {
                    if face_iso.has(face) {
                        // duplicate found
                        return Err(MeshEquivalenceDifference::DifferentFaces(face, other_face));
                    }
                    if !compare_face(self.face(face), other.face(other_face)) {
                        return Err(MeshEquivalenceDifference::DifferentFaces(face, other_face));
                    }
                    face_iso.insert(face, other_face);
                    // we don't have to check for duplicate matches since otherwise the number faces wouldn't be equal
                    break;
                }
            }

            if !face_iso.has(face) {
                return Err(MeshEquivalenceDifference::NoCorrespondingFace(face));
            }
        }

        Ok(face_iso)
    }

    /// Given two graphs and a vertex id isomorphism, check whether
    /// - the graphs have the same number of vertices, edges, and faces,
    /// - the corresponding vertices are adjacent in one mesh iff they are adjacent in the other mesh,
    /// - the faces have the same vertices up to rotation of the vertex list,
    /// - the three comparison functions hold for all pairs of corresponding vertices, edges, and faces.
    ///
    /// Can take up to O(e * (fe + d)) time where
    /// - e is the number of edges,
    /// - fe is the maximum number of faces per edge (usually 2),
    /// - d is the maximum number of edges per vertex.
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
        ignore_order: bool,
    ) -> MeshEquivalenceDifference<T, T2> {
        // TODO: how is this related to https://hackmd.io/@bo-JY945TOmvepQ1tAWy6w/SyuaFtay6

        if self.num_vertices() != other.num_vertices() {
            return MeshEquivalenceDifference::DifferentNumberOfVertices;
        }
        for v in self.vertices() {
            let other_v = other.vertex(*iso.get(v.id()).unwrap());
            if !compare_vertex(v, other_v) {
                return MeshEquivalenceDifference::DifferentVertices(v.id(), other_v.id());
            }
        }

        match self.find_edge_isomorphism::<T2, _>(other, iso, compare_edge) {
            Err(e) => e,
            Ok(edge_iso) => {
                if self
                    .find_face_isomorphism::<T2, _>(other, &edge_iso, compare_face, ignore_order)
                    .is_ok()
                {
                    MeshEquivalenceDifference::Equivalent
                } else {
                    MeshEquivalenceDifference::DifferentNumberOfFaces
                }
            }
        }
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
        self.is_isomorphic(other, &iso, |_, _| true, |_, _| true, |_, _| true, false)
    }

    /// Checks whether the two meshes are isomorphic with the same vertex positions.
    /// Ignores other payloads.
    fn is_isomorphic_by_pos<S: Scalar, const D: usize, Vec: Vector<S, D>, T2: MeshType>(
        &self,
        other: &T2::Mesh,
        eps: S,
    ) -> MeshEquivalenceDifference<T, T2>
    where
        T::VP: HasPosition<D, Vec, S = S>,
        T2::VP: HasPosition<D, Vec, S = S>,
    {
        self.is_isomorphic_by(other, |a: &T::Vertex, b: &T2::Vertex| {
            a.pos().is_about(&b.pos(), eps)
        })
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
