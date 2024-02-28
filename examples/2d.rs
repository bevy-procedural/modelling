//! This example demonstrates the most basic 2d usecase.

use bevy::{
    prelude::*,
    render::render_asset::RenderAssetUsages,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use procedural_modelling::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))).into(),
        material: materials.add(Color::PURPLE),
        ..default()
    });
}

fn update(
    query: Query<&Mesh2dHandle>,
    mut assets: ResMut<Assets<Mesh>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        // TODO
        greet();
    }
    // mesh.bevy_set(assets.get_mut(query.single().0.id()).unwrap());
}
