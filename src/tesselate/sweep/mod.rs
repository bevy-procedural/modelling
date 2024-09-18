mod chain;
mod interval;
mod point;
mod status;
mod sweep;
mod vertex_type;

pub use sweep::sweep_line_triangulation;
pub use vertex_type::VertexType;

use crate::{
    math::{HasPosition, IndexType, Vector3D},
    mesh::{Face, Face3d, MeshType},
    tesselate::IndexedVertex2D,
};

use super::{TesselationMeta, Triangulation};

/// Meta information for debuggin the sweep algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct SweepMeta<V: IndexType> {
    #[cfg(feature = "sweep_debug")]
    /// The type of the vertex in the reflex chain
    pub vertex_type: Vec<(V, VertexType)>,

    phantom: std::marker::PhantomData<V>,
}

impl<V: IndexType> Default for SweepMeta<V> {
    fn default() -> Self {
        SweepMeta {
            #[cfg(feature = "sweep_debug")]
            vertex_type: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<V: IndexType> SweepMeta<V> {
    /// Update the type of a vertex
    #[cfg(feature = "sweep_debug")]
    pub fn update_type(&mut self, i: V, t: VertexType) {
        // TODO: Not efficient
        for (j, ty) in self.vertex_type.iter_mut() {
            if *j == i {
                *ty = t;
            }
        }
    }
}

/// Uses the sweep line triangulation
pub fn sweep_line<T: MeshType>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
    meta: &mut TesselationMeta<T::V>,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    debug_assert!(face.may_be_curved() || face.is_planar2(mesh));

    // TODO: Improve performance by directly using the nd-vertices instead of converting to 2d
    let vec2s: Vec<_> = face
        .vertices_2d(mesh)
        .map(|(p, i)| IndexedVertex2D::<T::V, T::Vec2>::new(p, i))
        .collect();

    sweep_line_triangulation::<T::V, T::Vec2>(indices, &vec2s, &mut meta.sweep);
}
