use crate::{
    math::IndexType,
    mesh::{EdgeBasics, MeshBasics, MeshType, VertexBasics},
};
use std::collections::HashMap;

/// Basic Network science functionality for a mesh
pub trait NetworkScience<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Returns the adjacency matrix of the mesh.
    ///
    /// If `with_weights` is true, the matrix will contain the weights of the edges. panics if the mesh is not weighted.
    ///
    /// If `directed` is true, the matrix will not be symmetric. panics if the mesh is not directed.
    fn adjacency_matrix(&self, with_weights: bool, directed: bool) -> nalgebra::DMatrix<f64> {
        // TODO: detect weighted edges and directed edges
        // TODO: sparse matrix

        assert!(!with_weights, "weighted edges are not supported yet");
        assert!(!directed, "directed edges are not supported yet");
        assert!(
            self.has_consecutive_vertex_ids(),
            "vertex ids must be consecutive"
        );

        let n = self.num_vertices();
        let mut adj = nalgebra::DMatrix::from_element(n, n, 0.0);
        for e in self.edges() {
            let i = e.origin(self).id().index();
            let j = e.target(self).id().index();
            adj[(i, j)] = 1.0;
            adj[(j, i)] = 1.0;
        }
        adj
    }

    /// Iterates the degrees of the vertices. Sorted by vertex index.
    fn degrees(&self) -> impl Iterator<Item = (T::V, usize)> + '_ {
        // TODO: detect weighted edges and directed edges

        use itertools::Itertools;

        let mut degrees = HashMap::new();
        for v in self.vertices() {
            degrees.insert(v.id(), v.degree(self));
        }
        degrees
            .into_iter()
            .sorted_unstable_by_key(|(i, _)| i.index())
    }

    /// Returns the degree matrix
    fn degree_matrix(&self) -> nalgebra::DMatrix<f64> {
        let n = self.num_vertices();
        let mut deg = nalgebra::DMatrix::from_element(n, n, 0.0);
        for (i, d) in self.degrees() {
            deg[(i.index(), i.index())] = d as f64;
        }
        deg
    }

    /// Returns the Laplacian matrix
    fn laplacian_matrix(&self, with_weights: bool, directed: bool) -> nalgebra::DMatrix<f64> {
        self.degree_matrix() - self.adjacency_matrix(with_weights, directed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::HalfEdgeMesh;
    use nalgebra::DMatrix;

    #[test]
    fn test_adjacency_matrix() {
       // todo!();
    }
}
