use crate::{
    math::Transformable,
    mesh::{
        cursor::*, DefaultEdgePayload, DefaultFacePayload, EuclideanMeshType, HalfEdge,
        HalfEdgeMesh, MeshBasics, MeshType,
    },
};

/// Some basic operations to copy meshes into other meshes or convert their types.
pub trait MeshImport<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Create an empty mesh with the given payload.
    #[must_use]
    fn empty() -> Self;

    /// Copy the given mesh into the current mesh.
    ///
    /// Returns a cursor pointing to an arbitrary edge in the inserted mesh.
    #[must_use]
    fn insert_mesh<'a>(&'a mut self, other: &T::Mesh) -> EdgeCursorMut<'a, T>
    where
        // TODO: Get rid of these halfedge specific constraints
        T::Edge: HalfEdge<T>,
        T::Mesh: HalfEdgeMesh<T>;

    /// Copy the given mesh into the current mesh, but transform the payloads.
    ///
    /// Returns a cursor pointing to an arbitrary edge in the inserted mesh.
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
        T::MP: Transformable<D, Trans = T::Trans>;

    /// Copy the given mesh into the current mesh and transform the payloads using the given functions.
    /// Can be usefull for converting between different mesh types.
    ///
    /// Returns a cursor pointing to an arbitrary edge in the inserted mesh.
    /// The cursor is void if the imported mesh is empty.
    #[must_use]
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
        // TODO: Get rid of these halfedge specific constraints
        T2::Edge: HalfEdge<T2>,
        T2::Mesh: HalfEdgeMesh<T2>;

    /// Copy a triangle mesh based on indexed vertices into the current mesh.
    fn import_indexed_triangles<'a>(
        &'a mut self,
        indices: Vec<T::V>,
        vertices: Vec<T::VP>,
    ) -> EdgeCursorMut<'a, T>
    where
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload;
}
