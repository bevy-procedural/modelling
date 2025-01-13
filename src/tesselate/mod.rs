//! Triangulation Algorithms

// TODO: move this whole module to a separate crate!

mod convex;
mod delaunay;
mod ear_clipping;
mod fixed_n;
mod min_weight_dynamic;
mod min_weight_greedy;
mod sweep;

pub use convex::*;
pub use delaunay::*;
pub use ear_clipping::*;
pub use fixed_n::*;
pub use min_weight_dynamic::*;
pub use min_weight_greedy::*;
pub use sweep::*;

use crate::{
    math::IndexType,
    mesh::{FaceBasics, MeshType3D, Triangulation},
};

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

    /// The sweep-line algorithm, but with a O(n^2) dynamic programming min-weight algorithm running on each monotone sub-polygon.
    SweepDynamic,

    /// The sweep-line algorithm, but with a delaunay triangulation running on each monotone sub-polygon.
    SweepDelaunay,

    /// Slow, but large flat surfaces might render faster. Currently uses [Spade](https://github.com/Stoeoef/spade). TODO: allow Delaunay refinements! Runs in O(n log n) time. TODO: Isn't constrained delaunay O(n^2)?
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

/// Triangulate a face using the specified algorithm.
pub fn triangulate_face<T: MeshType3D>(
    face: &T::Face,
    mesh: &T::Mesh,
    tri: &mut Triangulation<T::V>,
    algorithm: TriangulationAlgorithm,
) {
    let n = face.num_vertices(mesh);

    // TODO: ignore faces with less than 3 vertices
    if n < 3 {
        return;
    }
    /*assert!(
        n >= 3,
        "a face must have at least 3 vertices, but {} only had {}",
        face.id(),
        n
    );*/

    if try_min_weight_small::<T>(face, mesh, tri) {
        return;
    }

    match algorithm {
        TriangulationAlgorithm::Auto => {
            // TODO: make something smarter
            delaunay_triangulation::<T>(face, mesh, tri);
        }
        TriangulationAlgorithm::EarClipping => {
            ear_clipping::<T>(face, mesh, tri, false);
        }
        TriangulationAlgorithm::Sweep => {
            let mut meta = TesselationMeta::default();
            sweep_line::<T, LinearMonoTriangulator<T::V, T::Vec2>>(face, mesh, tri, &mut meta);
        }
        TriangulationAlgorithm::SweepDynamic => {
            let mut meta = TesselationMeta::default();
            sweep_line::<T, DynamicMonoTriangulator<T::V, T::Vec2, T::Poly>>(
                face, mesh, tri, &mut meta,
            );
        }
        TriangulationAlgorithm::SweepDelaunay => {
            let mut meta = TesselationMeta::default();
            sweep_line::<T, DelaunayMonoTriangulator<T::V, T::Vec2>>(face, mesh, tri, &mut meta);
        }
        TriangulationAlgorithm::MinWeight => {
            minweight_dynamic::<T>(face, mesh, tri);
        }
        TriangulationAlgorithm::Delaunay => {
            delaunay_triangulation::<T>(face, mesh, tri);
        }
        TriangulationAlgorithm::EdgeFlip => {
            todo!("TriangulationAlgorithm::EdgeFlip is not implemented yet");
        }
        TriangulationAlgorithm::Fan => {
            fan_triangulation::<T>(face, mesh, tri);
        }
        TriangulationAlgorithm::Heuristic => {
            todo!("TriangulationAlgorithm::Heuristic is not implemented yet");
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        bevy::{Bevy2DPolygon, BevyMesh2d, BevyMesh3d, BevyMeshType3d32},
        mesh::{MeshBasics, TransformableMesh},
    };
    use bevy::math::{Vec2, Vec3};

    #[test]
    fn test_edge_length() {
        let svg = "<path d='M913.88 548.4c-66.14 35.43-141.83-7.68-141.83-7.68-112.76-53.91-246.31-55.82-246.31-55.82-34.09-2.34-25.47-17.51-20.69-25.88 0.73-1.27 1.74-2.36 2.59-3.56a187.06 187.06 0 0 0 34.17-108.08c0-103.78-84.13-187.92-187.92-187.92C251 159.47 167.37 242.24 166 344.87c-1 3.81-42.28 9.32-73-5.06-40-18.71-25.08 73.65 42.35 95.45l-2.31-0.1c-28.06-1.52-30.8 7.68-30.8 7.68s-16.14 29.75 83.13 38.37c31.39 2.72 35.71 8.11 42 16.64 11.92 16.14 3.57 39.25-12.15 59-44.53 55.77-71.84 180.68 49.78 270.85 103.12 76.47 377.65 79.95 497.37-15.13 108-85.76 156.72-170.47 185.79-241.14 3.9-9.54 31.84-58.43-34.28-23.03z' fill='#DFEDFF'/>";
        let mut m2d = BevyMesh2d::from_svg(svg);
        let mesh = m2d
            .scale(&Vec2::splat(-0.004))
            .translate(&Vec2::new(2.0, 2.0))
            .to_3d(0.001);

        let mut meta = TesselationMeta::default();
        let mut indices = Vec::new();
        let mut tri = Triangulation::new(&mut indices);
        triangulate_face::<BevyMeshType3d32>(
            mesh.face(0),
            &mesh,
            &mut tri,
            TriangulationAlgorithm::MinWeight,
            &mut meta,
        );
        let vec2s = mesh.face(0).vec2s(&mesh);
        let vec_hm: HashMap<u32, Vec2> = vec2s.iter().map(|v| (v.index, v.vec)).collect();
        tri.verify_full::<Vec2, Bevy2DPolygon>(&vec2s);
        let w = tri.total_edge_weight(&vec_hm);

        let mut indices3 = Vec::new();
        let mut tri3 = Triangulation::new(&mut indices3);
        delaunay_triangulation::<BevyMeshType3d32>(mesh.face(0), &mesh, &mut tri3);
        let w3 = tri3.total_edge_weight(&vec_hm);

        let mut indices4 = Vec::new();
        let mut tri4 = Triangulation::new(&mut indices4);
        let mut meta = TesselationMeta::default();
        sweep_line::<BevyMeshType3d32>(mesh.face(0), &mesh, &mut tri4, &mut meta);
        let w4 = tri4.total_edge_weight(&vec_hm);

        assert!(false, "Edge w: {} delaunay: {} Sweep: {}", w, w3, w4);
    }
}
*/
