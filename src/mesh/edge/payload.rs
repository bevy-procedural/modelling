use std::hash::Hash;

use crate::{
    math::Transformable,
    mesh::{EuclideanMeshType, MeshType},
};

use super::CurvedEdgeType;

/// A trait that defines how the payload of an edge should behave.
/// 
/// Edge payloads are exactly one per edge, .e.g., on half-edge graphs, 
/// there should only be one payload per pair of half-edges.
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
pub struct EmptyEdgePayload<T: MeshType> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: MeshType> EdgePayload for EmptyEdgePayload<T> {
    fn allocate() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl<T: MeshType> DefaultEdgePayload for EmptyEdgePayload<T> {}

impl<const D: usize, T: EuclideanMeshType<D>> Transformable<D> for EmptyEdgePayload<T> {
    type Rot = T::Rot;
    type S = T::S;
    type Trans = T::Trans;
    type Vec = T::Vec;

    fn transform(&mut self, _: &Self::Trans) -> &mut Self {
        self
    }

    fn lerp(&mut self, _: &Self, _: Self::S) -> &mut Self {
        self
    }
}

/// A curved edge payload with nothing else
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CurvedEdgePayload<const D: usize, T: EuclideanMeshType<D>> {
    curve: CurvedEdgeType<D, T>,
}

impl<const D: usize, T: EuclideanMeshType<D>> CurvedEdgePayload<D, T> {
    /// Returns the curve type of the edge
    pub fn curve_type(&self) -> CurvedEdgeType<D, T> {
        self.curve
    }

    /// Sets the curve type of the edge
    pub fn set_curve_type(&mut self, curve_type: CurvedEdgeType<D, T>) {
        self.curve = curve_type;
    }
}

// TODO: somehow make sure the twin is never curved or has the same curve type
// TODO: make sure edge payloads are printed even for winged edges
impl<const D: usize, T: EuclideanMeshType<D>> EdgePayload for CurvedEdgePayload<D, T> {
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

impl<const D: usize, T: EuclideanMeshType<D>> DefaultEdgePayload for CurvedEdgePayload<D, T> {}

impl<const D: usize, T: EuclideanMeshType<D>> Transformable<D> for CurvedEdgePayload<D, T> {
    type Rot = T::Rot;
    type S = T::S;
    type Trans = T::Trans;
    type Vec = T::Vec;

    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        match &mut self.curve {
            CurvedEdgeType::Linear => {}
            CurvedEdgeType::QuadraticBezier(control_point) => {
                control_point.transform(t);
            }
            CurvedEdgeType::CubicBezier(control_point1, control_point2) => {
                control_point1.transform(t);
                control_point2.transform(t);
            }
        }
        self
    }

    fn lerp(&mut self, _other: &Self, _t: Self::S) -> &mut Self {
        match &mut self.curve {
            CurvedEdgeType::Linear => {}
            CurvedEdgeType::QuadraticBezier(_cp) => {
                todo!();
            }
            CurvedEdgeType::CubicBezier(_cp1, _cp2) => {
                todo!();
            }
        }
        self
    }
}
