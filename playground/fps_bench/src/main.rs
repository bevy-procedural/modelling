//! Triangulates different polygons and measures the average FPS for each one.

use bevy::{
    prelude::*,
    asset::RenderAssetUsages,
    window::{PresentMode, VideoMode, WindowMode, WindowResolution},
};
use procedural_modelling::{extensions::bevy::*, prelude::*};
use std::io::Write;
use std::time::Duration;

#[derive(Resource, Clone, Debug)]
struct BenchmarkStats {
    name: String,
    frame_times: Vec<f64>,
    render_times: Vec<f64>,
    mesh: Handle<Mesh>,
    #[allow(dead_code)]
    num: usize,
}

#[derive(Resource)]
struct MeshList(Vec<BenchmarkStats>);

#[derive(Default, Resource)]
struct BenchmarkState {
    next_mesh_index: usize,
    accumulated_time: Duration,
    total_frames: u32,
    warm_up: bool,
    mesh_benchmarks: Vec<f32>,
}

const BENCHMARK_RENDER: Duration = Duration::from_secs(5);
const BENCHMARK_WARMUP: Duration = Duration::from_secs(5);
const BENCHMARK_DURATION: Duration = Duration::from_secs(15);
const TARGET_INSTANCES: usize = 100;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920, 1080).with_scale_factor_override(1.0),
                title: "Bevy Mesh Benchmark".to_string(),
                resizable: false,
                mode: WindowMode::Fullscreen(
                    MonitorSelection::Primary,
                    VideoModeSelection::Specific(VideoMode {
                        physical_size: UVec2::new(1920, 1080),
                        bit_depth: 24,
                        refresh_rate_millihertz: 60000,
                    }),
                ),
                // disable fps cap
                present_mode: PresentMode::Immediate,
                focused: true,
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
        generate_zigzag::<Vec2>(n).map(|v| BevyVertexPayload3d::from_pos(Vec3::new(v.x, 0.0, v.y))),
    )
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_list: ResMut<MeshList>,
) {
    for algo in [
        /*TriangulationAlgorithm::Delaunay,
        TriangulationAlgorithm::SweepDelaunay,*/
        TriangulationAlgorithm::Sweep,
        /*TriangulationAlgorithm::SweepDynamic,
        TriangulationAlgorithm::EarClipping,
        TriangulationAlgorithm::Fan,*/
        //TriangulationAlgorithm::Auto,
    ] {
        for (name, num_vertices, mesh) in [
            ("circle", 10, BevyMesh3d::regular_polygon(1.0, 10)),
            ("circle", 100, BevyMesh3d::regular_polygon(1.0, 100)),
            ("circle", 1000, BevyMesh3d::regular_polygon(1.0, 1000)),
            ("circle", 10000, BevyMesh3d::regular_polygon(1.0, 10000)),
            ("circle", 100000, BevyMesh3d::regular_polygon(1.0, 100000)),
            ("zigzag", 1000, zigzag(1000)),
            //("zigzag", 10000, zigzag(10000)),
        ] {
            if num_vertices > 1000 && (algo == TriangulationAlgorithm::SweepDynamic) {
                continue;
            }
            if num_vertices > 10000 && (algo == TriangulationAlgorithm::EarClipping) {
                continue;
            }
            mesh_list.0.push(BenchmarkStats {
                name: name.to_string() + format!("_{}_{:?}", num_vertices, algo).as_str(),
                mesh: meshes.add(mesh.to_bevy_ex(RenderAssetUsages::all(), algo, true)),
                num: num_vertices,
                frame_times: Vec::new(),
                render_times: Vec::new(),
            });

            println!("Benchmarkign Rendering time for {}", name);
            let start_time = std::time::Instant::now();
            // record up to 10000 samples
            for _ in 1..10000 {
                if start_time.elapsed() > BENCHMARK_RENDER {
                    break;
                }
                let start = std::time::Instant::now();
                let _ = mesh.triangulate_raw(algo);
                mesh_list
                    .0
                    .last_mut()
                    .unwrap()
                    .render_times
                    .push(start.elapsed().as_secs_f64());
            }

            /*println!("Rendering times for {} ", name);
            for render_time in mesh_list.0.last().unwrap().render_times.iter() {
                print!( "{:.10},", render_time);
            }*/
        }
    }

    //exit(0);

    commands.insert_resource(AmbientLight::default());
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
            ..default()
        },
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update_mesh(
    mut commands: Commands,
    mut state: ResMut<BenchmarkState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_list: Res<MeshList>,
    query: Query<Entity, With<Mesh3d>>,
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
            mesh_list.0[state.next_mesh_index - 1].name,
            avg_fps,
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
            println!("{}: {:.2} FPS", mesh_list.0[i].name, fps);
        }

        // write results to julia
        let mut file = std::fs::File::create("bench_results.jl").unwrap();
        writeln!(file, "data = [").unwrap();
        for bench in mesh_list.0.iter() {
            write!(file, "[\"{}\", [", bench.name).unwrap();
            for frame_time in bench.frame_times.iter() {
                write!(file, "{:.10},", frame_time).unwrap();
            }
            write!(file, "], [",).unwrap();
            for render_time in bench.render_times.iter() {
                write!(file, "{:.10},", render_time).unwrap();
            }
            writeln!(file, "]],",).unwrap();
        }
        writeln!(file, "]").unwrap();

        // Exit after all meshes are tested
        std::process::exit(0);
    }

    let stats = mesh_list.0[state.next_mesh_index].clone();
    let material = materials.add(Color::srgba(0.0, 0.0, 1.0, 0.01));

    // Spawn the next mesh
    for i in 0..TARGET_INSTANCES {
        commands.spawn((
            Mesh3d(stats.mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3::splat(0.01 * i as f32)),
        ));
    }

    println!("Starting benchmark for {}", stats.name);

    // Reset benchmark state
    state.accumulated_time = Duration::ZERO;
    state.total_frames = 0;
    state.next_mesh_index += 1;
    state.warm_up = true;
}

fn benchmark_fps(
    time: Res<Time>,
    mut state: ResMut<BenchmarkState>,
    mut mesh_list: ResMut<MeshList>,
) {
    // Accumulate time and frames
    state.accumulated_time += time.delta();
    if state.warm_up && state.accumulated_time > BENCHMARK_WARMUP {
        println!("Warm-up complete. Starting benchmark.");
        state.accumulated_time = Duration::ZERO;
        state.warm_up = false;
    }
    if !state.warm_up {
        state.total_frames += 1;
        if state.next_mesh_index >= 1 {
            mesh_list.0[state.next_mesh_index - 1]
                .frame_times
                .push(time.delta().as_secs_f64());
        }
    }
}
