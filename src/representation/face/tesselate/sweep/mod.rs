use std::collections::HashMap;

use super::{Face, Mesh, Payload, TesselationMeta};
use crate::{math::Vector3D, representation::IndexType};
mod chain;
mod interval;
mod point;
mod status;
mod sweep;
mod vertex_type;
pub use sweep::{generate_zigzag, sweep_line_triangulation};
pub use vertex_type::VertexType;

// See https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf

/// Meta information for debuggin the sweep algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct SweepMeta {
    /// The type of the vertex in the reflex chain
    #[cfg(feature = "sweep_debug")]
    pub vertex_type: HashMap<usize, VertexType>,
}

impl Default for SweepMeta {
    fn default() -> Self {
        SweepMeta {
            #[cfg(feature = "sweep_debug")]
            vertex_type: HashMap::new(),
        }
    }
}

impl SweepMeta {
    /// Expand the indices to global indices
    #[cfg(feature = "sweep_debug")]
    pub fn expand<V: IndexType>(&mut self, ids: &Vec<V>) {
        let mut new_vertex_type = HashMap::new();
        for (i, v) in self.vertex_type.iter() {
            new_vertex_type.insert(ids[*i].index(), *v);
        }
        self.vertex_type = new_vertex_type;
    }

    /// Update the type of a vertex
    #[cfg(feature = "sweep_debug")]
    pub fn update_type(&mut self, i: usize, t: VertexType) {
        self.vertex_type.insert(i, t);
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
        let vec2s: Vec<P::Vec2> = self.vertices_2d::<V, P>(mesh).map(|(v, _)| v).collect();
        sweep_line_triangulation(indices, &vec2s, &mut meta.sweep);

        // Apply the original indices
        let ids: Vec<V> = self.vertices(mesh).map(|v| v.id()).collect();
        indices.iter_mut().for_each(|i| *i = ids[i.index()]);
        meta.sweep.expand(&ids);
    }
}
