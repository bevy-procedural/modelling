//! In this example, we will construct a path with bezier curves.

use bevy::{prelude::*, asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, prelude::*};
#[path = "common/bevy.rs"]
mod bevy_examples;

/*
fn _make_bezier(_settings: &GlobalSettings) -> BevyMesh3d {
    let mut mesh2d = BevyMesh2d::new();
    /*mesh2d.insert_regular_star(1.0, 1.0, 3);
    let e = mesh2d.edge_mut(0);
    e.set_curve_type(procedural_modelling::mesh::CurvedEdgeType::CubicBezier(
        Vec2::new(0.2, 0.0),
        Vec2::new(0.9, 0.5),
    ));*/

    procedural_modelling::mesh::Font::new(
        include_bytes!("../../../assets/Cochineal-Roman.otf"),
        2.0,
    )
    .layout_text::<2, BevyMeshType2d32>("sFä", &mut mesh2d);

    /*
    PathBuilder::<BevyMeshType2d32>::start(&mut mesh2d, Vec2::new(0.0, 0.0))
        .line(Vec2::new(1.0, 0.0))
        .line(Vec2::new(0.0, -2.0))
        .cubic_bezier(
            Vec2::new(0.0, 2.0),
            Vec2::new(-2.0, -2.0),
            Vec2::new(-1.0, 0.5),
        )
        .close(Default::default());
    */


    println!("{:?}", mesh2d);

    let mut mesh3d = mesh2d.to_3d(0.01);
    mesh3d.extrude_boundary(Transform::from_translation(Vec3::new(0.0, 0.0, -0.2)));
    mesh3d
}

*/

fn generate_path(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texts: ResMut<Text3dGizmos>,
) {
    let mut mesh2d = BevyMesh2d::new();
    // start at the origin
    PathBuilder::<BevyMeshType2d32, _>::start(&mut mesh2d, Vec2::new(0.0, 0.0))
        // add a straight line to the right
        .line(Vec2::new(1.0, 0.0))
        // add a s-shaped cubic bezier
        .cubic(
            Vec2::new(1.0, 1.0),
            Vec2::new(3.0, -1.0),
            Vec2::new(3.0, 0.0),
        )
        // add a straight line up
        .line(Vec2::new(0.0, 3.0))
        // close the shape with a straight line and create a face with the `default` face payload
        // (we're not using face payloads here so there isn't anything but the default)
        .close(Default::default());

    /*
    // TODO: fix the path builder for this

    // get the indices of two of the above vertices
    let v0 = mesh2d.closest_vertex(Vec2::new(3.0, 0.0)).unwrap().id();
    let v1 = mesh2d.closest_vertex(Vec2::new(0.0, 3.0)).unwrap().id();
    // instead of inserting a new vertex, initialize the path builder with the existing vertex
        PathBuilder::<BevyMeshType2d32, _>:: start_at(&mut mesh2d, v0)
        // now add a quadratic bezier curve to the other vertex
        // notice that this will make our a mesh a multi-graph since there is already an edge between the two vertices
        .quad_to(Vec2::new(3.0, 3.0), v1)
        // close this shape. Since the shape has already a closed boundary, `close` will not
        // insert an additional edge. That way we can build shapes without straight lines.
        // Note that the face has only 2 vertices. This is fine since the edges are curved. 
        .close(Default::default());
    */

    // TODO: demonstrate how to change edge types afterwards

    println!("{:?}", mesh2d);

    // flatten the curves and make it lie flat on the floor
    let mesh = mesh2d
        .to_3d(0.1)
        .flipped_yz()
        .translated(&Vec3::new(0.0, 0.01, 0.0));

    // to declutter the visualization, we'll show the indices of the mesh without flattening the curves
    let mesh_for_vis = mesh2d.to_3d(core::f32::INFINITY).flipped_yz();
    show_vertex_indices(&mut texts, &mesh_for_vis);
    show_edges(&mut texts, &mesh_for_vis, 0.1);
    show_faces(&mut texts, &mesh_for_vis);

    commands.spawn((
        Mesh3d(meshes.add(mesh.to_bevy_ex(
            RenderAssetUsages::all(),
            // slowest triangulation, but looks nice for small examples
            TriangulationAlgorithm::MinWeight,
            true,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        })),
    ));
}

fn main() {
    bevy_examples::setup_basic_bevy_app()
        .add_systems(Startup, generate_path)
        .run();
}
