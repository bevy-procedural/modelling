use super::{HasPosition, VertexBasics};
use crate::{
    math::{Scalar, Vector, Vector3D},
    mesh::{MeshBasics, MeshType},
};

/// A trait that defines how vertex interpolators should behave.
/// As you might expect, vertex interpolators are used to interpolate vertices resp. their positions.
pub trait VertexInterpolator<const N: usize, T: MeshType> {
    /// Interpolates the vertex positions based on the given vertices.
    fn call(&self, mesh: &T::Mesh, vertices: [(usize, T::V); N]) -> T::VP;
}

/// Vertex interpolator that performs linear interpolation.
pub struct LinearVertexInterpolator {}
impl<T: MeshType> VertexInterpolator<3, T> for LinearVertexInterpolator
where
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Subdivides by linear interpolation of the positions of the vertices.
    fn call(&self, mesh: &T::Mesh, [(i, vi), (j, vj), (k, vk)]: [(usize, T::V); 3]) -> T::VP {
        let pi = mesh.vertex(vi).pos();
        let pj = mesh.vertex(vj).pos();
        let pk = mesh.vertex(vk).pos();
        T::VP::from_pos(
            (pi * T::S::from_usize(i) + pj * T::S::from_usize(j) + pk * T::S::from_usize(k))
                / T::S::from_usize(i + j + k),
        )
    }
}

/// Vertex interpolator that performs spherical linear interpolation (slerp).
pub struct SlerpVertexInterpolator<T: MeshType> {
    center: T::Vec,
    radius: T::S,
}

impl<T: MeshType> SlerpVertexInterpolator<T> {
    /// Creates a new SlerpVertexInterpolator with the given center and radius.
    /// Theoretically the radius could be inferred from the vertices, but to
    /// enforce a consistent radius and improve numerical stability, you should
    /// pass it explicitly.
    pub fn new(center: T::Vec, radius: T::S) -> Self {
        Self { center, radius }
    }
}

impl<T: MeshType> VertexInterpolator<3, T> for SlerpVertexInterpolator<T>
where
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Vec: Vector3D<S = T::S>,
{
    /// Subdivides by linear interpolation of the positions of the vertices.
    fn call(&self, mesh: &T::Mesh, [(i, vi), (j, vj), (k, vk)]: [(usize, T::V); 3]) -> T::VP {
        let pi = (mesh.vertex(vi).pos() - self.center).normalize();
        let pj = (mesh.vertex(vj).pos() - self.center).normalize();
        let pk = (mesh.vertex(vk).pos() - self.center).normalize();

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
