//! In this example, we will demonstrate a zoo of predefined shapes.

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, prelude::*};
use std::f32::consts::PI;
#[path = "common/bevy.rs"]
mod bevy_examples;

fn generate_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let circle_iter = |r: f32, y: f32, shift: f32| {
        (0..10).map(move |i| {
            BevyVertexPayload3d::from_pos(Vec3::new(
                ((i as f32 + shift) / 5.0 * PI).sin() * r,
                y,
                ((i as f32 + shift) / 5.0 * PI).cos() * r,
            ))
        })
    };

    let m = [
        // row 1
        BevyMesh3d::cone(1.0, 1.0, 16),
        BevyMesh3d::regular_antiprism(1.0, 0.5, 16),
        BevyMesh3d::regular_prism(1.0, 0.5, 16),
        {
            let mut mesh = BevyMesh3d::regular_star(1.0 / (f32::PHI * f32::PHI), 1.0, 10);
            mesh.flip_yz()
                .extrude_boundary(Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)));
            mesh
        },
        // row 2
        BevyMesh3d::cube(1.0),
        BevyMesh3d::regular_icosahedron(1.0),
        BevyMesh3d::regular_tetrahedron(2.0),
        BevyMesh3d::regular_octahedron(1.0),
        //BevyMesh3d::dodecahedron(1.0) // TODO: crash?
        // row 3
        BevyMesh3d::fake_uv_sphere(1.0, 16, 16),
        BevyMesh3d::geodesic_icosahedron(1.0, 8),
        BevyMesh3d::geodesic_tetrahedron(1.0, 16),
        BevyMesh3d::geodesic_octahedron(1.0, 16),
        // row 4
        BevyMesh3d::prism(circle_iter(1.0, 0.0, 0.0), 0.5),
        BevyMesh3d::antiprism_iter(circle_iter(1.0, 0.0, 0.0), circle_iter(1.0, 0.5, 0.5)),
        //BevyMesh3d::antiprism(circle_iter(0.0, 0.0), 0.5), // TODO
        BevyMesh3d::pyramid(
            circle_iter(1.0, 0.0, 0.0),
            BevyVertexPayload3d::from_pos(Vec3::new(0.0, 1.0, 0.0)),
        ),
        BevyMesh3d::frustum(
            circle_iter(1.0, 0.0, 0.0),
            circle_iter(0.5, 0.8, 0.0),
            false,
        ),
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
                TriangulationAlgorithm::Auto,
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

/*
fn _make_hex_bridge(settings: &MeshSettings) -> BevyMesh3d {
    let mut mesh = BevyMesh3d::regular_polygon(settings.r, 6); //cuboid(1.0, 1.0, 2.0);
    mesh.extrude(mesh.edge_between(1, 0).unwrap().id(), settings.d1, true);
    let fe = mesh.extrude_face(1, Vec3::new(0.2, 0.1, 0.2), true);
    mesh.extrude_face(fe, Vec3::new(0.2, -0.1, 0.2), true);
    println!("{}", mesh);
    mesh
}*/

/*
fn _make_spiral() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::new();
    let mut edge = mesh.insert_regular_star(1.0, 0.8, 30);
    mesh.flip_yz().translate(&Vec3::new(0.0, -0.99, 0.0));
    let trans = Transform::from_rotation(Quat::from_rotation_y(0.3))
        .with_translation(Vec3::new(-0.2, 0.3, -0.3));
    edge = mesh.extrude_tri(edge, trans);
    for _ in 0..5 {
        edge = mesh.extrude_tri_face(mesh.edge(edge).face_id(), trans);
    }
    mesh
}*/
