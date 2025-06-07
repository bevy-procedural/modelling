//! `cargo watch -w playground -w src -x "run -p playground_svg"`

use procedural_modelling::{
    extensions::{
        mini_renderer::{render2svg_wiggle, Render2SVGSettings},
        nalgebra::*,
    },
    prelude::*,
};
use std::io::Write;

// TODO: Optionally show triangulation with thinner lines and normals per tiny triangle!
// TODO: Calculate drop shadow. Also: self shadowing!
// TODO: Fix z-sort during rotation by duplicating some faces. How to make them invisible? (Have we already fixed this by collapsing backfaces to single points?)
// TODO: Smooth / Gouraud shading in svg? https://www.alecjacobson.com/weblog/3398.html or https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDiffuseLighting To avoid 
// TODO: Handle intersecting faces
// TODO: Render vertices/edges without faces too!

fn _make_spiral() -> Mesh3d64 {
    let mut mesh = Mesh3d64::new();
    let trans = NdHomography::from_rotation(NdRotate::from_axis_angle(Vec3::z_axis(), 0.3))
        .with_translation(Vec3::new(-0.2, -0.3, 0.3));
    let mut edge = mesh.insert_regular_star(1.0, 0.8, 30).extrude_tri(&trans);
    for _ in 0..5 {
        edge = edge.face().extrude_tri(&trans);
    }
    mesh.flip_yz().translate(&Vec3::new(0.0, -0.99, 0.0));
    mesh
}

fn main() {
    let mut mesh2d = Mesh2d64Curved::new();
    procedural_modelling::mesh::Font::new(include_bytes!("../../../assets/Cochineal-Roman.otf"), 2.0)
        .layout_text::<2, MeshType2d64PNUCurved>("Pg", &mut mesh2d);

    let mut mesh3d = mesh2d.to_3d(0.05);
    //mesh3d.extrude_boundary(&TransformTrait::from_translation(Vec3::new(0.0, 0.0, -0.2)));
    mesh3d.translate(&Vec3::new(-1.0, -0.5, 0.0));
        

    let mut mesh = 
    //_make_spiral();
//    Mesh3d64::regular_icosahedron(1.0);
    //Mesh3d64::fake_uv_sphere(1.0, 32, 32);
    //Mesh3d64::cube(1.0);
    mesh3d;
    //Mesh3d64::regular_star(1.0 / (f64::PHI * f64::PHI), 1.0, 10);
    //Mesh3d64::regular_antiprism(1.0, 1.0, 10);
    //Mesh3d64::regular_polygon(1.0, 10);

    assert_eq!(mesh.check(), Ok(()));

    mesh.simplify_all_islands().unwrap();

    // TODO: doesn't work with holes!
    //mesh.extrude_boundary(&NdHomography::from_translation(Vec3::new(0.0, 0.0, -0.2)));

    assert_eq!(mesh.check(), Ok(()));

    mesh = mesh.triangulated(TriangulationAlgorithm::Delaunay);

    assert_eq!(mesh.check(), Ok(()));

    mesh.transform(&NdHomography::from_scale(Vec3::splat(0.7)));

    // mesh.transform(&NdHomography::from_scale(Vec3::from_xyz(1.0, -1.0, 1.0)));
    //mesh.transform(&NdHomography::from_rotation_arc(Vec3::from_xyz(0.0, 0.0, 1.0), Vec3::from_xyz(-1.0, 0.0, 0.0)));

    let mut s = Render2SVGSettings::default();
    s.vertex_id_size = 0.0;
    //s.face_id_size = 0.0;
    s.vertex_size = 0.0;
    //s.steps = 0;
    //s.stroke_width = 0.0;
    s.eye = Vec3::new(0.0, 0.0, 2.0);
    
    let res = render2svg_wiggle::<f64, MeshType3d64PNU>(&mesh,0.5,&s);

    let mut file = std::fs::File::create("output.svg").unwrap();
    file.write_all(res.as_bytes()).unwrap();
    file.sync_all().unwrap();
}
