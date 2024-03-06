use super::{Face, Mesh, Payload};
use crate::{
    math::{Vector, Vector2D, Vector3D},
    representation::IndexType,
};
use itertools::Itertools;

mod convex;
mod delaunay;
mod ear_clipping;
mod min_weight;
mod sweep;

/// The algorithm to use for triangulating a face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TriangulationAlgorithm {
    /// The fastest algorithm (Ear Clipping for small triangles, Sweep for larger ones).
    #[default]
    Fast,

    /// The Ear Clipping algorithm. Works for arbitrary simple polygons. Runs in O(n^2) time.
    EarClipping,

    /// The Sweep algorithm. Works for arbitrary polygons (yes, they don't have to be simple). Runs in O(n log n) time.
    Sweep,

    /// Min-Weight Triangulation. Results can be rendered slightly faster on most graphics hardware. Runs in O(n^3) time.
    MinWeight,

    /// Delaunay Triangulation. Results are optimized for numerical stability. Currently runs in O(n^3) time (TODO: optimize).
    Delaunay,
}

/// The algorithm to use for generating normals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GenerateNormals {
    /// Do not generate normals. (no vertex duplication)
    None,

    /// Generate flat normals per face. (some vertex duplication)
    #[default]
    Flat,

    /// Generate smooth normals for smooth surfaces. (some vertex duplication)
    Smooth,

    /// Generate only smooth normals. (no vertex duplication)
    AllSmooth,
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
        local_indices: bool,
    ) where
        P::Vec: Vector3D<P::S>,
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
            TriangulationAlgorithm::Fast => {
                if n < 10 {
                    self.ear_clipping(mesh, indices, local_indices, false);
                } else {
                    self.sweep_line(mesh, indices, local_indices);
                }
            }
            TriangulationAlgorithm::EarClipping => {
                self.ear_clipping(mesh, indices, local_indices, false);
            }
            TriangulationAlgorithm::Sweep => {
                self.sweep_line(mesh, indices, local_indices);
            }
            TriangulationAlgorithm::MinWeight => {
                assert!(local_indices == false);
                self.min_weight_triangulation_stoch(mesh, indices);
            }
            TriangulationAlgorithm::Delaunay => {
                self.ear_clipping(mesh, indices, local_indices, false);
                if local_indices {
                    self.delaunayfy(mesh, indices, local_indices);
                } else {
                    let il = indices.len();
                    let nv = (self.num_vertices(mesh) - 2) * 3;
                    let mut i = indices[(il - nv)..].to_vec();
                    self.delaunayfy(mesh, &mut i, local_indices);
                    indices[(il - nv)..].copy_from_slice(&i);
                }
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
    ) where
        P::Vec: Vector3D<P::S>,
    {
        match generate_normals {
            GenerateNormals::None => {
                self.tesselate_inner(mesh, indices, algorithm, false);
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
                self.tesselate_inner(mesh, &mut local_indices, algorithm, true);
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
                self.tesselate_inner(mesh, &mut local_indices, algorithm, true);
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
