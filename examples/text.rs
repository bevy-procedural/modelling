//! In this example, we will create some 3d letters using a font file.

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, prelude::*};
mod bevy_examples;

fn generate_path(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh2d = BevyMesh2d::new();
    procedural_modelling::mesh::Font::new(include_bytes!("../assets/Cochineal-Roman.otf"), 2.0)
        .layout_text::<2, BevyMeshType2d32>("Hello World", &mut mesh2d);

    // Make some 3d letters
    let mut mesh = mesh2d.to_3d(0.05);
    mesh.extrude_boundary(Transform::from_translation(Vec3::new(0.0, 0.0, -0.2)));
    mesh.translate(&Vec3::new(-3.0, 0.0, 0.0));

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
