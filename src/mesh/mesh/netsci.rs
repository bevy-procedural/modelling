use crate::{
    math::{IndexType, Scalar, Vector},
    mesh::{cursor::*, EuclideanMeshType, MeshBasics, MeshType},
};
use std::collections::HashMap;

/// Basic Network science functionality for a mesh
pub trait NetworkScience<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Returns the adjacency matrix of the mesh.
    /// All weights will be 1 and the graph will be treated as undirected.
    fn adjacency_matrix<S: Scalar>(&self) -> nalgebra::DMatrix<S> {
        assert!(
            self.has_consecutive_vertex_ids(),
            "vertex ids must be consecutive"
        );

        let n = self.num_vertices();
        let mut adj = nalgebra::DMatrix::from_element(n, n, S::ZERO);
        for e in self.edges() {
            let i = e.origin_id().index();
            let j = e.target_id().index();
            adj[(i, j)] = S::ONE;
            adj[(j, i)] = S::ONE;
        }
        adj
    }

    /// Returns the adjacency matrix of the mesh.
    /// Euclidean distance will be used as weight. The graph will be treated as undirected.
    fn adjacency_matrix_euclidean<const D: usize>(&self) -> nalgebra::DMatrix<T::S>
    where
        T: EuclideanMeshType<D>,
    {
        assert!(
            self.has_consecutive_vertex_ids(),
            "vertex ids must be consecutive"
        );

        let n = self.num_vertices();
        let mut adj = nalgebra::DMatrix::from_element(n, n, Scalar::ZERO);
        for e in self.edges() {
            let i = e.origin_id().index();
            let j = e.target_id().index();
            // TODO: avoid unwrap
            let d: T::S = e
                .fork()
                .origin()
                .unwrap()
                .pos()
                .distance(&e.target().unwrap().pos());
            adj[(i, j)] = d;
            adj[(j, i)] = d;
        }
        adj
    }

    /// Iterates the degrees of the vertices. Sorted by vertex index.
    fn degrees(&self) -> impl Iterator<Item = (T::V, usize)> + '_ {
        // TODO: detect weighted edges and directed edges

        use itertools::Itertools;

        let mut degrees = HashMap::new();
        for v in self.vertices() {
            degrees.insert(v.id(), v.degree());
        }
        degrees
            .into_iter()
            .sorted_unstable_by_key(|(i, _)| i.index())
    }

    /// Returns the degree matrix
    fn degree_matrix<S: Scalar>(&self) -> nalgebra::DMatrix<S> {
        let n = self.num_vertices();
        let mut deg = nalgebra::DMatrix::from_element(n, n, S::ZERO);
        for (i, d) in self.degrees() {
            deg[(i.index(), i.index())] = S::from_usize(d);
        }
        deg
    }

    /// Returns the Laplacian matrix. All weights will be 1 and the graph will be treated as undirected.
    fn laplacian<S: Scalar>(&self) -> nalgebra::DMatrix<S> {
        self.degree_matrix::<S>() - self.adjacency_matrix::<S>()
    }

    /// Returns the Laplacian matrix. Euclidean distance will be used as weight. The graph will be treated as undirected.
    fn laplacian_euclidean<const D: usize>(&self) -> nalgebra::DMatrix<T::S>
    where
        T: EuclideanMeshType<D>,
    {
        self.degree_matrix() - self.adjacency_matrix_euclidean::<D>()
    }
}

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use super::*;
    use crate::{extensions::nalgebra::MeshNd64, math::Scalar, prelude::MakePrismatoid};
    use itertools::Itertools;

    #[test]
    fn test_adjacency_matrix() {
        let mesh = MeshNd64::<3>::cube(1.0);
        let adj = mesh.adjacency_matrix::<f64>();
        let eig = adj
            .eigenvalues()
            .expect("Should have eigenvalues")
            .iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .cloned()
            .collect_vec();

        println!("adjacency matrix:\n{}", adj);
        println!("eigenvalues: {:?}", eig);

        // 3, 1 (with multiplicity 3), -1 (with multiplicity 3), and -3.
        assert!(eig[0].is_about(-3.0, 1e-10));
        assert!(eig[1].is_about(-1.0, 1e-10));
        assert!(eig[2].is_about(-1.0, 1e-10));
        assert!(eig[3].is_about(-1.0, 1e-10));
        assert!(eig[4].is_about(1.0, 1e-10));
        assert!(eig[5].is_about(1.0, 1e-10));
        assert!(eig[6].is_about(1.0, 1e-10));
        assert!(eig[7].is_about(3.0, 1e-10));
    }

    #[test]
    fn test_laplacian_matrix() {
        let mesh = MeshNd64::<3>::cube(1.0);
        let lap = mesh.laplacian::<f64>();
        let eig = lap
            .eigenvalues()
            .expect("Should have eigenvalues")
            .iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .cloned()
            .collect_vec();

        println!("laplacian matrix:\n{}", lap);
        println!("eigenvalues: {:?}", eig);

        // smallest one should always be 0 since there is exactly one connected component
        assert!(eig[0].is_about(0.0, 1e-10));

        let algebraic_connectivity = eig[1];
        assert!(algebraic_connectivity.is_about(2.0, 1e-10));
    }
}
