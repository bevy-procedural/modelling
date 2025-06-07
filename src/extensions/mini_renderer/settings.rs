use crate::{
    extensions::nalgebra::*,
    math::{Scalar, Vector},
};

/// Settings for the SVG mini renderer pipeline.
/// Pass this to [super::render2svg] to customize the rendering process.
pub struct Render2SVGSettings<S: Scalar> {
    /// Number of animation steps per animation direction
    /// (the animation will have 2 * steps frames since it goes back and forth)
    /// Set to 0 to disable animation
    pub steps: usize,

    /// Width of the edge stroke.
    /// Set to 0 to disable edge rendering.
    pub stroke_width: f32,

    /// Color of the edge stroke in SVG format (e.g. "black", "#FF0000", etc.)
    pub stroke_color: String,

    /// Color of the circles on top of vertices in SVG format (e.g. "black", "#FF0000", etc.)
    pub vertex_color: String,

    /// Size of the circles on top of vertices.
    /// Set to 0 to disable vertex rendering.
    pub vertex_size: f32,

    /// Color of the vertex IDs in SVG format (e.g. "black", "#FF0000", etc.)
    pub vertex_id_color: String,

    /// Size of the vertex IDs.
    /// Set to 0 to disable vertex ID rendering.
    pub vertex_id_size: f32,

    /// Color of the face IDs in SVG format (e.g. "black", "#FF0000", etc.)
    pub face_id_color: String,

    /// Size of the face IDs.
    pub face_id_size: f32,

    /// Offset of ids over the underlying geometry to avoid occlusion.
    pub id_offset: S,

    /// Direction of the directional light source in the scene.
    /// This renderer only supports a single directional light source for now.
    pub light_direction: Vec3<f32>,

    /// Diffuse color of the light source in RGB format (e.g. [0.8, 0.2, 0.2])
    pub diffuse_color: Vec3<f32>,

    /// Ambient color of the light source in RGB format (e.g. [0.2, 0.2, 0.2])
    pub ambient_color: Vec3<f32>,

    /// Duration of the animation in seconds (e.g. "6s", "1.5s", etc.)
    pub dur: String,

    /// Whether to only show the otherwise hidden faces in the SVG output.
    /// This is for debugging purposes only.
    ///
    /// If your curious about the artifacts that sometimes appear in the SVG output,
    /// you can set this to true and understand where they come from.
    pub show_only_hidden_faces: bool,

    /// Camera perspective setting: aspect ratio for the SVG output.
    pub aspect: S,
    /// Camera perspective setting: field of view in radians for the SVG output.
    pub fov_y: S,
    /// Camera perspective setting: near clipping plane for the SVG output.
    pub z_near: S,
    /// Camera perspective setting: far clipping plane for the SVG output.
    pub z_far: S,

    /// Camera perspective setting: eye position in the scene.
    pub eye: Vec3<S>,
    /// Camera perspective setting: target position in the scene.
    pub target: Vec3<S>,
    /// Camera perspective setting: up vector in the scene.
    pub up: Vec3<S>,
}

impl<S: Scalar> Default for Render2SVGSettings<S> {
    fn default() -> Self {
        Self {
            steps: 10,
            stroke_width: 0.01,
            stroke_color: "black".to_string(),
            vertex_color: "black".to_string(),
            vertex_size: 0.02,
            vertex_id_color: "red".to_string(),
            vertex_id_size: 0.05,
            face_id_color: "blue".to_string(),
            face_id_size: 0.05,
            id_offset: S::from(-0.001),
            light_direction: Vec3::<f32>::new(-0.5, 1.0, 1.0).normalize(),
            diffuse_color: Vec3::<f32>::splat(0.8),
            ambient_color: Vec3::<f32>::splat(0.2),
            dur: "6s".to_string(),
            show_only_hidden_faces: false,
            aspect: S::ONE,
            fov_y: S::PI / S::FOUR,
            z_near: S::ONE / S::TEN,
            z_far: Scalar::from_usize(100),
            eye: Vec3::<S>::new(S::ZERO, S::ONE, S::TWO),
            target: Vec3::<S>::splat(S::ZERO),
            up: Vec3::<S>::new(S::ZERO, S::ONE, S::ZERO),
        }
    }
}
