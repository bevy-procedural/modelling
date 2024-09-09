//! Triangulation Algorithms

use super::{Face, Mesh, Payload};
use crate::{
    math::{Vector, Vector3D},
    representation::IndexType,
};
use itertools::Itertools;

mod convex;
mod delaunay;
mod ear_clipping;
mod min_weight;
mod triangulation;
pub use triangulation::{IndexedVertex2D, Triangulation};

/// The Sweep-line triangulation algorithm
pub mod sweep;

/// The algorithm to use for triangulating a face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TriangulationAlgorithm {
    /// Extremely fast, but only works for convex polygons. And even then, results are usually numerically unstable. Runs in O(n) time.
    Fan,

    /// Simple but slow textbook-algorithm for reference. Runs in O(n^2) time.
    EarClipping,

    /// Very fast sweep-line algorithm that might produces triangulations with unnecessarily long edges. Works for arbitrary polygons (yes, they don't have to be simple). Runs in O(n log n) time. See [CMSC 754](https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf).
    Sweep,

    /// Slow, but large flat surfaces might render faster. Currently uses [Spade](https://github.com/Stoeoef/spade). TODO: allow Delaunay refinements! Runs in O(n log n) time.
    Delaunay,

    /// Same output as Delaunay, but without external dependencies and using a very slow edge-flipping algorithm. Runs in O(n^3) time.
    EdgeFlip,

    /// Minimizes the overall edge length of the triangulation. Very slow, but produces the theoretically fastest rendering triangulations for large flat surfaces. Runs in O(2^n) time.
    MinWeight,

    /// Heuristic algorithm that tries to find a compromise between the speed of `Sweep` and the quality of `EdgeMin`.
    Heuristic,

    /// Automatically choose the "best" algorithm based on the input, i.e., with the given ratio of numerical stability and performance. Currently, it uses specialized implementations for the smallest polygons, then uses `Delaunay`, then `Heuristic`, and finally falls back to `Sweep` for the largest polygons.
    #[default]
    Auto,
}

/// The algorithm to use for generating normals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GenerateNormals {
    /// Do not generate normals. (no vertex duplication)
    None,

    /// Generate flat normals per face. (full vertex duplication)
    #[default]
    Flat,

    /// Generate smooth normals for smooth surfaces. (partial vertex duplication)
    Smooth,

    /// Generate only smooth normals. (no vertex duplication)
    AllSmooth,
}

/// Meta information for debugging the tesselation algorithm
#[derive(Debug, Clone, Default)]
pub struct TesselationMeta<V: IndexType> {
    /// Meta information for debugging the sweep algorithm
    pub sweep: sweep::SweepMeta<V>,
}

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    fn tesselate_inner<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        algorithm: TriangulationAlgorithm,
        meta: &mut TesselationMeta<V>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        let mut tri = Triangulation::new(indices);

        let n = self.num_vertices(mesh);
        if n < 3 {
            return;
        } else if n == 3 {
            let (a, b, c) = self.vertices(mesh).map(|v| v.id()).collect_tuple().unwrap();
            tri.insert_triangle(a, b, c);
            return;
        } else if n == 4 {
            self.quad_triangulate(mesh, &mut tri);
            return;
        }

        match algorithm {
            TriangulationAlgorithm::Auto => {
                // TODO: find a good threshold
                if n < 15 {
                    self.ear_clipping(mesh, &mut tri, false);
                } else {
                    self.sweep_line(mesh, &mut tri, meta);
                }
            }
            TriangulationAlgorithm::EarClipping => {
                self.ear_clipping(mesh, &mut tri, false);
            }
            TriangulationAlgorithm::Sweep => {
                self.sweep_line(mesh, &mut tri, meta);
            }
            TriangulationAlgorithm::MinWeight => {
                self.min_weight_triangulation_stoch(mesh, indices);
                todo!("TriangulationAlgorithm::MinWeight is not implemented yet");
            }
            TriangulationAlgorithm::Delaunay => {
                self.delaunay_triangulation(mesh, &mut tri);
            }
            TriangulationAlgorithm::EdgeFlip => {
                todo!("TriangulationAlgorithm::EdgeFlip is not implemented yet");
            }
            TriangulationAlgorithm::Fan => {
                todo!("TriangulationAlgorithm::Fan is not implemented yet");
            }
            TriangulationAlgorithm::Heuristic => {
                todo!("TriangulationAlgorithm::Heuristic is not implemented yet");
            }
        }

        // TODO: verify tri.ve
    }

    /// Converts the face into a triangle list.
    /// Might duplicate vertices when generating normals.
    /// When vertices is empty, use the original vertex indices without duplication.
    pub fn tesselate<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        vertices: &mut Vec<P>,
        indices: &mut Vec<V>,
        algorithm: TriangulationAlgorithm,
        generate_normals: GenerateNormals,
        meta: &mut TesselationMeta<V>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        match generate_normals {
            GenerateNormals::None => {
                self.tesselate_inner(mesh, indices, algorithm, meta);
            }
            GenerateNormals::Flat => {
                todo!("GenerateNormals::Flat");
                let v0 = vertices.len();
                let normal = self.normal(mesh);
                self.vertices(mesh).for_each(|v| {
                    let mut p = v.payload().clone();
                    p.set_normal(normal);
                    vertices.push(p)
                });
                let mut local_indices = Vec::new();
                self.tesselate_inner(mesh, &mut local_indices, algorithm, meta);
                indices.extend(local_indices.iter().map(|i| V::new(v0 + i.index())));
            }
            GenerateNormals::Smooth => {
                todo!("GenerateNormals::Smooth");
                let v0 = vertices.len();
                let normal = self.normal(mesh);
                self.vertices(mesh)
                    .circular_tuple_windows::<(_, _, _)>()
                    .for_each(|(prev, v, next)| {
                        let mut p = v.payload().clone();
                        let mut no = v.vertex().normal(*prev.vertex(), *next.vertex());
                        if no.dot(&normal) < 0.0.into() {
                            no = -no;
                        }
                        p.set_normal(no);
                        vertices.push(p)
                    });
                let mut local_indices = Vec::new();
                self.tesselate_inner(mesh, &mut local_indices, algorithm, meta);
                let n: usize = self.num_vertices(mesh);
                indices.extend(
                    local_indices
                        .iter()
                        .map(|i| V::new(v0 + ((i.index() + n - 1) % n))),
                );
            }
            GenerateNormals::AllSmooth => {
                todo!("GenerateNormals::AllSmooth")
            }
        }

        assert!(indices.len() % 3 == 0, "{:?}", indices.len());
        if vertices.is_empty() {
            debug_assert!(indices.iter().all(|i| i.index() < mesh.max_vertex_index()));
        } else {
            debug_assert!(indices.iter().all(|i| i.index() < vertices.len()));
        }
    }
}
