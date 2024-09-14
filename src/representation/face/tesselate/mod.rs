//! Triangulation Algorithms

use super::{Face, FacePayload, Mesh};
use crate::{
    math::Vector3D,
    representation::{payload::HasPosition, IndexType, MeshType},
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
    /// When the input provokes numerical instabilities, e.g., a very large circle, the algorithm switches to recovery mode running in up to O(n^3) time.
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

    /// Automatically choose the "best" algorithm based on the input, i.e., with the given ratio of numerical stability and performance.
    #[default]
    Auto,
}

/// Meta information for debugging the tesselation algorithm
#[derive(Debug, Clone, Default)]
pub struct TesselationMeta<V: IndexType> {
    /// Meta information for debugging the sweep algorithm
    pub sweep: sweep::SweepMeta<V>,
}

impl<E: IndexType, F: IndexType, FP: FacePayload> Face<E, F, FP> {
    /// Converts the face into a triangle list.
    pub fn triangulate<T: MeshType<E = E, F = F, FP = FP>>(
        &self,
        mesh: &Mesh<T>,
        tri: &mut Triangulation<T::V>,
        algorithm: TriangulationAlgorithm,
        meta: &mut TesselationMeta<T::V>,
    ) where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
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
                // TODO: make something smarter
                self.delaunay_triangulation(mesh, tri);
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
}
