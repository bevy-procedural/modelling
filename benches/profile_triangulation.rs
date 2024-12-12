//! This sub-module is used to profile the performance of selected procedural modelling algorithms.
//!
//! When running this on WSL2, use something like `PERF=/usr/lib/linux-tools/5.15.0-126-generic/perf cargo flamegraph --bench profile_triangulation --profile profiling`
//! (see https://stackoverflow.com/a/65276025/6144727)

#[cfg(test)]
mod tests {
    use procedural_modelling::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn profile_triangulation() {
        let mesh = Mesh3d64::regular_polygon(1.0, 10000);
        /*let mesh: Mesh3d64 = Mesh2d64Curved::polygon(
            generate_zigzag::<Vec2<f64>>(100).map(|v| VertexPayloadPNU::from_pos(v)),
        ).to_nd(0.01);*/
        for _ in 0..1000 {
            let mut meta = Default::default();
            let (_vs, _is) = mesh.triangulate(TriangulationAlgorithm::Sweep, &mut meta);
        }
    }
}
