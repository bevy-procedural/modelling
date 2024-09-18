//! This example renders the spiral from the README.

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::render_asset::RenderAssetUsages,
};
use procedural_modelling::mesh::{bevy::BevyMesh3d, tesselate::TriangulationAlgorithm};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin)
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .add_systems(Startup, setup_camera_and_light)
        .add_systems(Startup, setup_meshes)
        .run();
}

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = BevyMesh3d::new();
    let mut edge = mesh.insert_regular_star(1.0, 0.8, 30);
    mesh.flip_yz().translate(&Vec3::new(0.0, -0.99, 0.0));
    let trans = Transform::from_rotation(Quat::from_rotation_y(0.3))
        .with_translation(Vec3::new(-0.2, 0.3, -0.3));
    edge = mesh.extrude_tri(edge, trans);
    for _ in 0..5 {
        edge = mesh.extrude_tri_face(mesh.edge(edge).face_id(), trans);
    }

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh.to_bevy_ex(
                RenderAssetUsages::all(),
                TriangulationAlgorithm::Delaunay,
                true,
            )),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.5, 0.5),
                ..default()
            }),
            ..default()
        },
        Name::new("Generated Shape"),
    ));
}

/// Add a floor, a camera, and some lights
fn setup_camera_and_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0)))),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0))
                .with_scale(Vec3::splat(10.0)),
            ..default()
        },
        Name::new("Floor"),
    ));
    commands.insert_resource(AmbientLight::default());
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(3.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}
