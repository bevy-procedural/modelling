//! This module contains the svg-specific implementations

use crate::mesh::{
    CurvedEdge, DefaultEdgePayload, DefaultFacePayload, EuclideanMeshType, MeshTypeHalfEdge,
};

mod svg;

/// Backend trait for SVG import/export.
pub trait BackendSVG<T: EuclideanMeshType<2, Mesh = Self>>
where
    T::Edge: CurvedEdge<2, T>,
    T::FP: DefaultFacePayload,
    T::EP: DefaultEdgePayload,
{
    /// Import an SVG string into the mesh.
    #[cfg(feature = "svg")]
    fn import_svg(&mut self, svg: &str) -> &mut Self
    where
        T: MeshTypeHalfEdge,
    {
        svg::import_svg::<T>(self, svg);
        self
    }

    /// Create a new mesh from an SVG string.
    #[cfg(feature = "svg")]
    fn from_svg(svg: &str) -> Self
    where
        T: MeshTypeHalfEdge,
    {
        let mut mesh = Self::default();
        mesh.import_svg(svg);
        mesh
    }
}

impl<T: EuclideanMeshType<2>> BackendSVG<T> for T::Mesh
where
    T::Edge: CurvedEdge<2, T>,
    T::FP: DefaultFacePayload,
    T::EP: DefaultEdgePayload,
{
}
