use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::render_asset::RenderAssetUsages,
    window::WindowResolution,
};
use bevy_inspector_egui::{
    inspector_options::ReflectInspectorOptions,
    quick::{FilterQueryInspectorPlugin, ResourceInspectorPlugin, WorldInspectorPlugin},
    InspectorOptions,
};
use bevy_panorbit_camera::*;
use procedural_modelling::representation::bevy::MeshVec3;
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

    n: usize,
    
    #[inspector(min = 0.0, max = 10.0)]
    r: f32,
    
    d1: Vec3,
    d2: Vec3,
    d3: Vec3,
}

impl Default for MeshSettings {
    fn default() -> Self {
        MeshSettings {
            tol: -4.0,
            n: 6,
            r: 1.0,
            d1: Vec3::new(0.4, -1.0, 0.0),
            d2: Vec3::new(-1.0, 0.3, -1.0),
            d3: Vec3::new(-1.0, -0.3, -1.0),
        }
    }
}

fn make_mesh(settings: &MeshSettings) -> MeshVec3 {
    let mut mesh = MeshVec3::regular_polygon(settings.r, settings.n); //cuboid(1.0, 1.0, 2.0);
    mesh.extrude(mesh.edge_between(1, 0).unwrap().id(), settings.d1, true);
    let fe = mesh.extrude_face(3, settings.d2, true);
    mesh.extrude_face(fe, settings.d3, true);
    println!("{}", mesh);
    mesh
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
    query: Query<(&Handle<Mesh>, &MeshSettings), Changed<MeshSettings>>,
    mut assets: ResMut<Assets<Mesh>>,
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

    for (handle, settings) in query.iter() {
        let mesh = make_mesh(settings);
        mesh.bevy_set(assets.get_mut(handle).unwrap());
    }
}

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = make_mesh(&MeshSettings::default());
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
        MeshSettings::default(),
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

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
