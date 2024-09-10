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
        let ep = Default::default();
        let fp = Default::default();
        let mut mesh = Mesh::<T>::new();
        let make = |x: T::S, y: T::S, z: T::S| T::VP::from_vec(T::Vec::from_xyz(x, y, z));
        let (v0, v1) = mesh.add_isolated_edge(make(x, y, z), ep, make(-x, y, z), ep);
        let v2 = mesh.add_vertex_auto(v1, make(-x, -y, z), ep, ep).0;
        let v3 = mesh.add_vertex_auto(v2, make(x, -y, z), ep, ep).0;
        mesh.close_face_vertices(v2, ep, v3, ep, v0, fp, false);
        let v4 = mesh.add_vertex_auto(v1, make(-x, y, -z), ep, ep).0;
        let v5 = mesh.add_vertex_auto(v4, make(-x, -y, -z), ep, ep).0;
        mesh.close_face_vertices(v4, ep, v5, ep, v2, fp, false);
        let v6 = mesh.add_vertex_auto(v0, make(x, y, -z), ep, ep).0;
        let v7 = mesh.add_vertex_auto(v3, make(x, -y, -z), ep, ep).0;
        mesh.close_face_vertices(v3, ep, v7, ep, v6, fp, false);
        mesh.close_face_vertices(v2, ep, v5, ep, v7, fp, false);
        mesh.close_face_vertices(v0, ep, v6, ep, v4, fp, false);
        mesh.close_hole(mesh.edge_between(v6, v7).unwrap().id(), fp, false);
        mesh
    }
}
