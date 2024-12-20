//! Some scaffolding code is needed to integrate the generated meshes with bevy.
#![cfg(feature = "bevy_example")]

use bevy::{
    pbr::{
        wireframe::{NoWireframe, WireframeConfig, WireframePlugin},
        CascadeShadowConfigBuilder,
    },
    prelude::*,
};
use procedural_modelling::extensions::bevy::Text3dGizmosPlugin;
use std::f32::consts::PI;

/// Add a floor, a camera, and some lights
fn setup_camera_and_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0))))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        })),
        Name::new("Floor"),
        NoWireframe,
    ));

    commands.insert_resource(AmbientLight::default());
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// this changed in bevy 0.15
pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

pub fn setup_basic_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins((WireframePlugin, Text3dGizmosPlugin))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .add_systems(Startup, setup_camera_and_light)
        .add_systems(Update, close_on_esc);
    app
}
