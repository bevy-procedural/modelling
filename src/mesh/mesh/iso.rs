use super::MeshType;
use crate::math::IndexType;
use std::collections::HashMap;

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

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    fn cuboid_from_vertices(size: Vec3<f64>) -> Mesh3d64 {
        fn vp(x: f64, y: f64, z: f64) -> VertexPayloadPNU<f64, 3> {
            VertexPayloadPNU::from_pos(Vec3::<f64>::new(x, y, z))
        }

        let (x, y, z) = (size * 0.5).tuple();
        let mut mesh = Mesh3d64::new();
        let (v0, v1) = mesh.add_isolated_edge_default(vp(x, y, z), vp(-x, y, z));
        let v2 = mesh.add_vertex_via_vertex_default(v1, vp(-x, -y, z)).0;
        let v3 = mesh.add_vertex_via_vertex_default(v2, vp(x, -y, z)).0;
        mesh.close_face_vertices_default(v2, v3, v0, false);
        let v4 = mesh.add_vertex_via_vertex_default(v1, vp(-x, y, -z)).0;
        let v5 = mesh.add_vertex_via_vertex_default(v4, vp(-x, -y, -z)).0;
        mesh.close_face_vertices_default(v4, v5, v2, false);
        let v6 = mesh.add_vertex_via_vertex_default(v0, vp(x, y, -z)).0;
        let v7 = mesh.add_vertex_via_vertex_default(v3, vp(x, -y, -z)).0;
        mesh.close_face_vertices_default(v3, v7, v6, false);
        mesh.close_face_vertices_default(v2, v5, v7, false);
        mesh.close_face_vertices_default(v0, v6, v4, false);
        mesh.close_hole_default(mesh.shared_edge(v6, v7).unwrap().id());
        mesh
    }

    #[test]
    fn cube_equivalence() {
        let cube = Mesh3d64::cube(1.0);
        let large_cube = Mesh3d64::cube(10.0);
        let mut flipped_cube = Mesh3d64::cube(1.0);
        flipped_cube.scale(&Vec3::new(1.0, -1.0, 1.0));
        let cube_by_vertices = cuboid_from_vertices(Vec3::new(1.0, 1.0, 1.0));
        let mut rotated_cube = cube.clone();
        rotated_cube.rotate(&NdRotate::from_axis_angle(Vec3::x_axis(), f64::PI));

        assert!(cube.is_trivially_isomorphic(&cube).eq(),);
        assert!(cube.is_trivially_isomorphic(&large_cube).eq());
        assert!(cube.is_trivially_isomorphic(&flipped_cube).eq());
        assert!(cube.is_trivially_isomorphic(&cube_by_vertices).ne());
        assert!(cube.is_trivially_isomorphic(&rotated_cube).eq());

        assert!(cube.is_equivalent_pos(&cube, 1e-6).eq());
        assert!(cube.is_equivalent_pos(&large_cube, 1e-6).ne());
        assert!(cube.is_equivalent_pos(&flipped_cube, 1e-6).ne());
        assert!(cube.is_equivalent_pos(&cube_by_vertices, 1e-6).ne());
        assert!(cube.is_equivalent_pos(&rotated_cube, 1e-6).ne());

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

        assert!(cube.is_isomorphic_by::<MT, _>(&cube, ps).eq());
        assert!(cube.is_isomorphic_by::<MT, _>(&large_cube, ps).ne());
        // not equal - faces are inside out
        assert!(cube.is_isomorphic_by::<MT, _>(&flipped_cube, ps).ne());
        assert!(cube.is_isomorphic_by::<MT, _>(&cube_by_vertices, ps).eq());
        assert!(cube.is_isomorphic_by::<MT, _>(&rotated_cube, ps).eq());
    }
}
