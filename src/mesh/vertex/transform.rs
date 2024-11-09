use super::Vertex;
use crate::{math::Transformable, mesh::MeshType};

impl<U: Vertex> Transformable for U
where
    <U::T as MeshType>::VP: Transformable<
        Trans = <U::T as MeshType>::Trans,
        Rot = <U::T as MeshType>::Rot,
        Vec = <U::T as MeshType>::Vec,
        S = <U::T as MeshType>::S,
    >,
{
    type Rot = <U::T as MeshType>::Rot;
    type S = <U::T as MeshType>::S;
    type Trans = <U::T as MeshType>::Trans;
    type Vec = <U::T as MeshType>::Vec;

    #[inline(always)]
    fn transform(&mut self, transform: &Self::Trans) -> &mut Self {
        self.payload_mut().transform(transform);
        self
    }

    #[inline(always)]
    fn translate(&mut self, transform: &Self::Vec) -> &mut Self {
        self.payload_mut().translate(transform);
        self
    }

    #[inline(always)]
    fn rotate(&mut self, transform: &Self::Rot) -> &mut Self {
        self.payload_mut().rotate(transform);
        self
    }

    #[inline(always)]
    fn scale(&mut self, transform: &Self::Vec) -> &mut Self {
        self.payload_mut().scale(transform);
        self
    }

    #[inline(always)]
    fn lerp(&mut self, other: &Self, t: Self::S) -> &mut Self {
        self.payload_mut().lerp(other.payload(), t);
        self
    }
}
