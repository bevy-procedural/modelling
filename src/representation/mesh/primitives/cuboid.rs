use super::super::{IndexType, Mesh};
use crate::representation::{
    builder::{AddVertex, CloseFace},
    payload::{Payload, Vector3D},
};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
    P::Vec: Vector3D<P::S>,
{
    /// create a (rectangular) cuboid
    pub fn cuboid(x: P::S, y: P::S, z: P::S) -> Mesh<E, V, F, P> {
        //assert!(P::dimensions() == 3, "cuboids exist only in 3d space");
        let mut mesh = Mesh::<E, V, F, P>::new();
        let make = |x: P::S, y: P::S, z: P::S| P::from_vec(P::Vec::from_xyz(x, y, z));
        let (v0, v1) = mesh.add_isolated_edge(make(x, y, z), make(-x, y, z));
        let v2 = mesh.add_vertex((v1, make(-x, -y, z))).0;
        let v3 = mesh.add_vertex((v2, make(x, -y, z))).0;
        mesh.close_face((v2, v3, v0));
        let v4 = mesh.add_vertex((v1, make(-x, y, -z))).0;
        let v5 = mesh.add_vertex((v4, make(-x, -y, -z))).0;
        mesh.close_face((v4, v5, v2));
        let v6 = mesh.add_vertex((v0, make(x, y, -z))).0;
        let v7 = mesh.add_vertex((v3, make(x, -y, -z))).0;
        mesh.close_face((v3, v7, v6));
        mesh.close_face((v2, v5, v7));
        mesh.close_face((v0, v6, v4));
        mesh.close_face(mesh.edge_between(v6, v7).unwrap().id());
        mesh
    }
}
