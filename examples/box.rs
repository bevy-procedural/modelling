//! In this example, we will construct a cuboid with side
//! lengths `x`, `y`, and `z` using different methods.
//! This is a good way to see the different ways this crate
//! provides to build a mesh and compare their trade-offs.

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::render_asset::RenderAssetUsages,
};
use procedural_modelling::{
    math::{Scalar, Vector, Vector3D},
    mesh::{
        bevy::BevyMesh3d,
        payload::{vertex_payload::BevyVertexPayload, HasPosition, Transformable},
        tesselate::TriangulationAlgorithm,
        DefaultEdgePayload, DefaultFacePayload, Mesh as TMesh, MeshType,
    },
};
use std::f32::consts::PI;

/// A tiny helper function to create a bevy-compatible vertex payload
fn vp(x: f32, y: f32, z: f32) -> BevyVertexPayload {
    BevyVertexPayload::from_pos(Vec3::new(x, y, z))
}

/// Creates a cuboid with a given `size`.
///
/// This method attempts the most intuitive approach:
/// Define all the vertices and manually close each face by connecting the vertices.
///
/// This, however, is not very efficient and also quite unpleasant to write.
fn cuboid_from_vertices(size: Vec3) -> BevyMesh3d {
    let (x, y, z) = (size * 0.5).tuple();
    let mut mesh = BevyMesh3d::new();
    let (v0, v1) = mesh.add_isolated_edge_default(vp(x, y, z), vp(-x, y, z));
    let v2 = mesh.add_vertex_via_vertex_default(v1, vp(-x, -y, z)).0;
    let v3 = mesh.add_vertex_via_vertex_default(v2, vp(x, -y, z)).0;
    mesh.close_face_vertices_default(v2, v3, v0, false);
    let v4 = mesh.add_vertex_via_vertex_default(v1, vp(-x, y, -z)).0;
    let v5 = mesh.add_vertex_via_vertex_default(v4, vp(-x, -y, -z)).0;
    mesh.close_face_vertices_default(v4, v5, v2, false);
    let v6 = mesh.add_vertex_via_vertex_default(v0, vp(x, y, -z)).0;
    let v7 = mesh.add_vertex_via_vertex_default(v3, vp(x, -y, -z)).0;
    mesh.close_face_vertices_default(v3, v7, v6, false);
    mesh.close_face_vertices_default(v2, v5, v7, false);
    mesh.close_face_vertices_default(v0, v6, v4, false);
    mesh.close_hole_default(mesh.shared_edge(v6, v7).unwrap().id());
    mesh
}

/// Creates a cuboid with a given `size`.
///
/// Manually keeping track of all half-edges is the most low-level way
/// to build a mesh. It's cumbersome, but fast and gives you full control
/// over the connectivity of the mesh.
fn cuboid_from_edges(size: Vec3) -> BevyMesh3d {
    todo!("cuboid_from_edges")
}

/// Creates a cuboid with a given `size`.
///
/// The loft function is a powerful tool to create rows of polygons
/// connecting paths or loops of vertices. When creating a cuboid,
/// we can use it to automatically create all the edges and faces.
fn cuboid_from_loft(size: Vec3) -> BevyMesh3d {
    let p = size * 0.5;
    let mut mesh = BevyMesh3d::new();
    let vs = [
        (-p.x(), -p.y()),
        (p.x(), -p.y()),
        (p.x(), p.y()),
        (-p.x(), p.y()),
    ];

    // create the bottom face by inserting a polygon
    let bottom_edge = mesh.insert_polygon(vs.iter().map(|(x, y)| vp(*x, *y, -p.z())));

    // The parameters 2 and 2 specify that the loft should create polygons
    // with 2 vertices at the top and 2 at the bottom, i.e., rectangles.
    let top_edge = mesh.loft_polygon(bottom_edge, 2, 2, vs.iter().map(|(x, y)| vp(*x, *y, p.z())));

    // close the top face
    mesh.close_hole_default(top_edge);
    mesh
}

