//! In this example, we will demonstrate a zoo of predefined shapes.

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, prelude::*};
mod bevy_examples;

fn generate_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let m = [
        BevyMesh3d::cone(1.0, 1.0, 16),
        BevyMesh3d::regular_antiprism(1.0, 0.5, 16),
        BevyMesh3d::regular_prism(1.0, 0.5, 16),
        {
            let mut mesh = BevyMesh3d::regular_star(1.0 / (f32::PHI * f32::PHI), 1.0, 10);
            mesh.flip_yz()
                .extrude_boundary(Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)));
            mesh
        },
        BevyMesh3d::cube(1.0),
        BevyMesh3d::regular_icosahedron(1.0),
        BevyMesh3d::regular_tetrahedron(2.0),
        BevyMesh3d::regular_octahedron(1.0),
        BevyMesh3d::uv_sphere(1.0, 16, 16),
        BevyMesh3d::geodesic_icosahedron(1.0, 8),
        BevyMesh3d::geodesic_tetrahedron(1.0, 16),
        BevyMesh3d::geodesic_octahedron(1.0, 16),
    ];
    for (i, mesh) in m.into_iter().enumerate() {
        // place it on the floor
        let min_y = mesh
            .vertices()
            .map(|v| v.pos().y)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

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
            Transform::from_translation(Vec3::new(
                ((i % 4) as f32 - 2.0) * 2.5,
                -min_y,
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
