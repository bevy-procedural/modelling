use super::super::{IndexType, Mesh};
use crate::representation::payload::{Payload, Vector3D};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload + Vector3D<P::S>,
{
    /// create a (rectangular) cuboid
    #[allow(unused_variables)]
    pub fn cuboid(x: P::S, y: P::S, z: P::S) -> Mesh<E, V, F, P> {
        assert!(P::dimensions() == 3, "cuboids exist only in 3d space");
        let mut mesh = Mesh::<E, V, F, P>::from((P::from_xyz(x, y, z), P::from_xyz(-x, y, z)));

        let v0 = V::new(0);
        let v1 = V::new(1);
        let e01 = E::new(0);
        let e10 = E::new(1);
        let (v2, e12, e21) = mesh.add_vertex(e01, e10, P::from_xyz(-x, -y, z));
        let (v3, e23, e32) = mesh.add_vertex(e12, e21, P::from_xyz(x, -y, z));
        let (f1, e30, e03) = mesh.close_face(e23, e10);
        let (v4, e14, e41) = mesh.add_vertex(e21, e10, P::from_xyz(-x, y, -z));
        let (v5, e45, e54) = mesh.add_vertex(e14, e41, P::from_xyz(-x, -y, -z));
        let (f2, e52, e25) = mesh.close_face(e45, e32);
        let (v6, e06, e60) = mesh.add_vertex(e10, e03, P::from_xyz(x, y, -z));
        let (v7, e37, e73) = mesh.add_vertex(e03, e32, P::from_xyz(x, -y, -z));
        let (f3, e76, e67) = mesh.close_face(e37, e06);
        let (f4, e57, e75) = mesh.close_face(e25, e67);
        let (f5, e64, e46) = mesh.close_face(e06, e54);
        let f6 = mesh.close_final(e67);
        mesh
    }
}
