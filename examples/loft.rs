//! In this example, we demonstrate different uses of the loft and extrude methods.

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, math::Polygon, mesh, prelude::*};
#[path = "common/bevy.rs"]
mod bevy_examples;

// TODO: demonstrate other configurations

fn lofted_polygon(sides: usize, n: usize, m: usize, autoclose: bool, open: bool) -> BevyMesh3d {
    let mut mesh = BevyMesh3d::default();
    let e = mesh.insert_regular_polygon(1.0, sides);
    println!("{:?}", mesh);
    println!("{:?}", e);
    mesh.crochet(
        e,
        n,
        m,
        true,
        autoclose,
        open,
        circle_iter::<3, BevyMeshType3d32>(
            (((n - 1) as f32) / ((m - 1) as f32) * sides as f32) as usize,
            2.0,
            0.0,
        )
        .take(16),
    );
    /*mesh.crochet(
        e,
        n,
        m,
        false,
        autoclose,
        open,
        circle_iter_back::<3, BevyMeshType3d32>(
            (((n - 1) as f32) / ((m - 1) as f32) * sides as f32) as usize,
            2.0,
            0.0,
        )
        .take(16),
    );*/
    mesh.flip_yz().translate(&Vec3::new(0.0, 0.1, 0.0));

    for face in mesh.faces() {
        let poly = face.as_polygon(&mesh);
        println!("{:?}", poly.area());
    }

    mesh
}

fn generate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texts: ResMut<Text3dGizmos>,
) {
    for (i, mut mesh) in [
        lofted_polygon(8, 3, 3, true, false),
        lofted_polygon(4, 3, 3, true, false),
        lofted_polygon(4, 3, 3, false, false),
    ]
    .iter()
    .cloned()
    .enumerate()
    {
        mesh.translate(&Vec3::new(((i as f32 - 1.0) - 0.5) * 4.0, 0.0, 0.0));

        show_vertex_indices(&mut texts, &mesh);
        show_edges(&mut texts, &mesh, 0.1);
        //show_faces(&mut texts, &mesh);

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
}

fn main() {
    bevy_examples::setup_basic_bevy_app()
        .add_systems(Startup, generate_mesh)
        .run();
}
