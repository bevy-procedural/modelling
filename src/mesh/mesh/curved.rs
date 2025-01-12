use crate::{
    math::HasPosition,
    mesh::{
        CurvedEdge, CurvedEdgeType, DefaultEdgePayload, EuclideanMeshType, MeshBasics, MeshBuilder,
        MeshType,
    },
};

/// Some more advanced operations on the mesh.
pub trait MeshBasicsCurved<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Converts the mesh to a mesh without curved edges
    fn flatten_curved_edges<const D: usize>(&mut self, tol: T::S) -> &mut Self
    where
        T::Edge: CurvedEdge<D, T>,
        T::EP: DefaultEdgePayload,
        T: EuclideanMeshType<D>,
        T::VP: HasPosition<D, T::Vec>,
        T::Mesh: MeshBuilder<T>,
    {
        // TODO: assert that T::EP::default() is a linear edge

        // Convert curved edges
        for e in self.edge_ids().collect::<Vec<_>>().iter() {
            let edge = self.edge_ref(*e);
            if edge.curve_type(self) != CurvedEdgeType::Linear {
                let vs = edge.flatten_casteljau(tol, self);
                edge.clone()
                    .set_curve_type_in_mesh(self, CurvedEdgeType::Linear);
                if vs.len() == 0 {
                    continue;
                }
                self.subdivide_edge_iter(
                    *e,
                    vs.iter().map(|v| (T::EP::default(), T::VP::from_pos(*v))),
                );
            }
        }

        self
    }

    /// Returns whether the mesh has curved edges
    fn has_curved_edges<const D: usize>(&self) -> bool
    where
        T::Edge: CurvedEdge<D, T>,
        T: EuclideanMeshType<D>,
        T::VP: HasPosition<D, T::Vec>,
    {
        self.edges()
            .any(|e| e.curve_type(self) != CurvedEdgeType::Linear)
    }
}
