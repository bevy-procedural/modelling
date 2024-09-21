//! Benchmark for Bevy rendering performance.

/*
use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};
use std::{sync::Mutex, time::Duration};

const BENCHMARK_WARMUP: Duration = Duration::from_secs(5);
const TARGET_INSTANCES: usize = 100;

// Define a static mutable variable to hold the Duration
lazy_static::lazy_static! {
    static ref GLOBAL_DURATION: Mutex<Duration> = Mutex::new(Duration::default());
}

#[derive(Clone, Resource)]
struct BenchmarkState {
    accumulated_time: Duration,
    total_frames: u64,
    warm_up: bool,
    max_frames: u64,
    mesh: Mesh,
}

fn setup_mesh(
    mut commands: Commands,
    state: ResMut<BenchmarkState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Color::srgba(0.0, 0.0, 1.0, 0.01));

    // Spawn the next mesh
    for i in 0..TARGET_INSTANCES {
        //(TARGET_VERTICES / _mesh_num_vertices) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(state.mesh.clone()),
            material: material.clone(),
            transform: Transform::from_scale(Vec3::splat(4.0))
                .with_translation(Vec3::splat(0.01 * i as f32)),
            ..default()
        });
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

fn update_global_duration(delta: Duration) {
    let mut global_duration = GLOBAL_DURATION.lock().unwrap();
    *global_duration = delta;
}

fn get_global_duration() -> Duration {
    let global_duration = GLOBAL_DURATION.lock().unwrap();
    *global_duration
}

fn benchmark_fps(
    time: Res<Time>,
    mut state: ResMut<BenchmarkState>,
    mut exit_event: EventWriter<AppExit>,
) {
    // Accumulate time and frames
    state.accumulated_time += time.delta();
    if state.warm_up && state.accumulated_time > BENCHMARK_WARMUP {
        state.accumulated_time = Duration::ZERO;
        state.warm_up = false;
    }
    if !state.warm_up {
        state.total_frames += 1;
    }
    if state.total_frames > state.max_frames {
        exit_event.send(AppExit::Success);
    }
    update_global_duration(state.accumulated_time);
}

/// Run a benchmark to measure the time it takes to render `n` frames of a mesh.
pub fn run_fps_bench(n: u64, mesh: Mesh) -> f64 {
    // TODO: Currently, this fails to build the eventloop...
    
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920.0, 1080.0),
                        title: "Bevy Mesh Benchmark".to_string(),
                        // disable fps cap
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..default()
                    }),
                    close_when_requested: false,
                    ..default()
                })
                .disable::<LogPlugin>(),
        )
        .insert_resource(BenchmarkState {
            accumulated_time: Duration::default(),
            total_frames: 0,
            warm_up: false,
            max_frames: n,
            mesh,
        })
        .add_systems(Startup, setup_mesh)
        .add_systems(Update, benchmark_fps)
        .run();

    get_global_duration().as_secs_f64()
}
*/