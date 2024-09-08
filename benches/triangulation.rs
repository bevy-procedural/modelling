//! A benchmark to test the speed of the triangulation

use bevy::{
    math::{Quat, Vec2, Vec3},
    transform::components::Transform,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use procedural_modelling::representation::{
    bevy::MeshVec3,
    tesselate::{sweep::generate_zigzag, GenerateNormals, TriangulationAlgorithm},
};
use std::f32::consts::PI;

fn make_spiral() -> MeshVec3 {
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
    mesh
}

fn bench_spirals(c: &mut Criterion) {
    let mut group = c.benchmark_group("Triangulation");

    for (name, mesh) in [
        //("Spiral", make_spiral()),
        //("Star", MeshVec3::regular_star(2.0, 0.9, 1000)),
        ("Circle100", MeshVec3::regular_star(1.0, 1.0, 100)),
        ("Circle1000", MeshVec3::regular_star(1.0, 1.0, 1000)),
        ("Circle10000", MeshVec3::regular_star(1.0, 1.0, 10000)),
        (
            "Zigzag10001",
            MeshVec3::polygon(
                &generate_zigzag::<Vec2>(10001)
                    .iter()
                    .map(|v| Vec3::new(v.x, v.y, 0.0))
                    .collect::<Vec<_>>(),
            ),
        ),
    ] {
        group.bench_with_input(
            BenchmarkId::new("Auto", name),
            &mesh,
            |b, para: &MeshVec3| {
                b.iter(|| {
                    let mut meta = Default::default();
                    para.tesselate(
                        TriangulationAlgorithm::Auto,
                        GenerateNormals::None,
                        &mut meta,
                    );
                })
            },
        );
        /*group.bench_with_input(
            BenchmarkId::new("Ears", name),
            &mesh,
            |b, para: &MeshVec3| {
                b.iter(|| {
                    let mut meta = Default::default();
                    para.tesselate(
                        TriangulationAlgorithm::EarClipping,
                        GenerateNormals::None,
                        &mut meta,
                    );
                })
            },
        );*/
        group.bench_with_input(
            BenchmarkId::new("Delaunay", name),
            &mesh,
            |b, para: &MeshVec3| {
                b.iter(|| {
                    let mut meta = Default::default();
                    para.tesselate(
                        TriangulationAlgorithm::Delaunay,
                        GenerateNormals::None,
                        &mut meta,
                    );
                })
            },
        );
    }

    group.finish();
}

/// Create the benchmark group
criterion_group!(benches, bench_spirals);

/// Run the benchmark
criterion_main!(benches);
