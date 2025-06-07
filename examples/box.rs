//! In this example, we will construct a cuboid with side
//! lengths `x`, `y`, and `z` using different methods.
//! This is a good way to see the different ways this crate
//! provides to build a mesh and compare their trade-offs.

use bevy::{pbr::wireframe::NoWireframe, prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, mesh::MeshBuilder, prelude::*};
#[path = "common/bevy.rs"]
mod bevy_examples;

/// A tiny helper function to create a bevy-compatible vertex payload
fn vp(x: f32, y: f32, z: f32) -> BevyVertexPayload3d {
    BevyVertexPayload3d::from_pos(Vec3::new(x, y, z))
}

/// Creates a cuboid with a given `size`.
///
/// This method attempts the most intuitive approach:
/// Define all the vertices, then insert all the edges, then all the faces.
///
/// While being quite low-level, this method is not nice to read and
/// also not necessarily more efficient than the other methods.
fn cuboid_from_vertices(size: Vec3) -> BevyMesh3d {
    let (x, y, z) = (size * 0.5).tuple();
    let mut mesh = BevyMesh3d::new();

    // generate some default payloads (we don't store anything in faces or edges)
    let ep = Default::default();
    let fp = Default::default();

    // Generate the first vertices connected by a single edge
    let v0 = mesh.insert_vertex(vp(x, y, z)).unwrap().id();
    let (_e01, v1) = mesh.insert_vertex_v(v0, vp(-x, y, z), ep).unwrap();
    let (_e12, v2) = mesh.insert_vertex_v(v1, vp(-x, -y, z), ep).unwrap();
    let (_e23, v3) = mesh.insert_vertex_v(v2, vp(x, -y, z), ep).unwrap();

    // front face
    mesh.close_face_vvv(v2, v3, v0, ep, fp).unwrap();

    let (_e14, v4) = mesh.insert_vertex_v(v1, vp(-x, y, -z), ep).unwrap();
    let (_e45, v5) = mesh.insert_vertex_v(v4, vp(-x, -y, -z), ep).unwrap();

    // left face
    mesh.close_face_vvv(v4, v5, v2, ep, fp).unwrap();

    let (_e06, v6) = mesh.insert_vertex_v(v0, vp(x, y, -z), ep).unwrap();

    // top face
    mesh.close_face_vvv(v0, v6, v4, ep, fp).unwrap();

    let (_e37, v7) = mesh.insert_vertex_v(v3, vp(x, -y, -z), ep).unwrap();

    // right face
    mesh.close_face_vvv(v3, v7, v6, ep, fp).unwrap();

    // bottom face
    mesh.close_face_vvv(v2, v5, v7, ep, fp).unwrap();

    // back face
    mesh.insert_face(mesh.shared_edge(v6, v7).unwrap().id(), fp)
        .unwrap();

    mesh
}

/// Creates a cuboid with a given `size`.
///
/// Cursors provide a functional API for mesh traversal and modification.
///
/// They automatically keep track of the current edge/vertex/face and help with error handling.
/// That way, you can "walk" around the mesh as you build it and insert new vertices/edges/faces.
///
/// We are using an unnecessarily complex strategy here to demonstrate a larger palette of methods.
///
/// Since we are not using any custom payloads except for vertex positions, we can use the `Default` trait to initialize all other payloads.
fn cuboid_from_cursor(size: Vec3) -> BevyMesh3d {
    let (x, y, z) = (size * 0.5).tuple();
    let mut mesh = BevyMesh3d::new();

    mesh
        // First face (back)
        .insert_polygon([vp(x, y, -z), vp(-x, y, -z), vp(-x, -y, -z), vp(x, -y, -z)].into_iter())
        // Second face (right)
        .with_id(|c, e0| {
            c.insert_vertex(vp(x, y, z), Default::default())
                .insert_vertex(vp(x, -y, z), Default::default())
                .close_face(e0, Default::default(), Default::default())
                // Panic on failure
                .unwrap()
        })
        // Third face (front)
        .twin()
        .next()
        .with_id(|c, e1| {
            c.insert_vertex(vp(-x, y, z), Default::default())
                .insert_vertex(vp(-x, -y, z), Default::default())
                .close_face(e1, Default::default(), Default::default())
        })
        // Fourth face (bottom)
        .twin()
        .stay(|c| {
            c.next_n(2)
                .with_id(|c, e2| c.next_n(2).connect(e2, Default::default()))
                .insert_face(Default::default())
                .edge()
        })
        // A large non-planar six-sided temporary face
        .insert_face(Default::default())
        .edge()
        // Fifth+sixth face (bottom + left)
        .prev_n(2)
        .with_id(|c, e3| {
            c.next_n(2)
                // Split the large face into two planar quads
                .split_face(e3, Default::default(), Default::default())
        })
        // Assert the whole construction was successful
        .ensure();

    mesh
}

