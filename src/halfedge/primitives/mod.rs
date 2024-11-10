use super::{HalfEdgeImplMeshType, HalfEdgeMeshImpl};
use crate::{
    math::{HasPosition, TransformTrait, Transformable, Vector3D},
    mesh::{
        DefaultEdgePayload, DefaultFacePayload, EdgeBasics, Face3d, FaceBasics, HalfEdge,
        MeshTypeHalfEdge, MeshBasics, MeshBuilder, MeshPathBuilder, MeshPosition,
        MeshType3D,
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

impl<T: HalfEdgeImplMeshType> MakePlane<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge + MeshType3D> MakePrismatoid<T>
    for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::Vec: Vector3D<S = T::S>,
    T::VP: Transformable<Vec = T::Vec, Rot = T::Rot, Trans = T::Trans, S = T::S>
        + HasPosition<T::Vec, S = T::S>,
    Self: Make2dShape<T>,
    T::Edge: HalfEdge<T> + EdgeBasics<T>,
    T::Face: Face3d<T>,
    T::Trans: TransformTrait<S = T::S>,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge + MeshType3D> MakeSphere<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::Vec: Vector3D<S = T::S>,
    T::VP: Transformable<Vec = T::Vec, Rot = T::Rot, Trans = T::Trans, S = T::S>
        + HasPosition<T::Vec, S = T::S>,
    Self: Make2dShape<T>,
    T::Edge: HalfEdge<T> + EdgeBasics<T>,
    T::Face: Face3d<T>,
    T::Trans: TransformTrait<S = T::S>,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge> MeshSubdivision<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::Face: FaceBasics<T>,
    T::Edge: HalfEdge<T> + EdgeBasics<T>,
{
}

impl<T: HalfEdgeImplMeshType> MeshPosition<T> for HalfEdgeMeshImpl<T> where
    T::VP: HasPosition<T::Vec, S = T::S>
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge> MeshExtrude<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: Transformable<Trans = T::Trans, S = T::S>,
    T::Edge: HalfEdge<T> + EdgeBasics<T>,
{
}

impl<T: HalfEdgeImplMeshType + MeshTypeHalfEdge> MeshLoft<T> for HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: Transformable<Trans = T::Trans, S = T::S>,
    T::Edge: HalfEdge<T> + EdgeBasics<T>,
{
}
