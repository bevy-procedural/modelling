use super::{Face, Mesh, Payload, TesselationMeta, Triangulation};
use crate::{
    math::Vector3D,
    representation::{tesselate::IndexedVertex2D, IndexType},
};
mod chain;
mod interval;
mod point;
mod status;
mod sweep;
mod vertex_type;
pub use sweep::sweep_line_triangulation;
pub use vertex_type::VertexType;

// See https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf

/// Meta information for debuggin the sweep algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct SweepMeta<V: IndexType> {
    #[cfg(feature = "sweep_debug")]
    /// The type of the vertex in the reflex chain
    pub vertex_type: Vec<(V, VertexType)>,
}

impl<V: IndexType> Default for SweepMeta<V> {
    fn default() -> Self {
        SweepMeta {
            #[cfg(feature = "sweep_debug")]
            vertex_type: Vec::new(),
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

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Uses the sweep line triangulation
    pub fn sweep_line<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Triangulation<V>,
        meta: &mut TesselationMeta<V>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));

        // TODO: Improve performance by directly using the nd-vertices instead of converting to 2d
        let vec2s: Vec<_> = self
            .vertices_2d::<V, P>(mesh)
            .map(|(p, i)| IndexedVertex2D::<V, P::Vec2>::new(p, i))
            .collect();

        sweep_line_triangulation::<V, P::Vec2>(indices, &vec2s, &mut meta.sweep);
    }
}
