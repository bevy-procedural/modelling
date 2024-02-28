use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        CascadeShadowConfigBuilder,
    },
    prelude::*,
    window::WindowResolution,
};
use bevy_inspector_egui::{
    inspector_options::ReflectInspectorOptions,
    quick::{FilterQueryInspectorPlugin, ResourceInspectorPlugin, WorldInspectorPlugin},
    InspectorOptions,
};
use bevy_panorbit_camera::*;
use procedural_modelling::*;
use std::{env, f32::consts::PI};

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct GlobalSettings {
    #[inspector(min = -20.0, max = 10.0)]
    tol: f32,

    px: f32,
    py: f32,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings {
            tol: -4.0,
            px: 0.0,
            py: 0.0,
        }
    }
}

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
struct MeshSettings {
    #[inspector(min = -20.0, max = 10.0)]
    tol: f32,
}

impl Default for MeshSettings {
    fn default() -> Self {
        MeshSettings { tol: -4.0 }
    }
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full"

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920.0, 1080.0),
                position: WindowPosition::Centered(MonitorSelection::Index(1)),
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WireframePlugin)
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .register_type::<GlobalSettings>()
        .insert_resource(GlobalSettings::default())
        .register_type::<MeshSettings>()
        .add_plugins((
            ResourceInspectorPlugin::<GlobalSettings>::default(),
            FilterQueryInspectorPlugin::<With<MeshSettings>>::default(),
            WorldInspectorPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            PanOrbitCameraPlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup_meshes)
        .add_systems(Update, update_meshes)
        .run();
}

fn update_meshes(
    _query: Query<&Handle<Mesh>>,
    mut _assets: ResMut<Assets<Mesh>>,
    mut settings: ResMut<GlobalSettings>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    if let Some(ray) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    {
        let distance = ray
            .intersect_plane(Vec3::ZERO, Plane3d::new(Vec3::Y))
            .unwrap_or(0.0);
        let world_position = ray.get_point(distance);
        if settings.px != world_position.x || settings.py != world_position.z {
            settings.px = world_position.x;
            settings.py = world_position.z;
        }
    }

    if !settings.is_changed() {
        return;
    }

    // TODO

    greet();

    // mesh.bevy_set(assets.get_mut(query.single().id()).unwrap());
}

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                alpha_mode: AlphaMode::Blend,
                double_sided: false,
                cull_mode: None,
                ..default()
            }),
            ..default()
        },
        MeshSettings::default(),
        Name::new("Generated Shape"),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });

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
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..Default::default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 0.1).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
