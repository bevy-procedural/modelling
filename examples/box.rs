//! Draws a cube that was constructed using HalfEdge structure

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*, render::render_asset::RenderAssetUsages,
};
use procedural_modelling::{representation::payload::bevy::BevyPayload, *};
use std::{env, f32::consts::PI};

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full"

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin)
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup_meshes)
        .run();
}

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = representation::Mesh::<u32, u32, u32, BevyPayload>::cuboid(1.0, 1.0, 2.0);
    println!("{}", mesh);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh.to_bevy(RenderAssetUsages::all())),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                //alpha_mode: AlphaMode::Blend,
                double_sided: false,
                cull_mode: None,
                ..default()
            }),
            ..default()
        },
        Name::new("Generated Shape"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::Y))),
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
