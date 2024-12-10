use crate::{
    halfedge::{HalfEdgeImplMeshType, HalfEdgeMeshImpl},
    math::{HasPosition, Vector3D},
    mesh::{MeshBasics, MeshType3D, VertexBasics},
};

// TODO: Where to place this function?

impl<T: HalfEdgeImplMeshType + MeshType3D> HalfEdgeMeshImpl<T> {
    /// Flips the y and z coordinates of all vertices.
    pub fn flip_yz(&mut self) -> &mut Self {
        self.vertices_mut().for_each(|v| {
            let pos = v.payload().pos().xzy();
            v.payload_mut().set_pos(pos)
        });
        self
    }
}
