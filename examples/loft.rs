//! In this example, we demonstrate different uses of the loft and extrude methods.

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, mesh, prelude::*};
#[path = "common/bevy.rs"]
mod bevy_examples;

// TODO: demonstrate other configurations

fn lofted_polygon(sides: usize, m: usize, n: usize) -> BevyMesh3d {
    let circle_iter = |n: usize, r: f32, shift: f32| {
        let npi2: f32 = 2.0 / (n as f32) * std::f32::consts::PI;
        (0..n).map(move |i| {
            BevyVertexPayload3d::from_pos(Vec3::new(
                ((i as f32 + shift) * npi2).sin() * r,
                0.1,
                ((i as f32 + shift) * npi2).cos() * r,
            ))
        })
    };

    let mut mesh = BevyMesh3d::default();
    let e = mesh.insert_regular_polygon(1.0, sides);
    println!("{:?}", mesh);
    mesh.flip_yz()
        .translate(&Vec3::new(0.0, 0.1, 0.0))
        .loft_polygon(e, 3, 2, circle_iter(2*sides, 2.0, -2.0).take(7));
    mesh
}

fn generate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texts: ResMut<Text3dGizmos>,
) {
    let mesh = lofted_polygon(8, 2, 2);

    show_vertex_indices(&mut texts, &mesh);
    show_edges(&mut texts, &mesh, 0.1);
    show_faces(&mut texts, &mesh);

    commands.spawn((
        Mesh3d(meshes.add(mesh.to_bevy_ex(
            RenderAssetUsages::all(),
            // slowest triangulation, but looks nice for small examples
            TriangulationAlgorithm::MinWeight,
            true,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.85, 0.1),
            ..default()
        })),
    ));
}

fn main() {
    bevy_examples::setup_basic_bevy_app()
        .add_systems(Startup, generate_mesh)
        .run();
}
