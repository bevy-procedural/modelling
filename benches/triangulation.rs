//! A benchmark to test the speed of the triangulation

// TODO: Include the fps bench as custom measurement: https://github.com/bheisler/criterion.rs/blob/master/book/src/user_guide/custom_measurements.md
// TODO: Profiling https://github.com/bheisler/criterion.rs/blob/master/book/src/user_guide/profiling.md

//mod bevy_bench;

use bevy::{
    math::{Vec2, Vec3},
    utils::tracing::instrument::WithSubscriber,
};
use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
    Throughput,
};
use procedural_modelling::{extensions::bevy::*, prelude::*};
use std::time::Duration;

/*
fn _make_spiral() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::regular_star(1.0, 0.8, 30);
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
    mesh
}*/

fn zigzag(n: usize) -> BevyMesh3d {
    BevyMesh3d::polygon(
        generate_zigzag::<Vec2>(n).map(|v| BevyVertexPayload3d::from_pos(Vec3::new(v.x, v.y, 0.0))),
    )
}

fn bench_triangulation(c: &mut Criterion) {
    for (mesh_name, difficulty, is_convex, maker) in [
        (
            "Circle",
            1,
            true,
            Box::new(|n| BevyMesh3d::regular_star(1.0, 1.0, n)) as Box<dyn Fn(usize) -> BevyMesh3d>,
        ),
        /*(
            "Zigzag",
            1,
            false,
            Box::new(|n| zigzag(n)) as Box<dyn Fn(usize) -> BevyMesh3d>,
        ),*/
        //("Star", BevyMesh3d::regular_star(2.0, 0.9, 1000)),
        //("Spiral", _make_spiral()),
    ] {
        let mut group = c.benchmark_group(format!("Triangulation {}", mesh_name));
        // allow the lowest minimum number of samples since the large meshes are really slow.
        group
            .sample_size(10)
            .measurement_time(Duration::from_secs(5));

        let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
        group.plot_config(plot_config);

        for size in [10, 100, 1000, 10_000,/* 100_000, 1_000_000*/] {
            let mesh = maker(size);

            let mut create_bench =
                |algo_name: &str, max_size: usize, algo: TriangulationAlgorithm| {
                    if (size * difficulty) > max_size {
                        return;
                    }
                    group.throughput(Throughput::Elements(size as u64));
                    group.bench_with_input(
                        BenchmarkId::new(algo_name.to_string(), size),
                        &mesh,
                        |b, para: &BevyMesh3d| {
                            b.iter(|| {
                                let mut meta = Default::default();
                                para.triangulate(algo, &mut meta);
                            })
                        },
                    );
                };

            create_bench("Sweep", 1000_000, TriangulationAlgorithm::Sweep);
            //create_bench("SweepD", 2000, TriangulationAlgorithm::SweepDynamic);
            create_bench("Delaunay", 1000_000, TriangulationAlgorithm::Delaunay);
            /*create_bench("Ears", 10_000, TriangulationAlgorithm::EarClipping);
            create_bench("Ears", 10_000, TriangulationAlgorithm::MinWeight);
            if is_convex {
                create_bench("Fan", 1000_000, TriangulationAlgorithm::Fan);
            }*/
        }
        group.finish();
    }
}

criterion_group!(benches, bench_triangulation);
criterion_main!(benches);
