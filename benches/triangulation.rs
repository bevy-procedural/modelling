//! A benchmark to test the speed of the triangulation

use bevy::{
    math::{Quat, Vec3},
    transform::components::Transform,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use procedural_modelling::representation::{
    bevy::MeshVec3,
    tesselate::{GenerateNormals, TriangulationAlgorithm},
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
    let mut group = c.benchmark_group("Tesselate Spiral");

    for (name, mesh) in [
        ("Spiral", make_spiral()),
        ("Star", MeshVec3::regular_star(2.0, 0.9, 1000)),
    ] {
        group.bench_with_input(
            BenchmarkId::new("Fast", name),
            &mesh,
            |b, para: &MeshVec3| {
                b.iter(|| {
                    para.tesselate(TriangulationAlgorithm::Fast, GenerateNormals::None);
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Ears", name),
            &mesh,
            |b, para: &MeshVec3| {
                b.iter(|| {
                    para.tesselate(TriangulationAlgorithm::EarClipping, GenerateNormals::None);
                })
            },
        );
        /*group.bench_with_input(
            BenchmarkId::new("Delaunay", name),
            &mesh,
            |b, para: &MeshVec3| {
                b.iter(|| {
                    para.tesselate(TriangulationAlgorithm::Delaunay, GenerateNormals::None);
                })
            },
        );*/
    }

    group.finish();
}

criterion_group!(benches, bench_spirals);
criterion_main!(benches);
