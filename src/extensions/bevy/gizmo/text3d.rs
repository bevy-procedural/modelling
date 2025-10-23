//! Implements 3d text.

use bevy::prelude::*;

/// Support for 3d text
pub struct Text3dPlugin;

impl Plugin for Text3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_text_positions);
    }
}

/// Component to hold text data and world position
#[derive(Component, Debug)]
pub struct Text3d {
    world_position: Vec3,
    font_size: f32,
}

impl Text3d {
    /// Create a new Text3d component
    pub fn new(world_position: Vec3, font_size: f32) -> Self {
        Self {
            world_position,
            font_size,
        }
    }
}

// System to update text entity positions based on their 3D world position
fn update_text_positions(
    mut text_3d_query: Query<(&mut Node, &Text3d)>,
    mut camera: Query<(&mut Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
) {
    for (mut node, text_3d) in text_3d_query.iter_mut() {
        if let Ok((camera, _, camera_global_transform)) = camera.single_mut() {
            let world_position = text_3d.world_position;
            let Ok(viewport_position) =
                camera.world_to_viewport(camera_global_transform, world_position)
            else {
                continue;
            };

            node.top = Val::Px(viewport_position.y - text_3d.font_size / 2.0);
            node.left = Val::Px(viewport_position.x);
        }
    }
}
