use crate::{
    math::{HasPosition, IndexType, Scalar, Vector},
    mesh::{
        CursorData, CurvedEdge, EdgeBasics, EdgeCursorBasics, EuclideanMeshType, FaceBasics,
        FaceCursorBasics, MeshBasics, MeshType, VertexBasics,
    },
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// The difference between two meshes when comparing them for equivalence.
#[derive(Clone, Debug, PartialEq)]
pub enum MeshEquivalenceDifference<T1: MeshType, T2: MeshType> {
    /// The meshes are equivalent
    Equivalent,
    /// The meshes have a different number of vertices
    DifferentNumberOfVertices,
    /// The meshes have a different number of edges
    DifferentNumberOfEdges,
    /// The meshes have a different number of faces
    DifferentNumberOfFaces,
    /// The meshes have different vertices at the given indices according to the comparison function
    DifferentVertices(T1::V, T2::V),
    /// The meshes have different edges at the given indices according to the comparison function
    DifferentEdges(T1::E, T2::E),
    /// The meshes have different faces at the given indices according to the comparison function
    DifferentFaces(T1::F, T2::F),
    /// The meshes have no corresponding vertex for the given vertex
    NoCorrespondingVertex,
    /// The meshes have no corresponding edge for the given edge
    NoCorrespondingEdge(T1::E),
    /// The meshes have no corresponding face for the given face
    NoCorrespondingFace(T1::F),
}

impl<T1: MeshType, T2: MeshType> MeshEquivalenceDifference<T1, T2> {
    /// Whether the meshes are equivalent
    pub fn eq(&self) -> bool {
        matches!(self, MeshEquivalenceDifference::Equivalent)
    }

    /// Whether the meshes are not equivalent
    pub fn ne(&self) -> bool {
        !self.eq()
    }
}

/// A simple isomorphism between two sets of indices.
#[derive(Clone, Debug)]
pub struct IndexIsomorphism<V1: IndexType, V2: IndexType> {
    map: HashMap<V1, V2>,
}

impl<V1: IndexType, V2: IndexType> IndexIsomorphism<V1, V2> {
    /// Creates a new empty isomorphism
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Returns whether the isomorphism contains the given index
    pub fn has(&self, v1: V1) -> bool {
        self.map.contains_key(&v1)
    }

    /// Inserts a new isomorphism
    pub fn insert(&mut self, v1: V1, v2: V2) {
        self.map.insert(v1, v2);
    }

    /// Returns the isomorphism
    pub fn get(&self, v1: V1) -> Option<&V2> {
        self.map.get(&v1)
    }

    /// Iterates over all pairs in the isomorphism
    pub fn iter(&self) -> impl Iterator<Item = (&V1, &V2)> {
        self.map.iter()
    }
}

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

/// A trait for checking isomorphisms between meshes.
pub trait MeshIsomorphism<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Finds an edge isomorphism (if there is one) given a vertex isomorphism.
    ///
    /// Assumes there is at most one edge between each two vertices.
    ///
    /// Runs in O(e*d) where e is the number of edges and d is the maximum number of edges per vertex.
    fn find_edge_isomorphism<T2: MeshType, F: Fn(&T::Edge, &T2::Edge) -> bool>(
        &self,
        other: &T2::Mesh,
        iso: &IndexIsomorphism<T::V, T2::V>,
        compare_edge: F,
    ) -> Result<IndexIsomorphism<T::E, T2::E>, MeshEquivalenceDifference<T, T2>> {
        // TODO: assert max number of edges per directed vertex pair is 1

        if self.num_edges() != other.num_edges() {
            return Err(MeshEquivalenceDifference::DifferentNumberOfEdges);
        }

        let mut edge_iso = IndexIsomorphism::<T::E, T2::E>::new();

        for v in self.vertices() {
            let other_v = other.vertex(*iso.get(v.id()).unwrap());

            // Is there a corresponding edge?
            for e in v.edges_out(self) {
                if edge_iso.has(e) {
                    continue;
                }

                let Some(other_e) =
                    other.shared_edge(other_v.id(), *iso.get(self.edge(e).target_id()).unwrap())
                else {
                    return Err(MeshEquivalenceDifference::NoCorrespondingEdge(e));
                };

                edge_iso.insert(e, other_e.id());

                if !compare_edge(self.edge_ref(e), &other_e) {
                    return Err(MeshEquivalenceDifference::DifferentEdges(e, other_e.id()));
                }
            }
        }

        Ok(edge_iso)
    }

    /// Finds a face isomorphism (if there is one) given an edge isomorphism.
    /// If `ignore_order` is true, the order of the edges in the face is ignored.
    ///
    /// Assumes there is at most one face for each directed cycle of edges.
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
        // TODO: assert max number of faces per directed cycle of edges is 1

        if self.num_faces() != other.num_faces() {
            return Err(MeshEquivalenceDifference::DifferentNumberOfFaces);
        }

        let mut face_iso = IndexIsomorphism::<T::F, T2::F>::new();

        for face in self.face_ids() {
            assert!(!face_iso.has(face));

            // faces are the same if they have the same edges

            let es = self
                .face(face)
                .edge_ids()
                .map(|id| *iso.get(id).unwrap())
                .collect_vec();

            assert!(!es.is_empty());

            // check which face occurs in all corresponding edges.
            // This runs in O(e*fe) since all edges are checked once per incident face
            let mut other_faces: HashMap<T2::F, usize> =
                other.edge(es[0]).face_ids().map(|f| (f, 1)).collect();
            for e in es.iter().skip(1) {
                for other_face in other.edge(*e).face_ids() {
                    if let Some(count) = other_faces.get_mut(&other_face) {
                        *count += 1;
                    }
                }
            }

            // when the counts are equal, those are faces on the same edge set
            let num_edges = self.face(face).num_edges();
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
                if !compare_face(self.face_ref(face), other.face_ref(matches[0])) {
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
                let other_es = other.face(other_face).edge_ids().collect_vec();
                if equal_up_to_rotation(&es, &other_es) {
                    if face_iso.has(face) {
                        // duplicate found
                        return Err(MeshEquivalenceDifference::DifferentFaces(face, other_face));
                    }
                    if !compare_face(self.face_ref(face), other.face_ref(other_face)) {
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
    /// Assumes there is at most one edge between each two vertices
    /// and at most one face for each directed cycle of edges.
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
            let other_v = other.vertex_ref(*iso.get(v.id()).unwrap());
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
    ///
    /// Assumes there is at most one edge between each two vertices
    /// and at most one face for each directed cycle of edges.
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
            let other_v = other.vertex_ref(v.id());
            if v.id() != other_v.id() || !compare_vertex(v, other_v) {
                return MeshEquivalenceDifference::DifferentVertices(v.id(), other_v.id());
            }
        }

        for e in self.edges() {
            let other_e = other.edge(e.id());
            if e.id() != other_e.id()
                || e.origin(self).id() != other_e.origin_id()
                || e.target(self).id() != other_e.target_id()
                || !compare_edge(e, other_e.unwrap())
            {
                return MeshEquivalenceDifference::DifferentEdges(e.id(), other_e.id());
            }
        }

        for f in self.face_refs() {
            let other_f = other.face_ref(f.id());
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
            |a, b| a.curve_type(self).is_about(&b.curve_type(self), eps),
            |_, _| true,
        )
    }
}

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    fn cuboid_from_vertices(size: Vec3<f64>) -> Option<Mesh3d64> {
        fn vp(x: f64, y: f64, z: f64) -> VertexPayloadPNU<f64, 3> {
            VertexPayloadPNU::from_pos(Vec3::<f64>::new(x, y, z))
        }

        let (x, y, z) = (size * 0.5).tuple();
        let mut mesh = Mesh3d64::new();
        //let (v0, v1) = mesh.insert_isolated_edge(vp(x, y, z), vp(-x, y, z));
        let v0 = mesh.insert_vertex(vp(x, y, z));
        let v1 = mesh.insert_vertex(vp(-x, y, z));
        mesh.insert_edge_vv(v0, v1, Default::default())?;
        let (_, v2) = mesh.insert_vertex_v(v1, vp(-x, -y, z), Default::default())?;
        let (_, v3) = mesh.insert_vertex_v(v2, vp(x, -y, z), Default::default())?;
        mesh.close_face_vvv(v2, v3, v0, Default::default(), Default::default())?;
        let (_, v4) = mesh.insert_vertex_v(v1, vp(-x, y, -z), Default::default())?;
        let (_, v5) = mesh.insert_vertex_v(v4, vp(-x, -y, -z), Default::default())?;
        mesh.close_face_vvv(v4, v5, v2, Default::default(), Default::default())?;
        let (_, v6) = mesh.insert_vertex_v(v0, vp(x, y, -z), Default::default())?;
        let (_, v7) = mesh.insert_vertex_v(v3, vp(x, -y, -z), Default::default())?;
        mesh.close_face_vvv(v3, v7, v6, Default::default(), Default::default())?;
        mesh.close_face_vvv(v2, v5, v7, Default::default(), Default::default())?;
        mesh.close_face_vvv(v0, v6, v4, Default::default(), Default::default())?;
        mesh.insert_face(mesh.shared_edge(v6, v7)?.id(), Default::default())?;
        Some(mesh)
    }

    #[test]
    fn cube_equivalence() {
        let cube = Mesh3d64::cube(1.0);
        let large_cube = Mesh3d64::cube(10.0);
        let mut flipped_cube = Mesh3d64::cube(1.0);
        flipped_cube.scale(&Vec3::new(1.0, -1.0, 1.0));
        let cube_by_vertices = cuboid_from_vertices(Vec3::new(1.0, 1.0, 1.0)).unwrap();
        let mut rotated_cube = cube.clone();
        rotated_cube.rotate(&NdRotate::from_axis_angle(Vec3::x_axis(), f64::PI));

        assert!(cube.is_trivially_isomorphic(&cube).eq(),);
        assert!(cube.is_trivially_isomorphic(&large_cube).eq());
        assert!(cube.is_trivially_isomorphic(&flipped_cube).eq());
        assert!(cube.is_trivially_isomorphic(&cube_by_vertices).ne());
        assert!(cube.is_trivially_isomorphic(&rotated_cube).eq());

        assert!(cube.is_trivially_isomorphic_pos(&cube, 1e-6).eq());
        assert!(cube.is_trivially_isomorphic_pos(&large_cube, 1e-6).ne());
        assert!(cube.is_trivially_isomorphic_pos(&flipped_cube, 1e-6).ne());
        assert!(cube
            .is_trivially_isomorphic_pos(&cube_by_vertices, 1e-6)
            .ne());
        assert!(cube.is_trivially_isomorphic_pos(&rotated_cube, 1e-6).ne());

        let ps = |a: &Mesh3d64Vertex, b: &Mesh3d64Vertex| a.pos().is_about(&b.pos(), 1e-6);

        type MT = MeshType3d64PNU;

        assert!(cube.find_payload_isomorphism::<MT, _>(&cube, ps).is_ok());
        assert!(cube
            .find_payload_isomorphism::<MT, _>(&large_cube, ps)
            .is_err());
        assert!(cube
            .find_payload_isomorphism::<MT, _>(&flipped_cube, ps)
            .is_ok());
        assert!(cube
            .find_payload_isomorphism::<MT, _>(&cube_by_vertices, ps)
            .is_ok());
        assert!(cube
            .find_payload_isomorphism::<MT, _>(&rotated_cube, ps)
            .is_ok());

        assert!(cube.is_isomorphic_by_pos::<_, 3, _, MT>(&cube, 1e-6).eq());
        assert!(cube
            .is_isomorphic_by_pos::<_, 3, _, MT>(&large_cube, 1e-6)
            .ne());
        // not equal - faces are inside out
        assert!(cube
            .is_isomorphic_by_pos::<_, 3, _, MT>(&flipped_cube, 1e-6)
            .ne());
        assert!(cube
            .is_isomorphic_by_pos::<_, 3, _, MT>(&cube_by_vertices, 1e-6)
            .eq());
        assert!(cube
            .is_isomorphic_by_pos::<_, 3, _, MT>(&rotated_cube, 1e-6)
            .eq());
    }
}
