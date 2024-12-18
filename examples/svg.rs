//! In this example, we will load and render a 2d-duck from svg.

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{
    extensions::{bevy::*, svg::*},
    prelude::*,
};
#[path = "common/bevy.rs"]
mod bevy_examples;

fn generate_path(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // the input can be a bare svg path or a full-fledged svg file with css and anything
    let svg = "<path d='M913.88 548.4c-66.14 35.43-141.83-7.68-141.83-7.68-112.76-53.91-246.31-55.82-246.31-55.82-34.09-2.34-25.47-17.51-20.69-25.88 0.73-1.27 1.74-2.36 2.59-3.56a187.06 187.06 0 0 0 34.17-108.08c0-103.78-84.13-187.92-187.92-187.92C251 159.47 167.37 242.24 166 344.87c-1 3.81-42.28 9.32-73-5.06-40-18.71-25.08 73.65 42.35 95.45l-2.31-0.1c-28.06-1.52-30.8 7.68-30.8 7.68s-16.14 29.75 83.13 38.37c31.39 2.72 35.71 8.11 42 16.64 11.92 16.14 3.57 39.25-12.15 59-44.53 55.77-71.84 180.68 49.78 270.85 103.12 76.47 377.65 79.95 497.37-15.13 108-85.76 156.72-170.47 185.79-241.14 3.9-9.54 31.84-58.43-34.28-23.03z' fill='#DFEDFF'/>";

    let mut mesh = BackendSVG::<BevyMeshType2d32>::from_svg(svg)
        .scale(&Vec2::splat(-0.004))
        .translate(&Vec2::new(2.0, 3.8))
        .to_3d(0.05);
    mesh.extrude(0, Transform::from_translation(Vec3::new(0.0, 0.0, -0.2)));

    commands.spawn((
        Mesh3d(meshes.add(mesh.to_bevy_ex(
            RenderAssetUsages::all(),
            // slowest triangulation, but looks nice for small examples
            TriangulationAlgorithm::MinWeight,
            true,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.85, 0.1),
            ..default()
        })),
    ));
}

fn main() {
    bevy_examples::setup_basic_bevy_app()
        .add_systems(Startup, generate_path)
        .run();
}

// TODO: Handle self-intersecting svg paths etc
/*let svg = "
<svg xmlns='http://www.w3.org/2000/svg' width='320' height='320'>
    <path d='M 10 315
            L 110 215
            A 36 60 0 0 1 150.71 170.29
            L 172.55 152.45
            A 30 50 -45 0 1 215.1 109.9
            L 315 10' stroke='black' fill='green' stroke-width='2' fill-opacity='0.5'/>
    <circle cx='150.71' cy='170.29' r='2' fill='red'/>
    <circle cx='110' cy='215' r='2' fill='red'/>
    <ellipse cx='144.931' cy='229.512' rx='36' ry='60' fill='transparent' stroke='blue'/>
    <ellipse cx='115.779' cy='155.778' rx='36' ry='60' fill='transparent' stroke='blue'/>
</svg>";*/
