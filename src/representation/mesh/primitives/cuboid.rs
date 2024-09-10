use crate::{
    math::Vector,
    representation::{
        payload::VertexPayload, DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// create a (rectangular) cuboid
    pub fn cuboid(x: T::S, y: T::S, z: T::S) -> Mesh<T> {
        //assert!(P::dimensions() == 3, "cuboids exist only in 3d space");

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
            mesh.edge_between(v6, v7).unwrap().id(),
            Default::default(),
            false,
        );
        mesh
    }
}