/// Creates a cuboid with a given `size`.
///
/// This crate provides a collection of primitives that can be used
/// to build more complex shapes. For example, noticing that a cuboid
/// is just a prism with a rectangular base, we can use the `prism`
/// method to create a cuboid.
fn cuboid_from_prism(size: Vec3) -> BevyMesh3d {
    let p = size * 0.5;
    TMesh::prism(
        [
            vp(-p.x(), -p.y(), -p.z()),
            vp(p.x(), -p.y(), -p.z()),
            vp(p.x(), p.y(), -p.z()),
            vp(-p.x(), p.y(), -p.z()),
        ],
        p.z() * 2.0,
    )
}

/// Creates a cuboid with a given `size`.
///
/// To keep this crate flexible, most methods are highly generic.
/// Especially the vertex payloads are in no way restricted to
/// vertices with positions in 3D space. This method demonstrates
/// which traits are necessary to create a cuboid.
///
/// - The `MeshType` trait collects the many types needed by a `Mesh`.
///   When handling meshes, most types can be derived from the `MeshType`.
/// - The `EP` and `FP` are the edge and face payloads, respectively. Since
///   we are not planning to initialize special payloads here, we will restrict
///   the mesh to payloads that can be safely initialized with `Default::default()`.
/// - The `Vec` is the default vector type used for vertices. Since we are creating
///   a cuboid in 3D space, we will use the `Vector3D` type.
/// - The `VP` is the vertex payload. We will use the `HasPosition` trait to
///   indicate that the payload must have a position vector compatible with the
///   `T::Vec` type. Additionally, we will use the `Transformable` trait to indicate
///   that the payload can be transformed in 3D space.
/// - The `S` is the scalar type used in the vector. This is usually implemented
///   as a `f32` or `f64`, though, other types like fixed-point numbers are also possible.
fn cuboid_from_prism_generic<T: MeshType>(size: T::Vec) -> TMesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>
        + Transformable<Trans = T::Trans, Rot = T::Rot, Vec = T::Vec, S = T::S>,
{
    let p = size * T::S::HALF;
    let make = |x, y, z| T::VP::from_pos(T::Vec::from_xyz(x, y, z));
    TMesh::prism(
        [
            make(-p.x(), -p.y(), -p.z()),
            make(p.x(), -p.y(), -p.z()),
            make(p.x(), p.y(), -p.z()),
            make(-p.x(), p.y(), -p.z()),
        ],
        p.z() * T::S::TWO,
    )
}

/// Creates a cuboid with a given `size`.
///
/// Of course, for something as simple as a cuboid, we can just
/// use the `cuboid` method provided by the crate.
fn cuboid_from_cuboid(size: Vec3) -> BevyMesh3d {
    BevyMesh3d::cuboid(size)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin)
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .add_systems(Startup, setup_camera_and_light)
        .add_systems(Startup, setup_meshes)
        .run();
}

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = Vec3::new(1.0, 1.0, 2.0);
    let generated_meshes = [
        cuboid_from_vertices(size),
        //cuboid_from_edges(size),
        cuboid_from_loft(size),
        cuboid_from_prism(size),
        cuboid_from_prism_generic(size),
        cuboid_from_cuboid(size),
    ];

    // When printing a mesh, the output will be a list of vertices and edges.
    // This method will also do some sanity checks to ensure that the mesh is
    // correctly constructed and will warn you, e.g., if there are non-planar
    // faces or if the mesh is not manifold.
    println!("{}", generated_meshes[0]);

    // When adding a mesh to bevy, we need to convert it to a bevy mesh first.
    // This will triangulate the mesh and convert it to a format that bevy can
    // render. The `to_bevy_ex` method allows you to specify the render asset
    // usages, the triangulation algorithm, and whether the mesh should have
    // flat normals and tangents.
    for (i, mesh) in generated_meshes.iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh.to_bevy_ex(
                    RenderAssetUsages::all(),
                    TriangulationAlgorithm::Delaunay,
                    true,
                )),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.0, 0.0),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 - generated_meshes.len() as f32 / 2.0) * 1.5,
                    0.0,
                    0.0,
                )),
                ..default()
            },
            Name::new("Generated Shape"),
        ));
    }
}

/// Add a floor, a camera, and some lights
fn setup_camera_and_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0)))),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0))
                .with_scale(Vec3::splat(10.0)),
            ..default()
        },
        Name::new("Floor"),
    ));
    commands.insert_resource(AmbientLight::default());
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(3.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}
