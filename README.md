# Procedural Modelling

<!-- 
[![Documentation](https://docs.rs/procedural_modelling/badge.svg)](https://docs.rs/procedural_modelling)
[![crates.io](https://img.shields.io/crates/v/procedural_modelling)](https://crates.io/crates/procedural_modelling) 
[![Downloads](https://img.shields.io/crates/d/procedural_modelling)](https://crates.io/crates/procedural_modelling)
[![License](https://img.shields.io/crates/l/procedural_modelling)](https://bevyengine.org/learn/quick-start/plugin-development/#licensing)
-->
[![Build Status](https://github.com/bevy-procedural/modelling/actions/workflows/rust.yml/badge.svg)](https://github.com/bevy-procedural/modelling/actions)
[![GitHub Repo stars](https://img.shields.io/github/stars/bevy-procedural/modelling)](https://github.com/bevy-procedural/modelling)
[![Lines of Code](https://tokei.rs/b1/github/bevy-procedural/modelling)](https://github.com/bevy-procedural/modelling)

A Framework-Agnostic Procedural Modelling Library.

Uses a datastructure based on half-edge meshes to represent (open) manifold meshes with optional non-manifold vertices. Our goal is to implement operations like Boolean Operations, Subdivisions, Curved Surfaces, and Stitching. The library aims to support the tesselation of 2d surfaces in a modular way that can be used without any of the 3d features.

Currently there are quite a few crates that implement boolean operations and tesselation to achieve some other goal. We want to provide a common implementation to satisfy these very similar requirements and improve code-reuse among these projects so they can focus on their original goal.


## WARNING

This crate is still in a _very_ early stage of development. Expect frequent API modifications, bugs, and missing features. Feel free to contribute by opening issues, pull requests or sharing your ideas in [Github Discussion](https://github.com/bevy-procedural/modelling/discussions).


## Usage

<img src="assets/demo.png" alt="drawing" width="300"/>

Install using `cargo add procedural_modelling`.

```rs
let mut mesh = MeshVec3::regular_star(1.0, 0.8, 30);
mesh.transform(
    &Transform::from_translation(Vec3::new(0.0, -0.99, 0.0))
               .with_rotation(Quat::from_rotation_z(PI)),
);
let trans = Transform::from_rotation(Quat::from_rotation_y(0.3))
                      .with_translation(Vec3::new(0.4, 0.3, 0.0));
let mut f = mesh.extrude_ex(mesh.edge_between(1, 0).unwrap().id(), trans, true, true);
for _ in 0..5 {
    f = mesh.extrude_face_ex(f, trans, true, true);
}
mesh.to_bevy(RenderAssetUsages::default())
```

## Examples 

<!--
Try the live examples!
 * [2d](https://bevy-procedural.org/examples/modelling/2d)
 * [3d](https://bevy-procedural.org/examples/modelling/3d)
-->

Or run the [examples](https://github.com/bevy-procedural/modelling/tree/main/examples) on your computer like, e.g., `cargo run --features="bevy" --profile fast-dev --example 2d`.

For package development, we recommend using the `editor`-subcrate. This example has a little [egui](https://github.com/jakobhellermann/bevy-inspector-egui/)-editor. Run it using `cargo watch -w editor/src -w src -x "run -p editor --profile fast-dev"`. The `fast-dev` profile will enable optimizations for the dependencies, but not for the package itself. This will slow down the first build _significantly_, but incremental builds are slightly faster and bevy's performance (bevy is used as the renderer in the examples) improves a lot.


## Feature Progress

- [ ] Attributes
  - [x] Positions
  - [x] Normals
  - [ ] Smooth Surface Groups
  - [ ] Tangents
  - [ ] UV Coordinates
  - [ ] Custom Attributes
- [ ] Triangulation
  - [(x)] Montone Sweep-Line
  - [x] Constrained Delaunay (using Delaunator)
- [ ] Primitives
  - [x] Polygon, Star
  - [x] Cuboid
  - [x] Cylinder, Cone, Frustum, Tetrahedron, Octahedron
  - [ ] Dodecahedron, Icosahedron
  - [ ] UV Sphere
  - [ ] Cube Sphere
  - [ ] Icosphere
  - [ ] Torus
- [ ] Builder Primitives
  - [x] Lines
  - [ ] Quadratic Bezier Curves
  - [ ] Cubic Bezier Curves
  - [ ] Curved Surfaces (Bezier Surfaces / Parametric Surfaces / NURBS / Spline Networks...?)
- [ ] Operations   
  - [x] Extrude 
  - [ ] Loft
  - [ ] Inset
  - [ ] Plane Intersection
  - [ ] Union
  - [ ] Intersection
  - [ ] Difference
  - [ ] Symmetric Difference
  - [ ] (Anisotropic) Simplification / LODs
  - [ ] Stitching
  - [ ] Subdivision
  - [ ] Morphing
  - [ ] Voxelization
- [ ] Tools
  - [ ] Geodesic Pathfinding
  - [ ] Raycasting
  - [ ] Topology Analysis
  - [ ] Spatial Data Structures
- [ ] Debug Visualizations
  - [x] Indices
  - [ ] Normals
  - [ ] Tangents
  - [ ] Topology
- [ ] Backends
  - [x] Bevy
  - [ ] wgpu



## Features

The following features are available:

* `meshopt` -- Use [Meshopt](https://github.com/gwihlidal/meshopt-rs) to optimize the performance of generated meshes. 
* `bevy` -- Compiles with support for bevy. Necessary for the examples and the editor.


## License

Except where noted (below and/or in individual files), all code in these repositories is dual-licensed, allowing you the flexibility to choose between:

 - The MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
 - The Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0).


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
