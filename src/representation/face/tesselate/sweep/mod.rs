use super::{Face, Mesh, Payload};
use crate::{math::Vector3D, representation::IndexType};
mod chain;
mod point;
mod queue;
mod status;
mod vertex_type;
use point::IndexedVertexPoint;
pub use vertex_type::VertexType;

// See https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf

#[derive(Debug, Clone, Copy, PartialEq)]
/// Meta information for debuggin the sweep algorithm
pub struct SweepMeta {
    pub vertex_type: VertexType,
}

impl Default for SweepMeta {
    fn default() -> Self {
        SweepMeta {
            vertex_type: VertexType::Undefined,
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
        indices: &mut Vec<V>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));

        let vec2s: Vec<_> = self
            .vertices_2d::<V, P>(mesh)
            .enumerate()
            .map(|(i, (p, _))| IndexedVertexPoint::new(p, i))
            .collect();

        let event_queue = queue::SweepEventQueue::<P::Vec2, V>::new(&vec2s);
        let mut event_queue = event_queue;
        while event_queue.work(indices) {}
    }
}
