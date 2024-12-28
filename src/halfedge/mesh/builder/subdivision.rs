use crate::{
    halfedge::{HalfEdgeImpl, HalfEdgeMeshImpl, HalfEdgeMeshType, HalfEdgeVertexImpl},
    mesh::{DefaultEdgePayload, EdgeBasics, HalfEdge, MeshBasics},
};

impl<T: HalfEdgeMeshType> HalfEdgeMeshImpl<T> where T::EP: DefaultEdgePayload {}