/*
/// Creates a cuboid with a given `size`.
///
/// Manually keeping track of all half-edges is the most low-level way
/// to build a mesh. It's cumbersome, but fast and gives you full control
/// over the connectivity of the mesh.
fn cuboid_from_edges(_size: Vec3) -> BevyMesh3d {
    todo!("cuboid_from_edges")
}*/

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

    mesh
        // create the bottom face by inserting a polygon
        .insert_polygon(vs.iter().map(|(x, y)| vp(*x, *y, -p.z())))
        // go to the next edge
        .next()
        // The parameters 2 and 2 specify that the loft should create polygons
        // with 2 vertices at the top and 2 at the bottom, i.e., quadrilaterals.
        .loft(2, 2, vs.iter().map(|(x, y)| vp(*x, *y, p.z())))
        // close the top face
        .insert_face(Default::default())
        .ensure();
    mesh
}

/*
/// Creates a cuboid with a given `size`.
///
/// We can also use the fact that faces aren't necessarily planar.
/// Therefore, we construct the cube net with a single face and then
/// subdivide that face into quads to create the cuboid.
fn cuboid_from_net(size: Vec3) -> BevyMesh3d {
    todo!()
}*/

// TODO: Add a completely static example where you manually insert all vertices, then edges, then faces

/// Creates a cuboid with a given `size`.
///
/// This crate provides a collection of primitives that can be used
/// to build more complex shapes. For example, noticing that a cuboid
/// is just a prism with a rectangular base, we can use the `prism`
/// method to create a cuboid.
fn cuboid_from_prism(size: Vec3) -> BevyMesh3d {
    let p = size * 0.5;
    BevyMesh3d::prism(
        [
            vp(-p.x(), -p.y(), -p.z()),
            vp(p.x(), -p.y(), -p.z()),
            vp(p.x(), p.y(), -p.z()),
            vp(-p.x(), p.y(), -p.z()),
        ],
        p.z() * 2.0,
    )
}

/*
/// Creates a cuboid with a given `size`.
///
/// The `PathBuilder` is a more flexible way to create a 2d shape.
/// It's a bit overpowered here, but can be especially usefull when working with bezier curves.
fn cuboid_from_builder(size: Vec3) -> BevyMesh3d {
    let p = size * 0.5;
    let mut mesh = BevyMesh3d::new();

    PathBuilder::<BevyMeshType3d32, _>::start(&mut mesh, Vec3::new(-p.x(), -p.y(), -p.z()))
        .line(Vec3::new(p.x(), -p.y(), -p.z()))
        .line(Vec3::new(p.x(), p.y(), -p.z()))
        .line(Vec3::new(-p.x(), p.y(), -p.z()))
        .close(Default::default());

    mesh.extrude_boundary(Transform::from_translation(Vec3::new(
        0.0,
        0.0,
        p.z() * 2.0,
    )));
    mesh
}*/

