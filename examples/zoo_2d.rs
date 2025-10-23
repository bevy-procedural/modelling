//! Showcase different 2d shapes

use bevy::{asset::RenderAssetUsages, prelude::*};
use procedural_modelling::{extensions::bevy::*, mesh::MeshBuilder, prelude::*};
use std::f32::consts::PI;
#[path = "common/bevy.rs"]
mod bevy_examples;

fn generate_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let circle_iter = |r: f32, shift: f32| {
        (0..10).map(move |i| {
            BevyVertexPayload2d::from_pos(Vec2::new(
                ((i as f32 + shift) / 5.0 * PI).sin() * r,
                ((i as f32 + shift) / 5.0 * PI).cos() * r,
            ))
        })
    };

    let m = [
        // row 1
        BevyMesh2d::regular_polygon(1.0, 10),
        BevyMesh2d::regular_star(1.0 / (f32::PHI * f32::PHI), 1.0, 10),
        BevyMesh2d::polygon(circle_iter(1.0, 0.0)),
        BevyMesh2d::cubic_circle(1.0),
        {
            let mut mesh = BevyMesh2d::default();
            mesh.insert_loop_default(circle_iter(1.0, 0.0));
            mesh
        },
        // TODO: BevyMesh3d::hex_plane(10, 8);
        // TODO: zigzag2d
    ];
    for (i, mesh) in m.into_iter().enumerate() {
        let mesh = mesh
            .to_3d(0.01)
            .flipped_yz()
            .scaled(&Vec3::new(-1.0, 1.0, 1.0));

        commands.spawn((
            Mesh3d(meshes.add(mesh.to_bevy_ex(
                RenderAssetUsages::all(),
                // slowest triangulation, but looks nice for small examples
                TriangulationAlgorithm::MinWeight,
                true,
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.7, 0.6),
                ..default()
            })),
            Transform::from_translation(Vec3::new(
                ((i % 4) as f32 - 2.0) * 2.5,
                0.1,
                -((i / 4) as f32 - 0.5) * 2.5,
            )),
        ));
    }
}

fn main() {
    bevy_examples::setup_basic_bevy_app()
        .add_systems(Startup, generate_shapes)
        .run();
}
