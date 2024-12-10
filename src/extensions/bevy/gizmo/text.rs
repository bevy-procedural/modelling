//! Implements 3d text gizmos

use bevy::prelude::*;

use super::text3d;
pub use super::text3d::*;

// TODO: Convert this to a real gizmo where the text is requested every frame and other texts are deleted. Text widgets are re-used.
// TODO: move this to its own crate
// TODO: Hide text when it is behind objects

/// Support for 3d text gizmos
pub struct Text3dGizmosPlugin;

impl Plugin for Text3dGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(text3d::Text3dPlugin)
            .insert_resource(Text3dGizmos { texts: vec![] })
            .add_systems(Update, create_or_delete_text);
    }
}

/// A single 3d text gizmo
#[derive(Clone, Debug)]
pub struct Text3dGizmo {
    text: String,
    world_position: Vec3,
    should_remove: bool,
    entity: Option<Entity>,
    color: Color,
    font_size: f32,
}

impl Text3dGizmo {
    /// Creates a new 3d text gizmo
    pub fn new(text: String, pos: Vec3) -> Self {
        Self {
            text,
            world_position: pos,
            entity: None,
            should_remove: false,
            color: Color::WHITE,
            font_size: 20.0,
        }
    }

    /// Sets the text color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the font size
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }
}

/// A resource that holds all the 3d texts
#[derive(Resource, Clone, Debug)]
pub struct Text3dGizmos {
    texts: Vec<Text3dGizmo>,
}

impl Text3dGizmos {
    /// Add a new 3d text to the scene
    pub fn write(&mut self, text: Text3dGizmo) {
        self.texts.push(text);
    }

    fn remove_deleted(&mut self) {
        self.texts.retain(|text| !text.should_remove);
    }
}

fn create_or_delete_text(mut commands: Commands, mut texts: ResMut<Text3dGizmos>) {
    for text in texts.texts.iter_mut() {
        if let Some(entity) = text.entity {
            if text.should_remove {
                commands.entity(entity).despawn_recursive();
                text.entity = None;
            }
        } else {
            text.entity = Some(
                commands
                    .spawn((Node {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        overflow: Overflow::visible(),
                        max_width: Val::Px(0.0),
                        ..default()
                    },))
                    .with_children(|builder| {
                        builder.spawn((
                            Text::new(text.text.to_string()),
                            TextFont::from_font_size(text.font_size),
                            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                            TextColor(text.color),
                            text3d::Text3d::new(text.world_position, text.font_size),
                        ));
                    })
                    .id(),
            );
        }
    }
    texts.remove_deleted();
}