/// Creates a cuboid with a given `size`.
///
/// To keep this crate flexible, most methods are highly generic.
/// Especially the vertex payloads are in no way restricted to
/// vertices with positions in 3D space. This method demonstrates
/// which traits are necessary to create a cuboid.
///
/// - The `EP` and `FP` are the edge and face payloads, respectively. Since
///   we are not planning to initialize special payloads here, we will restrict
///   the mesh to payloads that can be safely initialized with `Default::default()`.
/// - `MakePrismatoid` is a trait that provides the `prism` method.
///
/// The `MeshType` trait collects the many types needed by a `Mesh`.
/// When handling meshes, most types can be derived from the `MeshType`.
/// The `HalfEdgeMeshType` and `MeshType3D` further restrict the mesh to
/// half-edge meshes and meshes with 3D position data and enable additional methods.
/// Some restrictions they imply include:
///
/// - The `Vec` is the default vector type used for vertices. Since we are creating
///   a cuboid in 3D space, we will use the `Vector3D` type.
/// - The `VP` is the vertex payload. We will use the `HasPosition` trait to
///   indicate that the payload must have a position vector compatible with the
///   `T::Vec` type. Additionally, we will use the `Transformable` trait to indicate
///   that the payload can be transformed in 3D space.
/// - The `S` is the scalar type used in the vector. This is usually implemented
///   as a `f32` or `f64`, though, other types like fixed-point numbers are also possible.
fn cuboid_from_prism_generic<T: MeshTypeHalfEdge + MeshType3D>(size: T::Vec) -> T::Mesh
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::Mesh: MakePrismatoid<T>,
{
    // TODO: With the new types, the description isn't that helpful anymore.

    let p = size * T::S::HALF;
    let make = |x, y, z| T::VP::from_pos(T::Vec::from_xyz(x, y, z));
    T::Mesh::prism(
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

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texts: ResMut<Text3dGizmos>,
) {
    let size = Vec3::new(1.0, 1.0, 2.0);
    let generated_meshes = [
        cuboid_from_cursor(size),
        cuboid_from_vertices(size),
        //cuboid_from_edges(size),
        cuboid_from_loft(size),
        //cuboid_from_builder(size),
        cuboid_from_prism(size),
        cuboid_from_prism_generic::<BevyMeshType3d32>(size),
        cuboid_from_cuboid(size),
    ];

    // When printing a mesh, the output will be a list of vertices and edges.
    // This method will also do some sanity checks to ensure that the mesh is
    // correctly constructed and will warn you, e.g., if there are non-planar
    // faces or if the mesh is non-manifold.
    println!("{:?}", generated_meshes[0]);

    // When adding a mesh to bevy, we need to convert it to a bevy mesh first.
    // This will triangulate the mesh and convert it to a format that bevy can
    // render. The `to_bevy_ex` method allows you to specify the render asset
    // usages, the triangulation algorithm, and whether the mesh should have
    // flat normals and tangents.
    for (i, mesh) in generated_meshes.iter().enumerate() {
        // We can verify that the generated meshes are isomorphic to a normal cuboid
        // (even though the might have a different vertex order)
        assert_eq!(
            BevyMesh3d::cuboid(size).is_isomorphic_by_pos::<_, 3, _, BevyMeshType3d32>(&mesh, 1e-5),
            MeshEquivalenceDifference::Equivalent
        );

        // Since we want to visualize the vertex indices, we translate the mesh here
        // instead of translating the bevy mesh.
        let mesh = mesh.translated(&Vec3::new(
            (i as f32 - generated_meshes.len() as f32 / 2.0) * 1.5,
            0.5,
            0.0,
        ));

        show_vertex_indices(&mut texts, &mesh);
        //show_edges(&mut texts, &mesh, 0.1);
        //show_faces(&mut texts, &mesh);

        commands.spawn((
            Mesh3d(meshes.add(mesh.to_bevy_ex(
                RenderAssetUsages::all(),
                TriangulationAlgorithm::Auto,
                true,
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            })),
            Name::new(format!("Box {}", i)),
        ));

        let mut cage = mesh.build_cage_mesh(0.1, 0.05);
        cage.generate_smooth_normals();
        commands.spawn((
            Mesh3d(meshes.add(cage.to_bevy_ex(
                RenderAssetUsages::all(),
                TriangulationAlgorithm::Auto,
                false,
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            })),
            NoWireframe,
        ));
    }
}

fn main() {
    bevy_examples::setup_basic_bevy_app()
        .add_systems(Startup, setup_meshes)
        .run();
}
