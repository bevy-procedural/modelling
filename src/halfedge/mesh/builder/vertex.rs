use crate::{
    halfedge::{HalfEdgeMeshImpl, HalfEdgeImplMeshType},
    math::{HasPosition, Vector3D},
    mesh::{MeshBasics, VertexBasics},
};

// TODO: Where to place this function?

impl<T: HalfEdgeImplMeshType> HalfEdgeMeshImpl<T>
where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Flips the y and z coordinates of all vertices.
    pub fn flip_yz(&mut self) -> &mut Self {
        self.vertices_mut().for_each(|v| {
            let pos = v.payload().pos().xzy();
            v.payload_mut().set_pos(pos)
        });
        self
    }
}
