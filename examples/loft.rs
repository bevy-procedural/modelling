//! In this example, we demonstrate different uses of the loft and extrude methods.

use bevy::{asset::RenderAssetUsages, prelude::*};
use itertools::Itertools;
use procedural_modelling::{extensions::bevy::*, math::Polygon, prelude::*};
#[path = "common/bevy.rs"]
mod bevy_examples;

// TODO: demonstrate other configurations

fn lofted_polygon(
    sides: usize,
    n: usize,
    m: usize,
    autoclose: bool,
    open: bool,
    vp: Option<Vec<BevyVertexPayload3d>>,
) -> BevyMesh3d {
    let mut mesh = BevyMesh3d::default();
    let e = mesh.insert_regular_polygon(1.0, sides).id();
    println!("{:?}", mesh);
    println!("{:?}", e);
    mesh.crochet(
        e,
        n,
        m,
        true,
        autoclose,
        open,
        vp.unwrap_or_else(|| {
            circle_iter::<3, BevyMeshType3d32>(
                (((n - 1) as f32) / ((m - 1) as f32) * sides as f32) as usize,
                2.0,
                0.0,
            )
            .collect_vec()
        }),
    )
    .unwrap();
    mesh.flip_yz().translate(&Vec3::new(0.0, 0.1, 0.0));

    for face in mesh.faces() {
        let poly = face.inner().as_polygon(&mesh);
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
        lofted_polygon(8, 3, 3, true, false, None),
        lofted_polygon(4, 3, 3, true, false, None),
        lofted_polygon(4, 3, 3, false, false, None),
        lofted_polygon(
            4,
            2,
            1,
            false,
            false,
            Some(
                (0..=3)
                    .into_iter()
                    .map(|i| {
                        BevyVertexPayload3d::from_pos(Vec3::new(
                            (2 * i) as f32 / 3.0 - 1.0,
                            2.0,
                            0.0,
                        ))
                    })
                    .collect_vec(),
            ),
        ),
    ]
    .iter()
    .cloned()
    .enumerate()
    {
        mesh.translate(&Vec3::new((i as f32 - 2.0) * 4.0, 0.0, 0.0));

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
