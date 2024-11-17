use std::hash::Hash;

use crate::mesh::MeshType;

use super::CurvedEdgeType;

/// A trait that defines how the payload of an edge should behave.
pub trait EdgePayload: Clone + std::fmt::Debug + PartialEq {
    /// Returns a new default instance without any meaningful data.
    fn allocate() -> Self;

    /// Returns true if the payload is empty.
    fn is_empty(&self) -> bool;
}

/// The default edge payload can be safely constructed with a default constructor.
/// For example, when extruding, it is ok for all new edges to have the same default payload.
pub trait DefaultEdgePayload: EdgePayload + Default {}

/// An empty edge payload if you don't need any additional information.
#[derive(Debug, Clone, Copy, PartialEq, Default, Hash)]
pub struct EmptyEdgePayload;

impl EdgePayload for EmptyEdgePayload {
    fn allocate() -> Self {
        Self
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl DefaultEdgePayload for EmptyEdgePayload {}

/// A curved edge payload with nothing else
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CurvedEdgePayload<T: MeshType> {
    curve: CurvedEdgeType<T>,
}

impl<T: MeshType> CurvedEdgePayload<T> {
    /// Returns the curve type of the edge
    pub fn curve_type(&self) -> CurvedEdgeType<T> {
        self.curve
    }

    /// Sets the curve type of the edge
    pub fn set_curve_type(&mut self, curve_type: CurvedEdgeType<T>) {
        self.curve = curve_type;
    }
}

impl<T: MeshType> EdgePayload for CurvedEdgePayload<T> {
    fn allocate() -> Self {
        Default::default()
    }
    fn is_empty(&self) -> bool {
        match self.curve {
            CurvedEdgeType::Linear => true,
            _ => false,
        }
    }
}

impl<T: MeshType> DefaultEdgePayload for CurvedEdgePayload<T> {}
