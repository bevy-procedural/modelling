use crate::{
    halfedge::{HalfEdgeFaceImpl, HalfEdgeImplMeshTypePlus, HalfEdgeMeshImpl, HalfEdgeVertexImpl},
    math::{IndexType, Transformable},
    mesh::{
        cursor::*, DefaultEdgePayload, DefaultFacePayload, EdgeBasics, EuclideanMeshType,
        FaceBasics, HalfEdge, HalfEdgeMesh, MeshBasics, MeshImport, MeshType,
    },
};

impl<T: HalfEdgeImplMeshTypePlus> MeshImport<T> for HalfEdgeMeshImpl<T> {
    #[inline]
    fn empty() -> Self {
        Self::new()
    }

    #[inline]
    fn insert_mesh<'a>(&'a mut self, _other: &T::Mesh) -> EdgeCursorMut<'a, T>
    where
        T::Edge: HalfEdge<T>,
        T::Mesh: HalfEdgeMesh<T>,
    {
        self.import_mesh::<_, _, _, _, T>(
            _other,
            |v: &T::VP| v.clone(),
            |e: &T::EP| e.clone(),
            |f: &T::FP| f.clone(),
            |m: &T::MP| m.clone(),
        )
    }

    #[inline]
    fn insert_transformed_mesh<'a, const D: usize>(
        &'a mut self,
        other: &T::Mesh,
        transform: &T::Trans,
    ) -> EdgeCursorMut<'a, T>
    where
        T: EuclideanMeshType<D>,
        T::VP: Transformable<D, Trans = T::Trans>,
        T::EP: Transformable<D, Trans = T::Trans>,
        T::FP: Transformable<D, Trans = T::Trans>,
        T::MP: Transformable<D, Trans = T::Trans>,
    {
        self.import_mesh::<_, _, _, _, T>(
            other,
            |v: &T::VP| v.transformed(&transform),
            |e: &T::EP| e.transformed(&transform),
            |f: &T::FP| f.transformed(&transform),
            |m: &T::MP| m.transformed(&transform),
        )
    }

    fn import_mesh<'a, FE, FV, FF, FM, T2: MeshType>(
        &'a mut self,
        mesh: &T2::Mesh,
        fv: FV,
        fe: FE,
        ff: FF,
        fm: FM,
    ) -> EdgeCursorMut<'a, T>
    where
        FE: Fn(&T2::EP) -> T::EP,
        FV: Fn(&T2::VP) -> T::VP,
        FF: Fn(&T2::FP) -> T::FP,
        FM: Fn(&T2::MP) -> T::MP,
        T2::Edge: HalfEdge<T2>,
        T2::Mesh: HalfEdgeMesh<T2>,
    {
        let mut vertex_map = std::collections::HashMap::new();
        for vertex in MeshBasics::vertices(mesh) {
            let v = self.vertices.allocate();
            vertex_map.insert(vertex.id(), v);
        }
        let mut face_map = std::collections::HashMap::new();
        face_map.insert(IndexType::max(), IndexType::max());
        for face in mesh.faces() {
            let f = self.faces.allocate();
            face_map.insert(face.id(), f);
        }

        let mut edge_map = std::collections::HashMap::new();
        for edge in HalfEdgeMesh::halfedges(mesh) {
            let e = self.halfedges.allocate();
            edge_map.insert(edge.id(), e);
        }

        for vertex in MeshBasics::vertices(mesh) {
            self.vertices.set(
                vertex_map[&vertex.id()],
                HalfEdgeVertexImpl::new(edge_map[&vertex.edge_id()], fv(vertex.payload())),
            );
        }

        for face in mesh.faces() {
            let f = HalfEdgeFaceImpl::new(
                edge_map[&face.edge_id()],
                ff(face.payload()),
                face_map[&face.inner().next_island_helper().unwrap_or(face.id())],
            );
            //TODO: f.set_next_island(face_map[&face.inner().next_island()]);
            self.faces.set(face_map[&face.id()], f);
        }

        for edge in HalfEdgeMesh::halfedges(mesh) {
            self.insert_halfedge_forced(
                edge_map[&edge.id()],
                vertex_map[&edge.origin_id()],
                face_map[&edge.face_id()],
                edge_map[&edge.prev_id()],
                edge_map[&edge.twin_id()],
                edge_map[&edge.next_id()],
                edge.unwrap().inner().payload_self().map(|x| fe(x)),
            );
        }

        self.set_payload(fm(MeshBasics::payload(mesh)));

        let Some(edge) = HalfEdgeMesh::halfedges(mesh).next() else {
            return self.edge_mut(IndexType::max());
        };

        self.edge_mut(edge_map[&edge.id()])
    }

    fn import_indexed_triangles<'a>(
        &'a mut self,
        indices: Vec<T::V>,
        mut vertices: Vec<T::VP>,
    ) -> EdgeCursorMut<'a, T>
    where
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
    {
        use std::collections::HashMap;

        assert_eq!(
            indices.len() % 3,
            0,
            "indices must be a multiple of three (triangles)"
        );

        /* ---------- vertices ---------- */
        let mut v_map = Vec::with_capacity(vertices.len()); // external-idx → internal-idx
        for payload in vertices.drain(..) {
            let v = self.vertices.allocate();
            self.vertices
                .set(v, HalfEdgeVertexImpl::new(IndexType::max(), payload));
            v_map.push(v);
        }

        /* ---------- edges & faces ---------- */
        let mut first_edge: Option<T::E> = None;
        let mut twin_map: HashMap<(T::V, T::V), T::E> = HashMap::with_capacity(indices.len());

        for tri in indices.chunks_exact(3) {
            let [i0, i1, i2] = [tri[0], tri[1], tri[2]];
            let vs = [v_map[i0.index()], v_map[i1.index()], v_map[i2.index()]];

            // face & three half-edges
            let f = self.faces.allocate();
            let hes = [
                self.halfedges.allocate(),
                self.halfedges.allocate(),
                self.halfedges.allocate(),
            ];
            if first_edge.is_none() {
                first_edge = Some(hes[0]);
            }

            // store face payload + representative edge
            self.faces
                .set(f, HalfEdgeFaceImpl::new(hes[0], T::FP::default(), f));

            for j in 0..3 {
                let he = hes[j];
                let origin = vs[j];
                let dest = vs[(j + 1) % 3];
                let prev = hes[(j + 2) % 3];
                let next = hes[(j + 1) % 3];

                let twin = twin_map
                    .get(&(dest, origin))
                    .cloned()
                    .unwrap_or(IndexType::max());

                // create half-edge (twin gets patched later)
                self.insert_halfedge_forced(
                    he,
                    origin,
                    f,
                    prev,
                    twin,
                    next,
                    if twin == IndexType::max() {
                        Some(T::EP::default())
                    } else {
                        None
                    },
                );

                // remember outgoing edge for the vertex if unset
                let mut v_rec = self.vertex_mut(origin).unwrap();
                if v_rec.edge_id() == IndexType::max() {
                    v_rec.set_edge(he);
                }

                // stitch twins (unordered key)
                if let Some(&twin) = twin_map.get(&(dest, origin)) {
                    // SAFETY: both half-edges exist
                    self.halfedges.get_mut(he).unwrap().set_twin(twin);
                    self.halfedges.get_mut(twin).unwrap().set_twin(he);
                    twin_map.remove(&(dest, origin));
                } else {
                    twin_map.insert((origin, dest), he);
                }
            }
        }

        // close boundaries / holes
        // PERF: This is not unique for non-manifold vertices!
        let mut unconnected_boundary_by_target: HashMap<T::V, T::E> =
            HashMap::with_capacity(twin_map.len());
        for ((origin, dest), he) in twin_map {
            let twin = self.halfedges.allocate();
            self.insert_halfedge_forced(
                twin,
                dest,
                IndexType::max(),
                IndexType::max(),
                he,
                IndexType::max(),
                None,
            );
            self.halfedges.get_mut(he).unwrap().set_twin(twin);
            unconnected_boundary_by_target.insert(origin, twin);
        }

        for (_target, e) in unconnected_boundary_by_target.clone() {
            let origin = self.edge(e).unwrap().origin_id();
            let prev = unconnected_boundary_by_target[&origin];
            self.edge_mut(e).unwrap().set_prev(prev);
            self.edge_mut(prev).unwrap().set_next(e);
        }

        self.set_payload(T::MP::default());

        self.edge_mut(first_edge.unwrap_or(IndexType::max()))
    }
}
