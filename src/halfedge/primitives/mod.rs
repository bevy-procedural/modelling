use super::{HalfEdgeImplMeshType, HalfEdgeImplMeshTypePlus, HalfEdgeMeshImpl};
use crate::{
    mesh::{
        cursor::*, DefaultEdgePayload, DefaultFacePayload, EdgeBasics, EuclideanMeshType,
        FaceBasics, HalfEdge, MeshBuilder, MeshPosition, MeshType3D, MeshTypeHalfEdge,
    },
    operations::{DebuggingConstruction, MeshExtrude, MeshLoft, MeshSubdivision},
    primitives::{Make2dShape, MakePlane, MakePrismatoid, MakeSphere},
};

impl<T: HalfEdgeImplMeshTypePlus<Mesh = Self>> Make2dShape<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    #[inline]
    fn insert_polygon(&mut self, vp: impl IntoIterator<Item = T::VP>) -> ValidEdgeCursorMut<'_, T> {
        self.insert_loop_default(vp)
            .stay(|c| {
                c.twin()
                    .unwrap()
                    .insert_face(Default::default())
                    .edge()
                    .unwrap()
            })
            .unwrap()
    }

    #[inline]
    fn insert_dihedron(
        &mut self,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> ValidEdgeCursorMut<'_, T> {
        self.insert_polygon(vp)
            .insert_face(Default::default())
            .edge()
            .unwrap()
    }
}

impl<const D: usize, T: HalfEdgeImplMeshType + EuclideanMeshType<D>> MakePlane<D, T>
    for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge + MeshType3D> MakePrismatoid<T>
    for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    Self: Make2dShape<T>,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge + MeshType3D> MakeSphere<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    Self: Make2dShape<T>,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge> DebuggingConstruction<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge> MeshSubdivision<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::Face: FaceBasics<T>,
    T::Edge: HalfEdge<T> + EdgeBasics<T>,
{
}

impl<const D: usize, T: HalfEdgeImplMeshType + EuclideanMeshType<D>> MeshPosition<D, T>
    for HalfEdgeMeshImpl<T>
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge> MeshExtrude<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge> MeshLoft<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
}
