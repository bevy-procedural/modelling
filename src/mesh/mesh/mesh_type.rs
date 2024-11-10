use crate::{
    math::{
        HasPosition, IndexType, Polygon, Rotator, Scalar, TransformTrait, Transformable, Vector,
        Vector2D, Vector3D, Vector4D,
    },
    mesh::{
        Edge, EdgePayload, Face, Face3d, FacePayload, HalfEdge, HalfEdgeMesh, HalfEdgeVertex,
        MeshBuilder, MeshHalfEdgeBuilder, MeshPayload, MeshTrait, Vertex, VertexPayload,
    },
};

use super::{MeshPathBuilder, MeshPosition};

// TODO: The `Copy` here is weird. Should probably remove it.

/// This trait defines all the associated types used in a mesh and puts them into relation.
pub trait MeshType: Copy + Eq {
    /// The type of the index used for edge.
    type E: IndexType;

    /// The type of the index used for vertices.
    type V: IndexType;

    /// The type of the index used for faces.
    type F: IndexType;

    /// The type of the edge payload.   
    type EP: EdgePayload;

    /// The type of the vertex payload.
    type VP: VertexPayload;

    /// The type of the face payload.
    type FP: FacePayload;

    /// The type of the mesh payload.
    type MP: MeshPayload<Self>;

    /// The type of the vector used for vertices.
    type Vec: Vector<
        Self::S,
        Vec2 = Self::Vec2,
        Vec3 = Self::Vec3,
        Vec4 = Self::Vec4,
        Trans = Self::Trans,
    >;

    /// The 2d vector type derived from the default vector
    type Vec2: Vector2D<S = Self::S>;

    /// The 3d vector type derived from the default vector
    type Vec3: Vector3D<S = Self::S>;

    /// The 4d vector type derived from the default vector
    type Vec4: Vector4D<S = Self::S>;

    /// The type of the scalar used for vertices.
    type S: Scalar;

    /// The type of the transformation used for vertices.
    type Trans: TransformTrait<S = Self::S, Vec = Self::Vec>;

    /// The type of the rotation data used for vertices.
    type Rot: Rotator<Self::Vec>;

    /// The implementation of 2d polygons.
    type Poly: Polygon<Self::Vec2>;

    /// The type of the mesh.
    type Mesh: MeshTrait<T = Self> + MeshBuilder<Self> + MeshPathBuilder<Self>;

    /// The type of the (half-)edge or arc.
    type Edge: Edge<T = Self>;

    /// The type of the vertex.
    type Vertex: Vertex<T = Self>;

    /// The type of the face.
    type Face: Face<T = Self>;
}

/// A `MeshType` specialized for half-edge meshes
pub trait HalfEdgeMeshType:
    MeshType<
    Mesh: MeshBuilder<Self> + HalfEdgeMesh<Self> + MeshHalfEdgeBuilder<Self>,
    Edge: HalfEdge<Self>,
    Vertex: HalfEdgeVertex<Self>,
>
{
}

/// A `MeshType` specialized for meshes with 3d position data
pub trait MeshType3D:
    MeshType<
    Mesh: MeshPosition<Self>,
    VP: Transformable<Vec = Self::Vec, Rot = Self::Rot, Trans = Self::Trans, S = Self::S>
            + HasPosition<Self::Vec, S = Self::S>,
    Vec: Vector3D<S = Self::S>,
    Face: Face3d<Self>,
>
{
}
