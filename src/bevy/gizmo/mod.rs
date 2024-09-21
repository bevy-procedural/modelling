//! gizmo implementations for bevy.

pub mod text;
mod text3d;

use super::BevyMesh3d;
use crate::{
    math::IndexType,
    mesh::{Edge, Face, FaceBasics, MeshBasics, Vertex, VertexBasics},
    tesselate::TesselationMeta,
};
use bevy::prelude::*;
use text::{Text3dGizmo, Text3dGizmos};

#[cfg(feature = "sweep_debug")]
use crate::mesh::payload::VertexPayload;

/// Show the vertex indices of a mesh in blue.
pub fn show_vertex_indices(texts: &mut ResMut<Text3dGizmos>, mesh: &BevyMesh3d) {
    mesh.vertices().for_each(|v| {
        texts.write(
            Text3dGizmo::new(v.id().to_string(), v.pos().clone())
                .with_color(Color::srgb(0.0, 0.0, 1.0)),
        );
    });
}

/// Visualized the tesselation meta data of a mesh.
pub fn show_tesselation_meta<V: IndexType>(
    _texts: &mut ResMut<Text3dGizmos>,
    _mesh: &BevyMesh3d,
    _meta: &TesselationMeta<V>,
) {
    #[cfg(feature = "sweep_debug")]
    for (index, t) in _meta.sweep.vertex_type.iter() {
        _texts.write(
            Text3dGizmo::new(
                format!("{} {:?}", index, t),
                *_mesh
                    .vertices()
                    .nth(index.index())
                    .unwrap()
                    .payload()
                    .vertex(),
            )
            .with_color(Color::srgb(1.0, 0.0, 0.0)),
        );
    }
}

/// Show the edge indices of a mesh.
/// Boundary edges are red, edges with faces are green.
/// Use `offset` to slightly shift them towards the face center.
pub fn show_edges(texts: &mut ResMut<Text3dGizmos>, mesh: &BevyMesh3d, offset: f32) {
    mesh.edges().for_each(|e| {
        if let Some(f) = e.face(mesh) {
            let p0 = e.centroid(mesh).clone();
            let p1 = f.centroid(mesh).clone();
            let p01 = p0 + (p1 - p0).normalize() * offset;
            texts.write(
                Text3dGizmo::new(e.id().to_string(), p01).with_color(Color::srgb(1.0, 1.0, 0.0)),
            );
        } else {
            texts.write(
                Text3dGizmo::new(e.id().to_string(), e.centroid(mesh).clone())
                    .with_color(Color::srgb(1.0, 0.0, 0.0)),
            );
        }
    });
}

/// Show the face indices of a mesh in green.
pub fn show_faces(texts: &mut ResMut<Text3dGizmos>, mesh: &BevyMesh3d) {
    mesh.faces().for_each(|f| {
        texts.write(
            Text3dGizmo::new(f.id().to_string(), f.centroid(mesh).clone())
                .with_color(Color::srgb(0.0, 1.0, 0.0)),
        );
    });
}
