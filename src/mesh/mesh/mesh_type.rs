use super::{payload::MeshPayload, Mesh};
use crate::{
    math::{IndexType, Scalar, Transform, Vector, Vector2D, Vector3D, Vector4D},
    mesh::{payload::VertexPayload, Edge, EdgePayload, Face, FacePayload, Vertex},
};

/// This trait defines all the associated types used in a mesh and puts them into relation.
pub trait MeshType: Copy {
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
    type Trans: Transform<S = Self::S, Vec = Self::Vec>;

    /// The type of the rotation data used for vertices.
    type Rot;

    type Mesh: Mesh<Self>;

    type Edge: Edge<Self::E, Self::V, Self::F, Self::EP>;

    type Vertex: Vertex<Self::E, Self::V, Self::VP>;

    type Face: Face<Self::E, Self::F, Self::FP>;
}
