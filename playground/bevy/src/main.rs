//! cargo watch -w playground -w src -x "run -p playground_bevy --profile fast-dev"
//!
//! When developing tests on windows, we recommend:
//! $env:RUST_BACKTRACE=1;cargo watch -w src -x "test some_test"
//! cargo llvm-cov --html
//! cargo watch -w src -w examples -x "run --example loft --profile fast-dev --features bevy_example"

use bevy::{
    light::{CascadeShadowConfigBuilder, NotShadowCaster, ShadowFilteringMethod}, pbr::wireframe::{NoWireframe, WireframeConfig, WireframePlugin}, prelude::*, window::WindowResolution
};
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    inspector_options::ReflectInspectorOptions,
    quick::{ResourceInspectorPlugin, WorldInspectorPlugin},
    InspectorOptions,
};
use bevy_panorbit_camera::*;
use procedural_modelling::{extensions::bevy::*, prelude::*};
use std::{env, f32::consts::PI};

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct GlobalSettings {
    #[inspector(min = -20.0, max = 10.0)]
    tol: f32,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings { tol: 0.01 }
    }
}

#[derive(Component)]
struct CustomMesh;

#[derive(Component)]
struct CageMesh;

fn make_mesh(_settings: &GlobalSettings) -> BevyMesh3d {
    let mut mesh = BevyMesh3d::fake_uv_sphere(1.0, 5, 5);
    //regular_pyramid(1.0, 1.0, 4);
    //regular_icosahedron(1.0);
    //regular_polygon(1.0, 4);
    //icosphere(1.0, 2);
    // cube(1.0);

    // place it "on the floor"
    let min_y = mesh
        .vertices()
        .map(|v| v.pos().y)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    mesh.translate(&Vec3::new(0.0, -min_y, 0.0));

    mesh
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "0"); // or "full", "1", "0"

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920, 1080),
                position: WindowPosition::Centered(MonitorSelection::Index(1)),
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .register_type::<GlobalSettings>()
        .insert_resource(GlobalSettings::default())
        .add_plugins((
            PanOrbitCameraPlugin,
            Text3dGizmosPlugin,
            WireframePlugin::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::default(),
            ResourceInspectorPlugin::<GlobalSettings>::default(),
        ))
        .add_systems(Startup, setup_meshes)
        .add_systems(Update, update_meshes)
        .add_systems(Update, exit_on_esc)
        .run();
}

fn exit_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Messages<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}

fn update_meshes(
    query: Query<&Mesh3d, With<CustomMesh>>,
    cage_query: Query<&Mesh3d, With<CageMesh>>,
    mut assets: ResMut<Assets<Mesh>>,
    global_settings: ResMut<GlobalSettings>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    // mut texts: ResMut<Text3dGizmos>,
) {
    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera_q.single().unwrap();
    if let Some(ray) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
    {
        let distance = ray
            .intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
            .unwrap_or(0.0);
        let _world_position = ray.get_point(distance);
    }

    if !global_settings.is_changed() {
        return;
    }

    let mut mesh = make_mesh(&global_settings);
    mesh.generate_smooth_normals();
    for bevy_mesh in query.iter() {
        mesh.bevy_set_ex(
            assets.get_mut(bevy_mesh).unwrap(),
            TriangulationAlgorithm::MinWeight,
            true,
        );

        // TODO: reimplement meta has a custom payload
        //show_tesselation_meta(&mut texts, &mesh, &meta);
    }

    let mut cage = mesh.build_cage_mesh(0.06, 0.03);
    cage.generate_smooth_normals();
    for bevy_mesh in cage_query.iter() {
        cage.bevy_set_ex(
            assets.get_mut(bevy_mesh).unwrap(),
            TriangulationAlgorithm::MinWeight,
            false,
        );
    }
}

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texts: ResMut<Text3dGizmos>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0))))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.6, 0.4),
            double_sided: true,
            cull_mode: None,
            ..default()
        })),
        Name::new("Floor"),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0))))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.9, 0.9),
            //alpha_mode: AlphaMode::Blend,
            double_sided: true,
            cull_mode: None,
            ..default()
        })),
        Name::new("Generated Shape"),
        CustomMesh,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0))))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.9, 0.2),
            //alpha_mode: AlphaMode::Blend,
            double_sided: false,
            cull_mode: None,
            ..default()
        })),
        Name::new("Cage"),
        CageMesh,
        NoWireframe,
        NotShadowCaster,
    ));

    if true {
        let mesh = make_mesh(&GlobalSettings::default());
        show_vertex_indices(&mut texts, &mesh);
        show_edges(&mut texts, &mesh, 0.1);
        show_faces(&mut texts, &mesh);
    }

    commands.insert_resource(AmbientLight::default());
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.)
                .rotate_towards(Quat::from_rotation_x(PI / 4.), 0.5),
            ..default()
        },
        CascadeShadowConfigBuilder::default().build(),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 7.0, 5.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        PanOrbitCamera::default(),
        ShadowFilteringMethod::Gaussian,
    ));
}
