//! Triangulates different polygons and measures the average FPS for each one.

use bevy::{
    prelude::*,
    render::render_asset::RenderAssetUsages,
    window::{PresentMode, WindowResolution},
};
use procedural_modelling::representation::{
    bevy::BevyMesh3d, payload::bevy::BevyVertexPayload, primitives::generate_zigzag,
    tesselate::TriangulationAlgorithm,
};
use std::time::Duration;

#[derive(Resource)]
struct MeshList(Vec<(String, Handle<Mesh>, usize)>);

#[derive(Default, Resource)]
struct BenchmarkState {
    next_mesh_index: usize,
    accumulated_time: Duration,
    total_frames: u32,
    warm_up: bool,
    mesh_benchmarks: Vec<f32>,
}

const BENCHMARK_WARMUP: Duration = Duration::from_secs(5);
const BENCHMARK_DURATION: Duration = Duration::from_secs(15);
const TARGET_VERTICES: usize = 100000;
const TARGET_INSTANCES: usize = 100;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920.0, 1080.0),
                title: "Bevy Mesh Benchmark".to_string(),
                // disable fps cap
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(MeshList(Vec::new())) // You can add mesh handles here
        .insert_resource(BenchmarkState {
            next_mesh_index: 0,
            accumulated_time: BENCHMARK_DURATION,
            total_frames: 0,
            mesh_benchmarks: Vec::new(),
            warm_up: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, benchmark_fps)
        .add_systems(Update, update_mesh)
        .run();
}

fn zigzag(n: usize) -> BevyMesh3d {
    BevyMesh3d::polygon(
        generate_zigzag::<Vec2>(n).map(|v| BevyVertexPayload::from_pos(Vec3::new(v.x, v.y, 0.0))),
    )
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_list: ResMut<MeshList>,
) {
    for algo in [
        //TriangulationAlgorithm::Delaunay,
        //TriangulationAlgorithm::Sweep,
        TriangulationAlgorithm::EarClipping,
        //TriangulationAlgorithm::Fan,
    ] {
        for (name, num_vertices, mesh) in [
            ("circle10", 10, BevyMesh3d::regular_star(1.0, 1.0, 10)),
            ("circle100", 100, BevyMesh3d::regular_star(1.0, 1.0, 100)),
            ("circle1000", 1000, BevyMesh3d::regular_star(1.0, 1.0, 1000)),
            (
                "circle10000",
                10000,
                BevyMesh3d::regular_star(1.0, 1.0, 10000),
            ),
            ("zigzag1000", 1000, zigzag(1000)),
            ("zigzag10000", 10000, zigzag(10000)),
        ] {
            if num_vertices > 1000 && algo == TriangulationAlgorithm::EarClipping {
                continue;
            }
            mesh_list.0.push((
                name.to_string() + format!("_{:?}", algo).as_str(),
                meshes.add(mesh.to_bevy_ex(RenderAssetUsages::all(), algo, true)),
                num_vertices,
            ));
        }
    }

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_mesh(
    mut commands: Commands,
    mut state: ResMut<BenchmarkState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_list: Res<MeshList>,
    mut query: Query<Entity, With<Handle<Mesh>>>,
) {
    if state.accumulated_time < BENCHMARK_DURATION {
        return;
    }

    if state.next_mesh_index > 0 {
        // Calculate and record the average FPS for the current mesh
        let avg_fps = state.total_frames as f32 / BENCHMARK_DURATION.as_secs_f32();
        state.mesh_benchmarks.push(avg_fps);
        println!(
            "{}: {:.2} FPS",
            mesh_list.0[state.next_mesh_index - 1].0,
            avg_fps
        );

        // Clean up the previous mesh
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }

    // Move to the next mesh
    if state.next_mesh_index >= mesh_list.0.len() {
        // Print or log the results
        println!("Benchmark complete. Average FPS for each mesh:");
        for (i, fps) in state.mesh_benchmarks.iter().enumerate() {
            println!("{}: {:.2} FPS", mesh_list.0[i].0, fps);
        }

        // Exit after all meshes are tested
        std::process::exit(0);
    }

    let mesh = mesh_list.0[state.next_mesh_index].clone();
    let _mesh_num_vertices = mesh.2;
    let material = materials.add(Color::srgba(0.0, 0.0, 1.0, 0.01));

    // Spawn the next mesh
    for i in 0..TARGET_INSTANCES {
        //(TARGET_VERTICES / _mesh_num_vertices) {
        commands.spawn(PbrBundle {
            mesh: mesh.1.clone(),
            material: material.clone(),
            transform: Transform::from_scale(Vec3::splat(4.0))
                .with_translation(Vec3::splat(0.01 * i as f32)),
            ..default()
        });
    }

    println!("Starting benchmark for {}", mesh.0);

    // Reset benchmark state
    state.accumulated_time = Duration::ZERO;
    state.total_frames = 0;
    state.next_mesh_index += 1;
    state.warm_up = true;
}

fn benchmark_fps(time: Res<Time>, mut state: ResMut<BenchmarkState>) {
    // Accumulate time and frames
    state.accumulated_time += time.delta();
    if state.warm_up && state.accumulated_time > BENCHMARK_WARMUP {
        println!("Warm-up complete. Starting benchmark.");
        state.accumulated_time = Duration::ZERO;
        state.warm_up = false;
    }
    if !state.warm_up {
        state.total_frames += 1;
    }
}
