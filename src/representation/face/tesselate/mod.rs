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

/// The Sweep-line triangulation algorithm
pub mod sweep;

/// The algorithm to use for triangulating a face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TriangulationAlgorithm {
    /// Extremely fast, but only works for convex polygons. And even then, results are usually numerically unstable. Runs in O(n) time.
    Fan,

    /// Simple but slow textbook-algorithm for reference. Runs in O(n^2) time.
    EarClipping,

    /// Very fast sweep-line algorithm without any guarantees on the quality of the triangulation. Works for arbitrary polygons (yes, they don't have to be simple). Runs in O(n log n) time. See [CMSC 754](https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf).
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
pub struct TesselationMeta {
    #[cfg(feature = "sweep_debug")]
    /// Meta information for debugging the sweep algorithm
    pub sweep: sweep::SweepMeta,
}

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Expand local indices to global indices if requested to.
    fn expand_local_indices<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        local_indices: bool,
        meta: &mut TesselationMeta,
        f: impl Fn(&Mesh<E, V, F, P>, &mut Vec<V>, &mut TesselationMeta),
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        // TODO: counting again and again is rather slow. Cache this values
        let n = self.num_vertices(mesh);
        let v0: usize = indices.len();
        f(mesh, indices, meta);
        if !local_indices {
            for i in v0..indices.len() {
                indices[i] = V::new(v0 + (indices[i].index() + 1) % n);
            }
            
            #[cfg(feature = "sweep_debug")]
            meta.sweep.expand(v0, n);
        }
    }

    fn tesselate_inner<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        algorithm: TriangulationAlgorithm,
        local_indices: bool,
        meta: &mut TesselationMeta,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        let n = self.num_vertices(mesh);
        if n < 3 {
            return;
        } else if n == 3 {
            if local_indices {
                indices.push(V::new(0));
                indices.push(V::new(1));
                indices.push(V::new(2));
            } else {
                indices.extend(self.vertices(mesh).map(|v| v.id()));
            }
            return;
        } else if n == 4 {
            self.quad_triangulate(mesh, indices, local_indices);
            return;
        }

        match algorithm {
            TriangulationAlgorithm::Auto => {
                if n < 15 {
                    // TODO: find a good threshold
                    self.ear_clipping(mesh, indices, local_indices, false);
                } else {
                    self.expand_local_indices(
                        mesh,
                        indices,
                        local_indices,
                        meta,
                        |mesh, indices, meta| self.sweep_line(mesh, indices, meta),
                    );
                }
            }
            TriangulationAlgorithm::EarClipping => {
                self.ear_clipping(mesh, indices, local_indices, false);
            }
            TriangulationAlgorithm::Sweep => {
                self.expand_local_indices(
                    mesh,
                    indices,
                    local_indices,
                    meta,
                    |mesh, indices, meta| self.sweep_line(mesh, indices, meta),
                );
            }
            TriangulationAlgorithm::MinWeight => {
                assert!(local_indices == false);
                self.min_weight_triangulation_stoch(mesh, indices);
            }
            TriangulationAlgorithm::Delaunay => {
                self.expand_local_indices(
                    mesh,
                    indices,
                    local_indices,
                    meta,
                    |mesh, indices, _| {
                        self.delaunay_triangulation(mesh, indices);
                    },
                );
            }
            TriangulationAlgorithm::EdgeFlip => {
                panic!("TriangulationAlgorithm::EdgeFlip is not implemented yet");
            }
            TriangulationAlgorithm::Fan => {
                panic!("TriangulationAlgorithm::Fan is not implemented yet");
            }
            TriangulationAlgorithm::Heuristic => {
                panic!("TriangulationAlgorithm::Heuristic is not implemented yet");
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
        meta: &mut TesselationMeta,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        match generate_normals {
            GenerateNormals::None => {
                self.tesselate_inner(mesh, indices, algorithm, false, meta);
            }
            GenerateNormals::Flat => {
                let v0 = vertices.len();
                let normal = self.normal(mesh);
                self.vertices(mesh).for_each(|v| {
                    let mut p = v.payload().clone();
                    p.set_normal(normal);
                    vertices.push(p)
                });
                let mut local_indices = Vec::new();
                self.tesselate_inner(mesh, &mut local_indices, algorithm, true, meta);
                indices.extend(local_indices.iter().map(|i| V::new(v0 + i.index())));
            }
            GenerateNormals::Smooth => {
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
                self.tesselate_inner(mesh, &mut local_indices, algorithm, true, meta);
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
