# Procedural Modelling

[![Documentation](https://docs.rs/procedural_modelling/badge.svg)](https://docs.rs/procedural_modelling)
[![crates.io](https://img.shields.io/crates/v/procedural_modelling)](https://crates.io/crates/procedural_modelling)
[![Downloads](https://img.shields.io/crates/d/procedural_modelling)](https://crates.io/crates/procedural_modelling)
[![License](https://img.shields.io/crates/l/procedural_modelling)](https://bevyengine.org/learn/quick-start/plugin-development/#licensing)
[![Build Status](https://github.com/bevy-procedural/modelling/actions/workflows/rust.yml/badge.svg)](https://github.com/bevy-procedural/modelling/actions)
[![GitHub Repo stars](https://img.shields.io/github/stars/bevy-procedural/modelling)](https://github.com/bevy-procedural/modelling)

A Framework-Agnostic Procedural Modelling Library.

Uses a data structure based on half-edge meshes to represent (open) manifold meshes with optional non-manifold vertices. Our goal is to implement operations like Boolean Operations, Subdivisions, Curved Surfaces, and Stitching. The library aims to support the triangulation of 2d surfaces in a modular way that can be used without any of the 3d features.

Currently there are quite a few crates that implement boolean operations and triangulation to achieve some other goal. We want to provide a common implementation to satisfy these very similar requirements and improve code-reuse among these projects so they can focus on their original goal.

## WARNING

This crate is still in a _very_ early stage of development. Expect frequent API modifications, bugs, and missing features. Feel free to contribute by opening issues, pull requests or sharing your ideas in [GitHub Discussion](https://github.com/bevy-procedural/modelling/discussions).

## Usage

<img src="assets/demo.png" alt="drawing" width="300"/>

Install using `cargo add procedural_modelling`. Generate the above mesh using the following code:

```rs
let mut mesh = BevyMesh3d::new();
let mut edge = mesh.insert_regular_star(1.0, 0.8, 30);
mesh.flip_yz().translate(&Vec3::new(0.0, -0.99, 0.0));
let trans = Transform::from_rotation(Quat::from_rotation_y(0.3))
    .with_translation(Vec3::new(-0.2, 0.3, -0.3));
edge = mesh.extrude_tri(edge, trans);
for _ in 0..5 {
    edge = mesh.extrude_tri_face(mesh.edge(edge).face_id(), trans);
}
mesh.to_bevy(RenderAssetUsages::default())
```

## Examples

Try the live examples!
 * [box](https://bevy-procedural.org/examples/modelling/box) demonstrates different methods to build a cube from scratch. This is a good place to get started with this crate!
 * [fps_bench](https://bevy-procedural.org/examples/modelling/fps_bench) benchmarks the rendering performance of the different triangulation algorithms.

Or run the [examples](https://github.com/bevy-procedural/modelling/tree/main/examples) on your computer like, e.g., `cargo run --features="bevy bevy/bevy_pbr bevy/bevy_winit bevy/tonemapping_luts" --profile fast-dev --example box`.

For package development, we recommend using the `editor`-subcrate. This example has a little [egui](https://github.com/jakobhellermann/bevy-inspector-egui/)-editor. Run it using `cargo watch -w editor/src -w src -x "run -p editor --profile fast-dev"`. The `fast-dev` profile will enable optimizations for the dependencies, but not for the package itself. This will slow down the first build _significantly_, but incremental builds are slightly faster and bevy's performance (bevy is used as the renderer in the examples) improves a lot.

When developing tests, we recommend `cargo watch -w editor/src -w src -x "test --profile fast-dev"`.

## Feature Progress

-   Attributes

    -   [x] Positions
    -   [x] Normals (flat, smooth)
    -   [x] Custom Attributes
    -   [ ] crease weights, surface groups
    -   [ ] Tangents
    -   [ ] UV Coordinates

-   Mesh Types

    -   [x] 2d Surface
    -   [x] 3d Open Manifold
    -   [ ] 4d Open Manifold
    -   [ ] Pseudomanifold (with singularities)
    -   [ ] Non-Manifold (with branching surfaces)
    -   [ ] Non-Euclidean Open Manifold
    -   [ ] Combinatorial (purely topo\logical)

-   Triangulation

    -   [x] Fan
    -   [x] Ear Clipping
    -   [x] Montone Sweep-Line
    -   [x] Constrained Delaunay (using [Spade](https://github.com/Stoeoef/spade))
    -   [ ] Steiner Points

-   Primitives

    -   [x] Polygon, Star, Loop
    -   [x] Cuboid, Cube
    -   [x] Cylinder, Cone
    -   [x] Prism, Antiprism
    -   [x] Pyramid, Frustum, Tetrahedron, Octahedron, Dodecahedron
    -   [ ] Icosahedron
    -   [x] UV Sphere
    -   [ ] Cube Sphere
    -   [ ] Icosphere
    -   [ ] Torus

-   Builder Primitives

    -   [x] Lines
    -   [ ] Quadratic Bezier Curves
    -   [ ] Cubic Bezier Curves
    -   [ ] Curved Surfaces (Bezier Surfaces / Parametric Surfaces / NURBS / Spline Networks...?)

-   Operations

    -   [x] Extrude & Linear Loft (Quad, Triangle)
    -   [x] Transform (Translate, Rotate, Scale, [ ] Shear)
    -   [ ] Taper
    -   [ ] Chamfer / Cantellate / Bevel / Truncate / Bitruncate / Omnitruncate
    -   [ ] Boolean Operations (Union, Intersection, Difference, Symmetric Difference)
    -   [ ] (Anisotropic) Simplification / LODs
    -   [ ] Stitch
    -   [ ] Subdivide
    -   [ ] Snub
    -   [ ] Inset
    -   [ ] Dualize
    -   [ ] Stellate
    -   [ ] Plane Intersection
    -   [ ] Morph
    -   [ ] Voxelate
    -   [ ] Smooth
    -   [ ] Nonlinear Loft
    -   [ ] Bridge
    -   [ ] Reflect
    -   [ ] Weld
    -   [ ] Twist
    -   [ ] Offset
    -   [ ] Inflate / Deflate
    -   [ ] Convex Hull
    -   [ ] Collapse
    -   [ ] Split
    -   [ ] Lattice
    -   [ ] Refine
    -   [ ] Crease
    -   [ ] Simplify
    -   [ ] Fractalize
    -   [ ] Project

-   Tools

    -   [ ] Geodesic Pathfinding
    -   [ ] Raycasting
    -   [ ] Topo\logy Analysis
    -   [ ] Spatial Data Structures

-   Debug Visualizations

    -   [x] Indices
    -   [ ] Normals
    -   [ ] Tangents
    -   [ ] Topo\logy

-   Backends

    -   [x] Bevy
    -   [ ] wgpu
    -   [ ] STL export/import
    -   [ ] OBJ export/import

## Features

The following cargo features are available:

-   `meshopt` -- Use [Meshopt](https://github.com/gwihlidal/meshopt-rs) to optimize the performance of generated meshes.
-   `bevy` -- Compiles with support for bevy. Necessary for the examples and the editor.
-   `benchmarks` -- Enable [criterion](https://github.com/bheisler/criterion.rs) for the benchmarks.

For development only:

-   `sweep_debug` -- Collect debug information during the sweepline triangulation and enable visualizations in the bevy backend.
-   `sweep_debug_print` -- Print debug information for the sweepline triangulation.

## Triangulation algorithms

The package supports different triangulation algorithms. The robustness and rendering speed of the produced triangulations varies significantly. These performance differences usually only matter for meshes with extremely large flat surfaces. In the table below, we compare the performance of the different algorithms for a circle with 100, 1000, and 10000 vertices. The "ZigZag" mesh has 10000 vertices and is designed to demonstrate the worst-case for the Sweep algorithm.

-   **Fan** Extremely fast, but only works for convex polygons. And even then, results are usually numerically unstable. Runs in $\mathcal{O}(n)$ time.
-   **EarClipping** Simple but slow textbook-algorithm for reference. Runs in $\mathcal{O}(n^2)$ time. When the input provokes numerical instabilities, e.g., a very large circle, the algorithm switches to recovery mode running in up to $\mathcal{O}(n^3)$ time.
-   **Sweep** Very fast sweep-line algorithm that might produces triangulations with unnecessarily long edges. Works for arbitrary polygons (yes, they don't have to be simple). Runs in $\mathcal{O}(n \log n)$ time. See [CMSC 754](https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf).
-   **Delaunay** Slow, but large flat surfaces might render faster. Currently uses [Spade](https://github.com/Stoeoef/spade). Runs in $\mathcal{O}(n \log n)$ time.
-   **EdgeFlip** Same output as Delaunay, but without external dependencies and using a very slow edge-flipping algorithm. Runs in $\mathcal{O}(n^3)$ time.
    EdgeFlip,
-   **MinWeight** Minimizes the overall edge length of the triangulation. Very slow, but produces the theoretically fastest rendering triangulations for large flat surfaces. Runs in $\mathcal{O}(2^n)$ time.
-   **Heuristic** Heuristic algorithm that tries to find a compromise between the speed of `Sweep` and the quality of `EdgeMin`.
-   **Auto** (default) Automatically choose the "best" algorithm based on the input, i.e., with the given ratio of numerical stability and performance.

| Algorithm   | Requirements | Worst Case | Circle 10        | Circle 100         | Circle 1000       | Circle 10000      | ZigZag 1000       | ZigZag 10000      |
| ----------- | ------------ | ---------- | ---------------- | ------------------ | ----------------- | ----------------- | ----------------- | ----------------- |
| Fan         | Convex       | $n$          | 0.258µs (151fps) | 2.419µs¹ (118fps)² | 15.41µs (52.4fps) | 161.8µs (12.5fps) | -                 | -                 |
| EarClipping | Simple       | $n^2$        | 0.746µs (150fps) | 21.75µs (118fps)   | 1.746ms (52.1fps) | 3.276s (11.4fps)  | 49.10ms (35.6fps) | 46.03s (9.51fps)  |
| Sweep       | None         | $n \log n$    | 1.584µs (151fps) | 13.58µs (118fps)   | 142.4µs (44.2fps) | 1.556ms (9.87fps) | 402.3µs (42.8fps) | 4.334ms (9.87fps) |
| Delaunay    | Simple       | $n \log n$    | 2.778µs (151fps) | 29.89µs (134fps)   | 308.5µs (132fps)  | 3.296ms (129fps)  | 3.002ms (42.1fps) | 158.7ms (9.33fps) |
| EdgeFlip    | Simple       | $n^3$        |                  |                    |                   |                   |                   |
| MinWeight   | Simple       | $2^n$        |                  |                    |                   |                   |                   |
| Heuristic   | Simple       | $n \log n$    |                  |                    |                   |                   |                   |
| Auto        | Simple       | $n \log n$    |                  |                    |                   |                   |                   |

-   ¹) Time for the triangulation on a Intel i7-12700K (single threaded). Run the benchmarks using `cargo bench --features benchmarks`.
-   ²) FPS when rendering 100 large, transparent instances with the bevy 0.14.2 pbr shader on a Nvidia GeForce RTX 4060 Ti in Full HD. See `cargo run --example fps_bench --profile release --features="bevy bevy/bevy_pbr bevy/bevy_winit bevy/tonemapping_luts"`. For the non-Delaunay algorithms, the rendering time deteriorates for the larger circles since the edge length is not minimized causing significant overdraw.

## Supported Bevy Versions

The following table shows the compatibility of `procedural_modelling` (when using the `bevy` feature) with certain versions of Bevy:

| bevy | bevy_procedural_meshes |
| ---- | ---------------------- |
| 0.14 | 0.2.\*, main           |
| 0.13 | 0.1.\*                 |

## License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed, allowing you the flexibility to choose between:

-   The MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
-   The Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## How to Contribute

We welcome contributions from the community! Here are some ways you can help:

1.  **Report Bugs:**

    -   If you find a bug, please open an issue on GitHub with detailed information on how to reproduce it.

2.  **Suggest Features:**

    -   Have an idea for a new feature? Open an issue to discuss it. We appreciate feedback and suggestions.

3.  **Submit Pull Requests:**

    -   Fork the repository and create a new branch for your feature or bug fix.
    -   Assign an issue to yourself or open a new issue to work on.
    -   Make your changes, ensuring that your code adheres to the project's coding standards.
    -   Write tests for your changes, if applicable.
    -   Submit a pull request with a clear description of your changes and the problem they solve.

4.  **Improve Documentation:**
    -   Help us improve our documentation by fixing typos, clarifying instructions, or adding new sections.
