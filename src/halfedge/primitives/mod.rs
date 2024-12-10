use super::{HalfEdgeImplMeshType, HalfEdgeMeshImpl};
use crate::{
    mesh::{
        DefaultEdgePayload, DefaultFacePayload, EdgeBasics, EuclideanMeshType, FaceBasics,
        HalfEdge, MeshBasics, MeshBuilder, MeshPosition, MeshType3D, MeshTypeHalfEdge,
    },
    operations::{MeshExtrude, MeshLoft, MeshSubdivision},
    primitives::{Make2dShape, MakePlane, MakePrismatoid, MakeSphere},
};

impl<T: HalfEdgeImplMeshType<Mesh = Self>> Make2dShape<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    fn insert_polygon(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        // TODO: assertions
        let first = self.insert_loop(vp);
        self.close_hole(first, Default::default(), false);
        self.edge(first).twin_id()
    }

    fn insert_dihedron(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let first = self.insert_polygon(vp);
        self.close_hole(self.edge(first).twin_id(), Default::default(), false);
        first
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
