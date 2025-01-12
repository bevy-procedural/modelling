use crate::{
    halfedge::{HalfEdgeFaceImpl, HalfEdgeMeshImpl, HalfEdgeVertexImpl},
    math::IndexType,
    mesh::{
        CursorData, EdgeBasics, FaceCursorBasics, HalfEdge, HalfEdgeMesh, MeshBasics, MeshType,
        VertexCursorBasics,
    },
    prelude::HalfEdgeImplMeshTypePlus,
};

impl<T: HalfEdgeImplMeshTypePlus> HalfEdgeMeshImpl<T> {
    pub(crate) fn import_mesh<FE, FV, FF, FM, T2: MeshType>(
        mesh: &T2::Mesh,
        fv: FV,
        fe: FE,
        ff: FF,
        fm: FM,
    ) -> Self
    where
        FE: Fn(&T2::EP) -> T::EP,
        FV: Fn(&T2::VP) -> T::VP,
        FF: Fn(&T2::FP) -> T::FP,
        FM: Fn(&T2::MP) -> T::MP,
        T2::Edge: HalfEdge<T2>,
        T2::Mesh: HalfEdgeMesh<T2>,
    {
        let mut res = Self::default();
        let mut vertex_map = std::collections::HashMap::new();
        for vertex in MeshBasics::vertices(mesh) {
            let v = res.vertices.allocate();
            vertex_map.insert(vertex.id(), v);
        }
        let mut face_map = std::collections::HashMap::new();
        face_map.insert(IndexType::max(), IndexType::max());
        for face in mesh.faces() {
            let f = res.faces.allocate();
            face_map.insert(face.id(), f);
        }
        let mut edge_map = std::collections::HashMap::new();
        for edge in HalfEdgeMesh::halfedges(mesh) {
            let e = res.halfedges.allocate();
            edge_map.insert(edge.id(), e);
        }

        for vertex in MeshBasics::vertices(mesh) {
            res.vertices.set(
                vertex_map[&vertex.id()],
                HalfEdgeVertexImpl::new(edge_map[&vertex.edge_id()], fv(vertex.payload())),
            );
        }

        for face in mesh.faces() {
            res.faces.set(
                face_map[&face.id()],
                HalfEdgeFaceImpl::new(edge_map[&face.edge_id()], ff(face.payload())),
            );
        }

        for edge in HalfEdgeMesh::halfedges(mesh) {
            res.insert_halfedge_forced(
                edge_map[&edge.id()],
                vertex_map[&edge.origin_id(mesh)],
                face_map[&edge.face_id()],
                edge_map[&edge.prev_id()],
                edge_map[&edge.twin_id()],
                edge_map[&edge.next_id()],
                edge.payload_self().map(|x| fe(x)),
            );
        }

        res.set_payload(fm(MeshBasics::payload(mesh)));

        res
    }
}
