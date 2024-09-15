use crate::{
    math::{Vector, Vector3D, Scalar},
    representation::{
        payload::HasPosition, DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// create a (rectangular) cuboid with side lengths `x`, `y`, and `z`
    pub fn cuboid(size: T::Vec3) -> Mesh<T> {
        //assert!(P::dimensions() == 3, "cuboids exist only in 3d space");

        let (x, y, z) = (size * T::S::HALF).tuple();

        // TODO: use the loft function!
        // Move this to the box example; demonstrate different builder techniques. This is just a prismatoid.

        let mut mesh = Mesh::<T>::new();
        let make = |x: T::S, y: T::S, z: T::S| T::VP::from_pos(T::Vec::from_xyz(x, y, z));
        let (v0, v1) = mesh.add_isolated_edge_default(make(x, y, z), make(-x, y, z));
        let v2 = mesh.add_vertex_via_vertex_default(v1, make(-x, -y, z)).0;
        let v3 = mesh.add_vertex_via_vertex_default(v2, make(x, -y, z)).0;
        mesh.close_face_vertices_default(v2, v3, v0, false);
        let v4 = mesh.add_vertex_via_vertex_default(v1, make(-x, y, -z)).0;
        let v5 = mesh.add_vertex_via_vertex_default(v4, make(-x, -y, -z)).0;
        mesh.close_face_vertices_default(v4, v5, v2, false);
        let v6 = mesh.add_vertex_via_vertex_default(v0, make(x, y, -z)).0;
        let v7 = mesh.add_vertex_via_vertex_default(v3, make(x, -y, -z)).0;
        mesh.close_face_vertices_default(v3, v7, v6, false);
        mesh.close_face_vertices_default(v2, v5, v7, false);
        mesh.close_face_vertices_default(v0, v6, v4, false);
        mesh.close_hole(
            mesh.shared_edge(v6, v7).unwrap().id(),
            Default::default(),
            false,
        );
        mesh
    }

    /// create a cube with side length `x`
    pub fn cube(x: T::S) -> Mesh<T> {
        Self::cuboid(T::Vec3::splat(x))
    }
}
