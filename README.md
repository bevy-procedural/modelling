# Procedural Modelling

*A framework-agnostic toolkit for constructive geometry and mesh processing.*

[![Documentation](https://docs.rs/procedural_modelling/badge.svg)](https://docs.rs/procedural_modelling)
[![crates.io](https://img.shields.io/crates/v/procedural_modelling)](https://crates.io/crates/procedural_modelling)
[![Downloads](https://img.shields.io/crates/d/procedural_modelling)](https://crates.io/crates/procedural_modelling)
[![License](https://img.shields.io/crates/l/procedural_modelling)](https://bevy.org/learn/quick-start/plugin-development/#licensing)
[![Build Status](https://github.com/bevy-procedural/modelling/actions/workflows/rust.yml/badge.svg)](https://github.com/bevy-procedural/modelling/actions)
[![GitHub Repo stars](https://img.shields.io/github/stars/bevy-procedural/modelling)](https://github.com/bevy-procedural/modelling)

`procedural_modelling` provides a **functional, trait-based API** for building and analysing 2-, 3- and _n_-dimensional meshes. It is:

-   **Renderer-agnostic** – works with either the `bevy`, `wgpu`, or `nalgebra` back-end (Bevy 0.16 supported).
-   **Versatile** – supports half-edge meshes, Bézier curves, subdivision, triangulation, and more. Boolean operations, stitching, and non-manifold meshes are planned.
-   **Extensible** – implement a few lightweight traits and the algorithms operate on _your_ mesh data structures, payloads, and scalar types.
-   **Functional & safe** – immutable [cursors](https://docs.rs/procedural_modelling/latest/procedural_modelling/#cursors) let you traverse / edit meshes without interior mutability, minimizing bugs.

## WARNING

> This crate is still in an early stage of development; breaking changes may arrive without deprecation. Performance has not been tuned, and several algorithms are incomplete or unimplemented. Bug reports and PRs are welcome – see [How to Contribute](#how-to-contribute).

## Usage

<img src="doc/images/demo.png" alt="drawing" width="300"/>

Install using `cargo add procedural_modelling`. Generate the above mesh using the following code:

```rs
let mut mesh = Mesh3d64::new();
let trans = NdAffine::from_rotation(NdRotate::from_axis_angle(Vec3::z_axis(), 0.3))
    .with_translation(Vec3::new(-0.2, -0.3, 0.3));
let mut edge = mesh.insert_regular_star(1.0, 0.8, 30).extrude_tri(&trans);
for _ in 0..5 {
    edge = edge.face().extrude_tri(&trans);
}
mesh.flip_yz().translate(&Vec3::new(0.0, -0.99, 0.0));
let svg_string = render2svg::<MeshType3d64PNU, _>(&mesh, &s, |t: f64|
    NdAffine::from_rotation(NdRotate::from_axis_angle(Vec3::<f64>::y_axis(), 0.8 * (std::f64::consts::PI * t).sin())));
```

A key component of this library are `Cursors`, which provide a convenient way to traverse and modify the mesh in a functional, performant, and safe way. See the [cursors tutorial](https://docs.rs/procedural_modelling/latest/procedural_modelling/#cursors) for more information.

## Examples

<!-- Try the live examples!
 TODO: bevy-procedural.org -->

-   [box](https://github.com/bevy-procedural/modelling/blob/main/examples/box.rs) demonstrates different methods to build a cube from scratch. This is a good place to get started with this crate!
-   [loft](https://github.com/bevy-procedural/modelling/blob/main/examples/loft.rs) demonstrates the usage of `loft` and `extrude`. These functions are extremely versatile and you should definitely look at this example when you plan to constructively model your meshes.
-   [path](https://github.com/bevy-procedural/modelling/blob/main/examples/path.rs) demonstrates the path builder with bezier curves.
-   [text](https://github.com/bevy-procedural/modelling/blob/main/examples/text.rs) demonstrates how to insert text into a mesh.
-   [zoo](https://github.com/bevy-procedural/modelling/blob/main/examples/zoo.rs) showcases a variety of different predefined 3d shapes.
-   [zoo_2d](https://github.com/bevy-procedural/modelling/blob/main/examples/zoo_2d.rs) showcases a variety of different predefined 2d shapes.
-   [svg](https://github.com/bevy-procedural/modelling/blob/main/examples/svg.rs) loads and renders a duck from a svg string.
-   [triangulation](https://github.com/bevy-procedural/modelling/blob/main/examples/triangulation.rs) demonstrates the different triangulation algorithms.
-   [fern](https://github.com/bevy-procedural/modelling/blob/main/examples/fern.rs) is a more advanced example creating a detailed fern leaf.
-   [custom_mesh_type](https://github.com/bevy-procedural/modelling/blob/main/examples/custom_mesh_type.rs) demonstrates how to define a custom mesh by extending the default implementation with vertex colors.

<!-- TODO: demonstrate smooth normals, 4d geometry, triangulation strategies, mesh comparison, net science -->

You can compile and run the [examples](https://github.com/bevy-procedural/modelling/tree/main/examples) like, e.g., `cargo run --features=bevy_example --profile fast-dev --example box`. The `fast-dev` profile will enable optimizations for the dependencies, but not for the package itself. This will slow down the first build _significantly_, but incremental builds are slightly faster and bevy's performance (bevy is used as the renderer in the examples) improves a lot.

## Tutorial

We are currently working on some tutorials for the most important features.

-   [Getting started](https://docs.rs/procedural_modelling/latest/procedural_modelling/#getting-started)
-   [Cursors](https://docs.rs/procedural_modelling/latest/procedural_modelling/#cursors)

## Feature Progress

-   Attributes

    -   [x] Positions
    -   [x] Normals ([flat](https://github.com/bevy-procedural/modelling/blob/main/examples/box.rs), smooth)
    -   [x] Custom Attributes
    -   [ ] Crease Weights, Surface Groups
    -   [ ] Tangents
    -   [ ] UV Coordinates

-   Mesh Types

    -   [x] Open PL 2-Manifold in 2d and 3d Space
    -   [x] [Bezier Curves for 2d Meshes](https://github.com/bevy-procedural/modelling/blob/main/examples/path.rs)
    -   [ ] Self-intersecting surfaces
    -   [ ] Open PL 2-Manifold in nd Space
    -   [ ] Open PL $n$-Manifold in md Space
    -   [ ] Pseudomanifold (with singularities)
    -   [ ] Non-Manifold (with branching surfaces)
    -   [ ] Non-Euclidean
    -   [ ] Arbitrary Graphs
    -   [ ] NURBS, T-Splines <!-- Bezier Surfaces, Parametric Surfaces, Spline Networks...? -->

-   Triangulation (comparison [below](#triangulation-algorithms), also see the [example](https://github.com/bevy-procedural/modelling/blob/main/examples/triangulation.rs))

    -   [x] Fan
    -   [x] Ear Clipping
    -   [x] Montone Sweep-Line
    -   [x] Constrained Delaunay (using [Spade](https://github.com/Stoeoef/spade))
    -   [ ] Constrained Delaunay (using Monotone Sweep-Line)
    -   [x] Min-Weight Triangulation (using Dynamic Programming)
    -   [ ] Min-Weight Heuristic in $\mathcal{O}(n \log n)$
    -   [ ] Steiner Points

-   Primitives

    -   [x] [2d stuff](https://github.com/bevy-procedural/modelling/blob/main/examples/zoo_2d.rs): Polygon, Star, Circle, Loop, ...
    -   [x] [Prismatoids](https://github.com/bevy-procedural/modelling/blob/main/examples/zoo.rs): Prism, Antiprism, Cuboid, Pyramid, Frustum, ...
    -   [x] [Platonic solids](https://github.com/bevy-procedural/modelling/blob/main/examples/zoo.rs): Tetrahedron, Cube, Octahedron, Dodecahedron, Icosahedron
    -   [x] [Round things](https://github.com/bevy-procedural/modelling/blob/main/examples/zoo.rs): Cylinder, Cone, UV Sphere, Icosphere, Geodesic Polyhedra
    -   [ ] 4d stuff: Tesseract, Hypersphere, Hypersimplex, ...
    -   [ ] Cube Sphere
    -   [ ] Torus, Clifford Torus

-   Operations

    -   [x] [Extrude](https://github.com/bevy-procedural/modelling/blob/main/examples/box.rs)
    -   [x] Linear Loft (Triangle, Polygon), [ ] Loft along path
    -   [x] Transform (Translate, Rotate, Scale, [ ] Shear)
    -   [x] Frequency Subdivision (partial)
    -   [ ] Chamfer, Cantellate, Bevel, Truncate, Bitruncate, Omnitruncate
    -   [ ] Boolean Operations (Union, Intersection, Difference, Symmetric Difference)
    -   [ ] (Anisotropic) Simplification, LODs
    -   [ ] Dualize

-   Tools

    -   [x] Basic Network Science Tools (Laplacian, Adjacency, Degree, Spectrum)
    -   [x] Mesh Isomorphism (partial)
    -   [ ] 2d Polygons: Area, Efficient Valid Diagonals, Convexity, ...
    -   [ ] Geodesic Pathfinding
    -   [ ] Raycasting
    -   [ ] Topology Analysis
    -   [ ] Spatial Data Structures

<!--
-   Debug Visualizations

    -   [x] Indices
    -   [ ] Normals
    -   [ ] Tangents
    -   [ ] Topology
-->

-   Extensions

    -   [x] [bevy](https://github.com/bevy-procedural/modelling/tree/main/playground/bevy)
    -   [x] [wgpu](https://github.com/bevy-procedural/modelling/tree/main/playground/wgpu)
    -   [x] [mini-renderer](https://github.com/bevy-procedural/modelling/tree/main/playground/svg) to quickly render to svg, e.g., the animations embedded in this readme.
    -   [x] nalgebra (when not using bevy)
    -   [x] SVG import/ [ ] export
    -   [ ] STL import/export
    -   [ ] OBJ import/export

## Customization via Traits

The availability of algorithms and operations for different mesh data structures is represented by traits. Some examples:

-   The `Transformable` trait indicates that you can apply affine transformation to a mesh and implements methods such as `translate` and `rotate`.
-   The `EuclideanMeshType` indicates that the mesh has vertex positions and lives in an Euclidean space and associates the mesh data type with a `Scalar` and `Vector` type etc.
-   The `MakePrismatoid` trait implements methods such as `insert_pyramid` or `insert_cube`.
-   The `MeshTypeHalfEdge` trait indicates that the mesh is based on a half-edge data structure (or can be treated as if) and makes sure that the mesh uses edge implementations that implement half-edge related methods like `twin_id`. It also enables the use of many algorithms that are currently only implemented for half-edge meshes.

For a full list of traits see the [documentation](https://docs.rs/procedural_modelling).

When using this trait-based library, you need to define your own type of `Mesh` and implement all traits you need. For most traits all methods have reasonable default implementations. This allows you to quickly implement meshes backed by a custom data structure or using custom vertex, face, edge, or mesh payloads. If you don't need anything special, you can use one of our default implementations such as `Bevy3dMesh` or the generic backend-agnostic `MeshNd<d>`. See `backends/bevy/mesh3d.rs` or `backends/nalgebra/mesh_nd.rs` for the exemplary default implementations.

## Features

The following cargo features are available:

-   `bevy` -- Compiles with support for bevy.
-   `wgpu` -- Compiles with support for wgpu.
-   `bevy_example` -- Compiles with the dependencies necessary for the examples.
-   `netsci` -- Enable network science tools.
-   `svg` -- Enable SVG import. Adds [usvg](https://github.com/linebender/resvg) as a dependency.
-   `fonts` -- Enable font rendering. Adds [ab_glyph](https://github.com/alexheretic/ab-glyph) as a dependency.
-   `meshopt` -- Enable mesh optimization. Adds [meshopt](https://github.com/gwihlidal/meshopt-rs) as a dependency.
-   `nalgebra` -- Enable [nalgebra](https://nalgebra.org/) as a backend. This is usually required for anything but bevy.

For development only:

-   `sweep_debug` -- Collect debug information during the sweepline triangulation and enable visualizations in the bevy backend.
-   `sweep_debug_print` -- Print debug information for the sweepline triangulation.

## Triangulation algorithms

The package supports different triangulation algorithms. The robustness and rendering speed of the produced triangulations varies significantly. These performance differences usually only matter for meshes with extremely large flat surfaces. In the table below, we compare the performance of the different algorithms for a circle with 100, 1000, and 10000 vertices. The "ZigZag" mesh has 1000 reps. 10000 vertices and is designed to demonstrate the worst-case for the Sweep algorithm.

-   **Fan** Extremely fast, but only works for convex polygons and results are often numerically unstable. Runs in $\mathcal{O}(n)$ time.
-   **EarClipping** Simple but slow and numerically unstable textbook-algorithm for reference. Runs in $\mathcal{O}(n^2)$ time. When the input provokes near-degenerate triangles, e.g., a very large circle, the algorithm switches to recovery mode, taking up to $\mathcal{O}(n^3)$ time.
-   **Sweep** Very fast sweep-line algorithm that might produce triangulations with unnecessarily long edges. Works for arbitrary polygons. Runs in $\mathcal{O}(n \log n)$ time. See [CMSC 754](https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf).
-   **Delaunay** Fast and numerically stable triangulation. Currently uses [Spade](https://github.com/Stoeoef/spade). Runs in $\mathcal{O}(n \log n)$ time.
-   **EdgeFlip** Same output as Delaunay, but without external dependencies and using a very slow edge-flipping algorithm. Runs in $\mathcal{O}(n^3)$ time.
    EdgeFlip,
-   **SweepDynamic** Applies the MinWeight algorithm to each monotone sub-polygon.
-   **MinWeight** Calculates the minimum weight triangulation, i.e., minimizes the overall edge length of the triangulation. Very slow, but produces the theoretically fastest rendering triangulations for large flat surfaces. Runs in $\mathcal{O}(n^3)$ time using dynamic programming. (Since we don't have inner points this is not NP-hard)
-   **Heuristic** Heuristic algorithm that tries to find a compromise between the speed of `Sweep` and the quality of `MinWeight`.
-   **Auto** (default) Automatically choose the "best" algorithm based on the input. The edge-weight will be the same as Delaunay or better. Uses specialized fast implementations for small polygons to quickly generate min-weight triangulations. Falls back to Delaunay for larger polygons.

<img src="assets/fps_vs_render.svg" alt="FPS Boxplot" width="800"/>

-   Time for the triangulation on a Intel i7-12700K (single threaded). Run the benchmarks using `cargo bench --features benchmarks --profile release`.
-   FPS when rendering 100 large, transparent instances with the bevy 0.14.2 pbr shader on a Nvidia GeForce RTX 4060 Ti in Full HD. See `cargo run -p fps_bench --profile release` and `julia --project=./playground/fps_bench/FPSBench ./playground/fps_bench/FPSBench/src/main.jl`. For the non-Delaunay algorithms, the rendering time deteriorates for the larger circles since the edge length is not minimized causing significant overdraw.

## Supported Bevy Versions

The following table shows the compatibility of `procedural_modelling` (when using the `bevy` feature) with certain versions of Bevy:

| bevy | bevy_procedural_meshes |
| ---- | ---------------------- |
| 0.16 | 0.4.\*, main           |
| 0.15 | 0.3.\*                 |
| 0.14 | 0.2.\*                 |
| 0.13 | 0.1.\*                 |

## License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed, allowing you the flexibility to choose between:

-   The MIT License (LICENSE-MIT or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
-   The Apache License, Version 2.0 (LICENSE-APACHE or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)).

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
