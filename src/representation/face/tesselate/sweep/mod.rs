use super::{Face, Mesh, Payload, TesselationMeta};
use crate::{math::Vector3D, representation::IndexType};
mod chain;
mod point;
mod queue;
mod status;
mod vertex_type;
use point::IndexedVertexPoint;
pub use vertex_type::VertexType;

// See https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf

/// Meta information for debuggin the sweep algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct SweepMeta<P: Payload> {
    /// The type of the vertex in the reflex chain
    pub vertex_type: Vec<(P::Vec, VertexType)>,
}

impl<P: Payload> Default for SweepMeta<P> {
    fn default() -> Self {
        SweepMeta {
            vertex_type: Vec::new(),
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
        meta: &mut TesselationMeta<P>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));

        let vec2s: Vec<_> = self
            .vertices_2d::<V, P>(mesh)
            .enumerate()
            .map(|(i, (p, _))| IndexedVertexPoint::new(p, i))
            .collect();

        let mut event_queue = queue::SweepEventQueue::<P::Vec2, V>::new(&vec2s);

        // #[cfg(feature = "sweep_debug")]
        meta.sweep = event_queue.extract_meta(
            &self
                .vertices(mesh)
                .map(|v| v.vertex().clone() as P::Vec)
                .collect(),
        );

        while event_queue.work(indices) {}
    }
}
