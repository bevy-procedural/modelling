//! Triangulation Algorithms

use std::collections::HashMap;

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
    /// When the input provokes numerical instabilities, e.g., a very large cirlce, the algorithm switches to recovery mode running in up to O(n^3) time.
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
        tri: &mut Triangulation<V>,
        algorithm: TriangulationAlgorithm,
        meta: &mut TesselationMeta<V>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        let n = self.num_vertices(mesh);
        if n < 3 {
            return;
        } else if n == 3 {
            let (a, b, c) = self.vertices(mesh).map(|v| v.id()).collect_tuple().unwrap();
            tri.insert_triangle(a, b, c);
            return;
        } else if n == 4 {
            self.quad_triangulate(mesh, tri);
            return;
        }

        match algorithm {
            TriangulationAlgorithm::Auto => {
                // TODO: find a good threshold
                if n < 15 {
                    self.ear_clipping(mesh, tri, false);
                } else {
                    self.sweep_line(mesh, tri, meta);
                }
            }
            TriangulationAlgorithm::EarClipping => {
                self.ear_clipping(mesh, tri, false);
            }
            TriangulationAlgorithm::Sweep => {
                self.sweep_line(mesh, tri, meta);
            }
            TriangulationAlgorithm::MinWeight => {
                //self.min_weight_triangulation_stoch(mesh, indices);
                todo!("TriangulationAlgorithm::MinWeight is not implemented yet");
            }
            TriangulationAlgorithm::Delaunay => {
                self.delaunay_triangulation(mesh, tri);
            }
            TriangulationAlgorithm::EdgeFlip => {
                todo!("TriangulationAlgorithm::EdgeFlip is not implemented yet");
            }
            TriangulationAlgorithm::Fan => {
                self.fan_triangulation(mesh, tri);
            }
            TriangulationAlgorithm::Heuristic => {
                todo!("TriangulationAlgorithm::Heuristic is not implemented yet");
            }
        }
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
        let mut tri = Triangulation::new(indices);

        match generate_normals {
            GenerateNormals::None => {
                self.tesselate_inner(mesh, &mut tri, algorithm, meta);
            }
            GenerateNormals::Flat => {
                let normal = self.normal(mesh);
                let mut id_map = HashMap::new();
                // generate a new list of vertices (full duplication)
                self.vertices(mesh).for_each(|v| {
                    let mut p = v.payload().clone();
                    id_map.insert(v.id(), V::new(vertices.len()));
                    p.set_normal(normal);
                    vertices.push(p)
                });
                self.tesselate_inner(mesh, &mut tri, algorithm, meta);
                tri.map_indices(&id_map);
            }
            GenerateNormals::Smooth => {
                // TODO: What's happening here? This doesn't look right at all.
                let normal = self.normal(mesh);
                let mut id_map = HashMap::new();
                self.vertices(mesh)
                    .circular_tuple_windows::<(_, _, _)>()
                    .for_each(|(prev, v, next)| {
                        let mut p = v.payload().clone();
                        let mut no = v.vertex().normal(*prev.vertex(), *next.vertex());
                        if no.dot(&normal) < 0.0.into() {
                            no = -no;
                        }
                        p.set_normal(no);
                        id_map.insert(v.id(), V::new(vertices.len()));
                        vertices.push(p)
                    });
                self.tesselate_inner(mesh, &mut tri, algorithm, meta);
               tri.map_indices(&id_map);
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
