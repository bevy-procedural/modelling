use super::HasPosition;
use crate::{
    math::{Scalar, Vector, Vector3D},
    mesh::{cursor::*, EuclideanMeshType, MeshBasics, MeshType},
};

/// A trait that defines how vertex interpolators should behave.
/// They interpolate vertices resp. their positions.
pub trait VertexInterpolator<const N: usize, T: MeshType> {
    /// Interpolates the vertex positions based on the given vertices.
    fn call(&self, mesh: &T::Mesh, vertices: [(usize, T::V); N]) -> T::VP;
}

/// Vertex interpolator that performs linear interpolation.
pub struct LinearVertexInterpolator<const D: usize> {}

impl<const D: usize, T: EuclideanMeshType<D>> VertexInterpolator<3, T>
    for LinearVertexInterpolator<D>
where
    T::VP: HasPosition<D, T::Vec, S = T::S>,
{
    /// Subdivides by linear interpolation of the positions of the vertices.
    fn call(&self, mesh: &T::Mesh, [(i, vi), (j, vj), (k, vk)]: [(usize, T::V); 3]) -> T::VP {
        // TODO: avoid unwrap
        let pi = mesh.vertex(vi).unwrap().pos();
        let pj = mesh.vertex(vj).unwrap().pos();
        let pk = mesh.vertex(vk).unwrap().pos();
        T::VP::from_pos(
            (pi * T::S::from_usize(i) + pj * T::S::from_usize(j) + pk * T::S::from_usize(k))
                / T::S::from_usize(i + j + k),
        )
    }
}

/// Vertex interpolator that performs spherical linear interpolation (slerp).
pub struct SlerpVertexInterpolator<const D: usize, T: EuclideanMeshType<D>> {
    center: T::Vec,
    radius: T::S,
}

impl<const D: usize, T: EuclideanMeshType<D>> SlerpVertexInterpolator<D, T> {
    /// Creates a new SlerpVertexInterpolator with the given center and radius.
    /// Theoretically the radius could be inferred from the vertices, but to
    /// enforce a consistent radius and improve numerical stability, you should
    /// pass it explicitly.
    pub fn new(center: T::Vec, radius: T::S) -> Self {
        Self { center, radius }
    }
}

impl<const D: usize, T: EuclideanMeshType<D>> VertexInterpolator<3, T>
    for SlerpVertexInterpolator<D, T>
where
    T::Vec: Vector3D<S = T::S>,
{
    // TODO: avoid unwrap

    /// Subdivides by linear interpolation of the positions of the vertices.
    fn call(&self, mesh: &T::Mesh, [(i, vi), (j, vj), (k, vk)]: [(usize, T::V); 3]) -> T::VP {
        let pi = (mesh.vertex(vi).unwrap().pos() - self.center).normalize();
        let pj = (mesh.vertex(vj).unwrap().pos() - self.center).normalize();
        let pk = (mesh.vertex(vk).unwrap().pos() - self.center).normalize();

        // slerp
        let pos = if i == 0 {
            pj.slerp(&pk, T::S::HALF)
        } else if j == 0 {
            pk.slerp(&pi, T::S::HALF)
        } else if k == 0 {
            pi.slerp(&pj, T::S::HALF)
        } else {
            todo!("slerp 3")
        };

        T::VP::from_pos(self.center + pos.normalize() * self.radius)
    }
}
