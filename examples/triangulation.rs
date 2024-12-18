//! In this example, we will demonstrate different triangulation methods.

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, prelude::*};
mod bevy_examples;

fn _make_2d_merge_join() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        [
            // Front edge
            (1.0, -1.0),
            (0.5, 0.9),
            (0.0, -0.8),
            (-0.6, -1.0),
            (-0.8, -0.8),
            (-1.0, -1.0),
            // Back edge
            (-1.0, 1.0),
            (0.0, 0.8),
            (0.6, 1.0),
            (0.8, 0.8),
            (1.0, 1.0),
        ]
        .iter()
        .map(|(x, z)| BevyVertexPayload3d::from_pos(Vec3::new(*x, 0.0, *z))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_hell_8() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        [
            (4.5899906, 0.0),
            (0.7912103, 0.7912103),
            (-4.2923173e-8, 0.9819677),
            (-1.2092295, 1.2092295),
            (-0.835097, -7.30065e-8),
        ]
        .iter()
        .map(|(x, z)| BevyVertexPayload3d::from_pos(Vec3::new(*x, 0.0, *z))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_hell_10() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        [
            [0.8590163, 0.0],
            [0.52688754, 0.52688754],
            [-3.721839e-8, 0.8514575],
            [-0.41275758, 0.41275758],
            [-0.13604999, -1.1893867e-8],
            [-0.45389745, -0.4538976],
            [1.8924045e-9, -0.15869379],
            [0.28799793, -0.28799775],
        ]
        .iter()
        .map(|[x, z]| BevyVertexPayload3d::from_pos(Vec3::new(*x, 0.0, *z))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_star() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::regular_star(2.0, 2.0f32.sqrt(), 10000);
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_random_star() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        random_star::<Vec2>(5, 6, 0.1, 1.0)
            .map(|v| BevyVertexPayload3d::from_pos(Vec3::new(v.x, 0.0, v.y))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_zigzag() -> BevyMesh3d {
    let n = 50;
    let mut mesh = BevyMesh3d::polygon(
        generate_zigzag::<Vec2>(n).map(|v| BevyVertexPayload3d::from_pos(Vec3::new(v.x, 0.0, v.y))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn generate_path(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = _make_2d_zigzag();

    commands.spawn((
        Mesh3d(meshes.add(mesh.to_bevy_ex(
            RenderAssetUsages::all(),
            TriangulationAlgorithm::MinWeight,
            true,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        })),
    ));
}

fn main() {
    bevy_examples::setup_basic_bevy_app()
        .add_systems(Startup, generate_path)
        .run();
}
