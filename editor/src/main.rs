use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    window::WindowResolution,
};
use bevy_inspector_egui::{
    inspector_options::ReflectInspectorOptions,
    quick::{FilterQueryInspectorPlugin, ResourceInspectorPlugin, WorldInspectorPlugin},
    InspectorOptions,
};
use bevy_panorbit_camera::*;
use procedural_modelling::{
    gizmo::{
        self,
        bevy::{text::Text3dGizmos, *},
    },
    representation::{
        bevy::BevyMesh3d,
        payload::{bevy::BevyVertexPayload, HasPosition},
        primitives::{generate_zigzag, random_star},
        tesselate::{TesselationMeta, TriangulationAlgorithm},
    },
};
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

    #[inspector(min = 0.0, max = 10.0)]
    r2: f32,

    d1: Vec3,
    rot: f32,
    segs: usize,
}

impl Default for MeshSettings {
    fn default() -> Self {
        MeshSettings {
            tol: -4.0,
            n: 30,
            r: 1.0,
            r2: 0.8,
            d1: Vec3::new(0.4, 0.3, 0.0),
            rot: 0.3,
            segs: 5,
        }
    }
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
}

fn _make_2d_merge_join() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        [
            // Front edge
            (1.0, -1.0),
            (0.5, 0.9),
            (0.0, -0.8),
            (-0.6, -1.0),
            (-0.8, -0.8),
            (-1.0, -1.0),
            // Back edge
            (-1.0, 1.0),
            (0.0, 0.8),
            (0.6, 1.0),
            (0.8, 0.8),
            (1.0, 1.0),
        ]
        .iter()
        .map(|(x, z)| BevyVertexPayload::from_pos(Vec3::new(*x, 0.0, *z))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_hell_8() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        [
            (4.5899906, 0.0),
            (0.7912103, 0.7912103),
            (-4.2923173e-8, 0.9819677),
            (-1.2092295, 1.2092295),
            (-0.835097, -7.30065e-8),
        ]
        .iter()
        .map(|(x, z)| BevyVertexPayload::from_pos(Vec3::new(*x, 0.0, *z))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_star(_settings: &MeshSettings) -> BevyMesh3d {
    let mut mesh = BevyMesh3d::regular_star(2.0, 2.0f32.sqrt(), 10000);
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_random_star() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        random_star::<Vec2>(5, 6, 0.1, 1.0)
            .map(|v| BevyVertexPayload::from_pos(Vec3::new(v.x, 0.0, v.y))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_zigzag() -> BevyMesh3d {
    let n = 50;
    let mut mesh = BevyMesh3d::polygon(
        generate_zigzag::<Vec2>(n).map(|v| BevyVertexPayload::from_pos(Vec3::new(v.x, 0.0, -v.y))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_prism() -> BevyMesh3d {
    /*BevyMesh3d::prism(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        0.4,
    )*/
    /* BevyMesh3d::antiprism_iter(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                ((i as f32 + 0.5) / 5.0 * PI).sin(),
                0.3,
                ((i as f32 + 0.5) / 5.0 * PI).cos(),
            ))
        }),
    )*/

    /*BevyMesh3d::antiprism(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        0.4,
    )*/

    /*BevyMesh3d::pyramid(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        BevyVertexPayload::from_pos(Vec3::new(0.0, 1.0, 0.0)),
    )*/

    BevyMesh3d::frustum(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                ((i as f32) / 5.0 * PI).sin() * 0.5,
                0.8,
                ((i as f32) / 5.0 * PI).cos() * 0.5,
            ))
        }),
        false,
    )
}

fn make_mesh(_settings: &MeshSettings) -> BevyMesh3d {
    //_make_hell_8()
    //BevyMesh3d::regular_polygon(1.0, 10)
    //_make_spiral()
    //_make_2d_zigzag()
    //BevyMesh3d::octahedron(1.0)
    //BevyMesh3d::cone(1.0, 1.0, 16)
    //BevyMesh3d::regular_antiprism(1.0, 1.0, 8)
    //BevyMesh3d::uniform_antiprism(1.0, 16)
    //BevyMesh3d::regular_prism(1.0, 1.0, 8)
    //BevyMesh3d::uniform_prism(1.0, 8)
    //BevyMesh3d::regular_frustum(1.0, 0.5, 1.0, 8, false)
    //BevyMesh3d::regular_pyramid(1.0, 1.0, 8)
    //BevyMesh3d::tetrahedron(1.0)
    //BevyMesh3d::octahedron(1.0)


    //BevyMesh3d::dodecahedron(1.0)

    /*let mut mesh = BevyMesh3d::hex_plane(10, 8);
    mesh.flip_yz();
    mesh*/

    //BevyMesh3d::uv_sphere(3.0, 64, 64)
    BevyMesh3d::icosphere(3.0, 64)
    //BevyMesh3d::geodesic_tetrahedron(3.0, 128)
    //BevyMesh3d::geodesic_octahedron(3.0, 128)
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full", "1", "0"

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
            gizmo::bevy::text::Text3dGizmosPlugin,
        ))
        .add_systems(Startup, setup_meshes)
        .add_systems(Update, update_meshes)
        .add_systems(Update, exit_on_esc)
        .run();
}

fn exit_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

fn update_meshes(
    query: Query<(&Handle<Mesh>, &MeshSettings), Changed<MeshSettings>>,
    mut assets: ResMut<Assets<Mesh>>,
    mut settings: ResMut<GlobalSettings>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut texts: ResMut<Text3dGizmos>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    if let Some(ray) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    {
        let distance = ray
            .intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
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
        let mut mesh = make_mesh(settings);
        let mut meta = TesselationMeta::default();
        mesh.generate_smooth_normals();
        mesh.bevy_set_ex(
            assets.get_mut(handle).unwrap(),
            TriangulationAlgorithm::Delaunay,
            false,
            &mut meta,
        );

        show_tesselation_meta(&mut texts, &mesh, &meta);
    }
}

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texts: ResMut<Text3dGizmos>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0)))),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.9, 0.9, 0.9, 1.0),
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

    if false {
        let mesh = make_mesh(&MeshSettings::default());
        show_vertex_indices(&mut texts, &mesh);
        show_edges(&mut texts, &mesh, 0.1);
        show_faces(&mut texts, &mesh);
    }

    /*commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0)))),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0))
                .with_scale(Vec3::splat(10.0)),
            ..default()
        },
        Name::new("Floor"),
    ));*/

    commands.insert_resource(AmbientLight::default());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: false,
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
