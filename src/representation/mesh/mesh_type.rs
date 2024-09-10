use crate::{
    math::{IndexType, Scalar, Transform, Vector, Vector2D, Vector3D},
    representation::{payload::VertexPayload, EdgePayload, FacePayload},
};

/// This trait defines all the associated types used in a mesh and puts them into relation.
pub trait MeshType: Copy {
    /// The type of the index used for edges.
    type E: IndexType;

    /// The type of the index used for vertices.
    type V: IndexType;

    /// The type of the index used for faces.
    type F: IndexType;

    /// The type of the edge payload.   
    type EP: EdgePayload;

    /// The type of the vertex payload.
    type VP: VertexPayload<
        Vec = Self::Vec,
        S = Self::S,
        Trans = Self::Trans,
        Vec2 = Self::Vec2,
        Vec3 = Self::Vec3,
    >;

    /// The type of the face payload.
    type FP: FacePayload;

    /// The type of the vector used for vertices.
    type Vec: Vector<Self::S, Vec2 = Self::Vec2, Vec3 = Self::Vec3, Trans = Self::Trans>;

    /// The 2d vector type derived from the default vector
    type Vec2: Vector2D<S = Self::S>;

    /// The 3d vector type derived from the default vector
    type Vec3: Vector3D<S = Self::S>;

    /// The type of the scalar used for vertices.
    type S: Scalar;

    /// The type of the transformation used for vertices.
    type Trans: Transform<S = Self::S, Vec = Self::Vec>;

    // TODO: Also provide shorthands for other derived types like HalfEdge, Vertex, Face
    // We could use https://crates.io/crates/supertrait
    // or https://github.com/rust-lang/rust/issues/29661 once thre RFC is stable
    // type VertexType = Vertex<Self::E, Self::V, Self::VP>;
}
