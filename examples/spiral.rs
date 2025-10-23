//! This is a minimal example constructing a 3d spiral.

use bevy::{prelude::*, asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, prelude::*};
#[path = "common/bevy.rs"]
mod bevy_examples;

fn setup_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = BevyMesh3d::new();
    let trans = Transform::from_rotation(Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 0.3))
        .with_translation(Vec3::new(-0.2, -0.3, 0.3));
    let mut edge = mesh.insert_regular_star(1.0, 0.8, 30).extrude_tri(&trans);
    for _ in 0..5 {
        edge = edge.face().extrude_tri(&trans);
    }
    mesh.flip_yz();

    commands.spawn((
        Mesh3d(meshes.add(mesh.to_bevy_ex(
            RenderAssetUsages::all(),
            TriangulationAlgorithm::Auto,
            true,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
    ));
}

fn main() {
    bevy_examples::setup_basic_bevy_app()
        .add_systems(Startup, setup_mesh)
        .run();
}
