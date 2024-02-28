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

A framework-agnostic Procedural Modelling crate.

Uses [Render Dynamic Meshes](http://wscg.zcu.cz/WSCG2006/Papers_2006/Short/E17-full.pdf). Our goal is to implement operations like Boolean Operations, Subdivisions, Curved Surfaces, and Stitching. The library aims to support both 2D and 3D pathes and shapes.

## WARNING

This crate is still in a _very_ early stage of development. Expect frequent API modifications, bugs, and missing features. Feel free to contribute by opening issues, pull requests or sharing your ideas in [Github Discussion](https://github.com/bevy-procedural/modelling/discussions).


## Examples 

<!--
Try the live examples!
 * [2d](https://bevy-procedural.org/examples/modelling/2d)
 * [3d](https://bevy-procedural.org/examples/modelling/3d)
-->

Or run the [examples](https://github.com/bevy-procedural/modelling/tree/main/examples) on your computer like, e.g., `cargo run --features="bevy" --example 2d`.

For package development, we recommend using the `editor`-subcrate. This example has a little [egui](https://github.com/jakobhellermann/bevy-inspector-egui/)-editor. Run it using `cargo watch -w editor/src -w src -x "run -p editor --profile fast-dev"`. The `fast-dev` profile will enable optimizations for the dependencies, but not for the package itself. This will slow down the first build _significantly_, but incremental builds are slightly faster and bevy's performance (bevy is used as the renderer in the examples) improves a lot.


## Usage

Install using `cargo add procedural_modelling`.

```rs
// TODO
```


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
