//! This example demonstrates how to implement a custom mesh type with vertex colors.

use itertools::Itertools;
use procedural_modelling::{extensions::nalgebra::*, prelude::*};

fn main() {
    let mut mesh = Mesh3dColored::cube(1.0);

    // make the vertices of one face red
    let vs = mesh.faces().next().unwrap().vertex_ids(&mesh).collect_vec();
    for v in vs {
        mesh.vertex_ref_mut(v).payload_mut().color = VertexColor { r: 255, g: 0, b: 0 };
    }

    // since we implemented the Transformable trait for our custom payload, we can transform the mesh
    mesh.translate(&Vec3::new(1.0, 0.0, 0.0));

    // triangulate the mesh and retrieve the indices
    let (indices, vertices) = mesh.triangulate(TriangulationAlgorithm::Auto);

    // Check how many triangles have all red vertices
    let red_triangles = indices
        .iter()
        .tuples()
        .filter(|(i1, i2, i3)| {
            let v1 = vertices[**i1 as usize];
            let v2 = vertices[**i2 as usize];
            let v3 = vertices[**i3 as usize];
            v1.color.r == 255 && v2.color.r == 255 && v3.color.r == 255
        })
        .count();
    assert_eq!(red_triangles, 2);
}

/// Let's say vertex colors are represented as RGB colors
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct VertexColor {
    // red
    r: u8,
    // green
    g: u8,
    // blue
    b: u8,
}

impl VertexColor {
    /// Linearly interpolate between two colors
    fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            r: (self.r as f64 * (1.0 - t) + other.r as f64 * t) as u8,
            g: (self.g as f64 * (1.0 - t) + other.g as f64 * t) as u8,
            b: (self.b as f64 * (1.0 - t) + other.b as f64 * t) as u8,
        }
    }
}

/// A nd mesh type with 16 bit indices and colored vertices with 64-bit vertex positions
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct MeshTypeColored<const D: usize>;

// Define the datatypes used in the mesh
impl<const D: usize> MeshType for MeshTypeColored<D> {
    type E = u16;
    type V = u16;
    type F = u16;
    type EP = EmptyEdgePayload<Self>;
    type VP = VertexPayloadColored<f64, D>;
    type FP = EmptyFacePayload<Self>;
    type MP = EmptyMeshPayload<Self>;
    type Mesh = MeshNdColored<D>;
    type Face = HalfEdgeFaceImpl<Self>;
    type Edge = HalfEdgeImpl<Self>;
    type Vertex = HalfEdgeVertexImpl<Self>;
}

// enable functionality for meshes in euclidean space
impl<const D: usize> EuclideanMeshType<D> for MeshTypeColored<D> {
    type S = f64;
    type Vec = VecN<f64, D>;
    type Vec2 = VecN<f64, 2>;
    type Trans = NdAffine<f64, D>;
    type Rot = NdRotate<f64, D>;
    type Poly = Polygon2d<f64>;
}

// enable half-edge-specific functionality  
impl<const D: usize> HalfEdgeImplMeshType for MeshTypeColored<D> {}
impl<const D: usize> HalfEdgeImplMeshTypePlus for MeshTypeColored<D> {}
impl<const D: usize> MeshTypeHalfEdge for MeshTypeColored<D> {}

// enable some extra functionality for 3d meshes
impl MeshType3D for MeshTypeColored<3> {}

/// A nd mesh with colored vertices
pub type MeshNdColored<const D: usize> = HalfEdgeMeshImpl<MeshTypeColored<D>>;

/// d-dimensional Vertex Payload with position, normal, and uv coordinates.
#[derive(Clone, PartialEq, Copy, Debug)]
pub struct VertexPayloadColored<S: Scalar, const D: usize> {
    /// The position of the vertex.
    position: VecN<S, D>,

    /// The normal of the vertex.
    color: VertexColor,
}

impl<S: Scalar, const D: usize> VertexPayload for VertexPayloadColored<S, D> {
    fn allocate() -> Self {
        Self {
            position: VecN::zeros(),
            color: VertexColor::default(),
        }
    }
}

// When we want to transform our mesh, we need to implement the Transformable trait for our payloads
impl<S: ScalarPlus, const D: usize> Transformable<D> for VertexPayloadColored<S, D> {
    type S = S;
    type Vec = VecN<S, D>;
    type Trans = NdAffine<S, D>;
    type Rot = NdRotate<S, D>;

    #[inline]
    fn translate(&mut self, v: &Self::Vec) -> &mut Self {
        self.position += *v;
        self
    }

    #[inline]
    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        self.position = t.apply(self.position);
        self
    }

    #[inline]
    fn lerp(&mut self, _other: &Self, t: Self::S) -> &mut Self {
        self.position = self.position.lerp(&_other.position, t);
        self.color = self.color.lerp(&_other.color, t.as_f64());
        self
    }
}

// Our payloads have positions! We need to define how to access and modify them
impl<S: Scalar, const D: usize> HasPosition<D, VecN<S, D>> for VertexPayloadColored<S, D> {
    type S = S;

    #[inline]
    fn from_pos(v: VecN<S, D>) -> Self {
        Self {
            position: v,
            color: VertexColor::default(),
        }
    }

    #[inline]
    fn pos(&self) -> &VecN<S, D> {
        &self.position
    }

    #[inline]
    fn set_pos(&mut self, v: VecN<S, D>) {
        self.position = v;
    }
}

/// shorthand for the 3d version of our mesh
pub type Mesh3dColored = MeshNdColored<3>;
/// shorthand for the 3d version of our mesh vertex type
pub type Mesh3dColoredVertex = HalfEdgeVertexImpl<MeshTypeColored<3>>;
/// shorthand for the 3d version of our mesh edge type
pub type Mesh3dColoredEdge = HalfEdgeImpl<MeshTypeColored<3>>;
/// shorthand for the 3d version of our mesh face type
pub type Mesh3dColoredFace = HalfEdgeFaceImpl<MeshTypeColored<3>>;
/// shorthand for the 3d version of our mesh type
pub type MeshTypeColored3d = MeshTypeColored<3>;
