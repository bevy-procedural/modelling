use super::{Face, Mesh, Payload, TesselationMeta};
use crate::{math::Vector3D, representation::IndexType};
mod chain;
mod point;
mod queue;
mod status;
mod vertex_type;
use point::LocallyIndexedVertex;
pub use vertex_type::VertexType;

// See https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf

/// Meta information for debuggin the sweep algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct SweepMeta {
    #[cfg(feature = "sweep_debug")]
    /// The type of the vertex in the reflex chain
    pub vertex_type: Vec<(usize, VertexType)>,
}

impl Default for SweepMeta {
    fn default() -> Self {
        SweepMeta {
            #[cfg(feature = "sweep_debug")]
            vertex_type: Vec::new(),
        }
    }
}

impl SweepMeta {
    /// Expand the indices to global indices
    #[cfg(feature = "sweep_debug")]
    pub fn expand(&mut self, v0: usize, n: usize) {
        for (i, _) in self.vertex_type.iter_mut() {
            *i = v0 + (*i + (n - 1)) % n;
        }
    }

    /// Update the type of a vertex
    #[cfg(feature = "sweep_debug")]
    pub fn update_type(&mut self, i: usize, t: VertexType) {
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
        indices: &mut Vec<V>,
        meta: &mut TesselationMeta,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));

        // TODO: Improve performance by directly using the nd-vertices instead of converting to 2d
        let vec2s: Vec<_> = self
            .vertices_2d::<V, P>(mesh)
            .enumerate()
            .map(|(i, (p, _))| LocallyIndexedVertex::new(p, i))
            .collect();

        let mut event_queue = queue::SweepEventQueue::<P::Vec2, V>::new(&vec2s);

        #[cfg(feature = "sweep_debug")]
        {
            meta.sweep = event_queue.extract_meta();
        }

        while event_queue.work(indices, &mut meta.sweep) {}
    }
}
