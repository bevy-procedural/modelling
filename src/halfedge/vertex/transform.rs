use super::HalfEdgeVertex;
use crate::{halfedge::HalfEdgeMeshType, math::Transformable, mesh::VertexBasics};

/*
impl<T: HalfEdgeMeshType> Transformable for HalfEdgeVertex<T>
where
    T::VP: Transformable<Trans = T::Trans, Rot = T::Rot, Vec = T::Vec, S = T::S>,
{
    type Rot = T::Rot;
    type S = T::S;
    type Trans = T::Trans;
    type Vec = T::Vec;

    #[inline(always)]
    fn transform(&mut self, transform: &T::Trans) {
        self.payload.transform(transform);
    }

    #[inline(always)]
    fn translate(&mut self, transform: &T::Vec) {
        self.payload.translate(transform);
    }

    #[inline(always)]
    fn rotate(&mut self, transform: &T::Rot) {
        self.payload.rotate(transform);
    }

    #[inline(always)]
    fn scale(&mut self, transform: &T::Vec) {
        self.payload.scale(transform);
    }

    #[inline(always)]
    fn lerp(&mut self, other: &Self, t: T::S) {
        self.payload.lerp(other.payload(), t);
    }
}
*/