use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin,
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        CascadeShadowConfigBuilder, ShadowFilteringMethod,
    },
    prelude::*,
    window::WindowResolution,
};
use bevy_inspector_egui::{
    inspector_options::ReflectInspectorOptions,
    quick::{FilterQueryInspectorPlugin, ResourceInspectorPlugin, WorldInspectorPlugin},
    InspectorOptions,
};
use bevy_panorbit_camera::*;
use procedural_modelling::{
    extensions::{
        bevy::{text::*, *},
        svg::BackendSVG,
    },
    prelude::*,
};
use std::{env, f32::consts::PI};

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct GlobalSettings {
    #[inspector(min = -20.0, max = 10.0)]
    tol: f32,

    #[inspector(min = 0.0, max = 100.0)]
    prog: f32,

    px: f32,
    py: f32,

    curvature: f32,
    angle: f32,
    step_base: f32,
    step_tip: f32,
    smoothness: f32,
    overshoot: f32,
    overshoot_grow: f32,
    c1: f32,
    c2: f32,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings {
            tol: 0.01,
            px: 0.0,
            py: 0.0,
            prog: 50.0,

            curvature: -1.5,
            angle: 0.5,
            step_base: 0.1,
            step_tip: 0.03,

            smoothness: 0.2,
            overshoot: 1.2,
            overshoot_grow: 0.9,
            c1: 1.0,
            c2: 0.1,
        }
    }
}

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
struct MeshSettings {
    #[inspector(min = -20.0, max = 10.0)]
    tol: f32,

    n: usize,

    #[inspector(min = 0.0, max = 10.0)]
    r: f32,

    #[inspector(min = 0.0, max = 10.0)]
    r2: f32,

    d1: Vec3,
    rot: f32,
    segs: usize,
}

impl Default for MeshSettings {
    fn default() -> Self {
        MeshSettings {
            tol: -4.0,
            n: 30,
            r: 1.0,
            r2: 0.8,
            d1: Vec3::new(0.4, 0.3, 0.0),
            rot: 0.3,
            segs: 5,
        }
    }
}

/*
fn _make_hex_bridge(settings: &MeshSettings) -> BevyMesh3d {
    let mut mesh = BevyMesh3d::regular_polygon(settings.r, 6); //cuboid(1.0, 1.0, 2.0);
    mesh.extrude(mesh.edge_between(1, 0).unwrap().id(), settings.d1, true);
    let fe = mesh.extrude_face(1, Vec3::new(0.2, 0.1, 0.2), true);
    mesh.extrude_face(fe, Vec3::new(0.2, -0.1, 0.2), true);
    println!("{}", mesh);
    mesh
}*/

/*
fn _make_spiral() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::new();
    let mut edge = mesh.insert_regular_star(1.0, 0.8, 30);
    mesh.flip_yz().translate(&Vec3::new(0.0, -0.99, 0.0));
    let trans = Transform::from_rotation(Quat::from_rotation_y(0.3))
        .with_translation(Vec3::new(-0.2, 0.3, -0.3));
    edge = mesh.extrude_tri(edge, trans);
    for _ in 0..5 {
        edge = mesh.extrude_tri_face(mesh.edge(edge).face_id(), trans);
    }
    mesh
}*/

fn _make_2d_merge_join() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        [
            // Front edge
            (1.0, -1.0),
            (0.5, 0.9),
            (0.0, -0.8),
            (-0.6, -1.0),
            (-0.8, -0.8),
            (-1.0, -1.0),
            // Back edge
            (-1.0, 1.0),
            (0.0, 0.8),
            (0.6, 1.0),
            (0.8, 0.8),
            (1.0, 1.0),
        ]
        .iter()
        .map(|(x, z)| BevyVertexPayload3d::from_pos(Vec3::new(*x, 0.0, *z))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_hell_8() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        [
            (4.5899906, 0.0),
            (0.7912103, 0.7912103),
            (-4.2923173e-8, 0.9819677),
            (-1.2092295, 1.2092295),
            (-0.835097, -7.30065e-8),
        ]
        .iter()
        .map(|(x, z)| BevyVertexPayload3d::from_pos(Vec3::new(*x, 0.0, *z))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_hell_10() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        [
            [0.8590163, 0.0],
            [0.52688754, 0.52688754],
            [-3.721839e-8, 0.8514575],
            [-0.41275758, 0.41275758],
            [-0.13604999, -1.1893867e-8],
            [-0.45389745, -0.4538976],
            [1.8924045e-9, -0.15869379],
            [0.28799793, -0.28799775],
        ]
        .iter()
        .map(|[x, z]| BevyVertexPayload3d::from_pos(Vec3::new(*x, 0.0, *z))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_star(_settings: &MeshSettings) -> BevyMesh3d {
    let mut mesh = BevyMesh3d::regular_star(2.0, 2.0f32.sqrt(), 10000);
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_random_star() -> BevyMesh3d {
    let mut mesh = BevyMesh3d::polygon(
        random_star::<Vec2>(5, 6, 0.1, 1.0)
            .map(|v| BevyVertexPayload3d::from_pos(Vec3::new(v.x, 0.0, v.y))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

fn _make_2d_zigzag() -> BevyMesh3d {
    let n = 50;
    let mut mesh = BevyMesh3d::polygon(
        generate_zigzag::<Vec2>(n).map(|v| BevyVertexPayload3d::from_pos(Vec3::new(v.x, 0.0, v.y))),
    );
    mesh.transform(&Transform::from_translation(Vec3::new(0.0, -0.99, 0.0)));
    mesh
}

/*
fn _make_prism() -> BevyMesh3d {
    /*BevyMesh3d::prism(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        0.4,
    )*/
    /* BevyMesh3d::antiprism_iter(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                ((i as f32 + 0.5) / 5.0 * PI).sin(),
                0.3,
                ((i as f32 + 0.5) / 5.0 * PI).cos(),
            ))
        }),
    )*/

    /*BevyMesh3d::antiprism(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        0.4,
    )*/

    /*BevyMesh3d::pyramid(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        BevyVertexPayload::from_pos(Vec3::new(0.0, 1.0, 0.0)),
    )*/

    BevyMesh3d::frustum(
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                (i as f32 / 5.0 * PI).sin(),
                0.0,
                (i as f32 / 5.0 * PI).cos(),
            ))
        }),
        (0..10).map(|i| {
            BevyVertexPayload::from_pos(Vec3::new(
                ((i as f32) / 5.0 * PI).sin() * 0.5,
                0.8,
                ((i as f32) / 5.0 * PI).cos() * 0.5,
            ))
        }),
        false,
    )
}
*/

fn _make_blechnum_spicant(settings: &GlobalSettings) -> BevyMesh3d {
    // leaf strength
    let r1 = 0.1;
    // let r1b = r1 * 1.1;
    // strength at the tip
    let r2 = 0.03;

    // number of circle segments
    let n = 10;

    // number of spiral segments
    let m = 80;

    /*
    // log spiral radius
    let a = 4.0;

    // log spiral curvature
    let curvature = -0.2;
    let v_res = 0.2;

    let smoothness = 0.1;

    let log_spiral = |a: f32, phi: f32, k: f32| {
        let r = a * (k * phi).exp();
        Vec3::new(r * phi.cos(), r * phi.sin(), 0.0)
    };

    let archimedean_spiral = |a: f32, phi: f32| {
        let r = a * phi;
        Vec3::new(r * phi.cos(), r * phi.sin(), 0.0)
    };*/

    let circle = |r: f32, i: i32, n: i32| {
        let phi = PI * ((2 * i) as f32) / n as f32;
        Vec3::new(r * phi.cos(), 0.0, r * phi.sin())
    };
    /*let curve = |i: i32| {
        let p = settings.prog / 100.0;

        //let alpha = ((i - m / 2) as f32 * 0.04).tanh() / 2.0 + 0.5;
        let vr = v_res;
        let phi = (1.0 - p) * vr * i as f32 - PI * 0.1;

        let phi1 = vr * i as f32 - p * 12.0 + 8.0;
        let log_spiral_r = a * (curvature * phi1).exp();

        let vr2 = v_res; // / (1.0 + p * 10.0);
        let phi2 = (m as f32 * vr2) - vr2 * i as f32;
        let a2 = r1b / PI;
        let archimedean_spiral_r = a2 * phi2;

        assert!(log_spiral_r >= 0.0);
        assert!(archimedean_spiral_r >= 0.0);

        let alpha = ((i as f32) * smoothness).tanh() * 0.5 + 0.5;
        let r = log_spiral_r.lerp(archimedean_spiral_r, alpha);
        Vec3::new(r * phi.cos(), r * phi.sin(), 0.0)
    };*/
    let mut curve: Vec<Vec3> = (0..m).map(|_| Vec3::ZERO).collect();

    let smoothstep = |x: f32, center: f32, smoothness: f32, low: f32, high: f32| {
        (((x - center) * smoothness).tanh() * 0.5 + 0.5) * (high - low) + low
    };

    let archimedean_arch = |a: f32, phi: f32| {
        let pps = (1.0 + phi * phi).sqrt();
        a * 0.5 * (phi * pps + (phi + pps).ln())
    };

    // TODO: do a symbolic regression in julia to approximate this with a closed form
    let archimedean_phi = |a: f32, arch: f32| {
        // binary search to find phi such that archimedean_arch(a, phi) = arch
        let mut low = 0.0;
        let mut high = 1000.0;
        for _ in 0..10 {
            let mid = (low + high) / 2.0;
            let mid_val = archimedean_arch(a, mid);
            if mid_val < arch {
                low = mid;
            } else {
                high = mid;
            }
        }
        let phi = (low + high) / 2.0;
        return phi;
    };

    let archimedean_curvature =
        |a: f32, phi: f32| (phi * phi + 2.0) / (a * (1.0 + phi * phi).powf(1.5));

    let p = settings.prog / 100.0;
    // in the beginning, the curvature starts reversed to balance the rolled leaf
    let mut curvature = settings.curvature * p;
    // the leaf is leaning a bit outwards
    let mut angle = settings.angle;
    // the leaf is stronger at the base
    let step_base = settings.step_base;
    // the leaf is weaker at the tip
    let step_tip = settings.step_tip;

    // smoothness of the region between the archimedean and log spiral
    let smoothness = settings.smoothness;
    // factor of how much earlier to the final development of the leaf the archimedean spiral should be finished
    let overshoot = settings.overshoot;
    // factor of how much earlier to the final development of the leaf the cells should grow
    let overshoot_grow = settings.overshoot_grow;
    let overshoot_leaf = 1.1;
    let leaf_offset = 0.0;
    // How eager the spiral is to get into the archimedean spiral
    let c1 = settings.c1;
    // Curvature of the spiral in the log spiral region
    let c2 = settings.c2;

    let archimedeanness = 1.0; // 1.0 - p*p*0.5;

    // TODO: The archimedean part should have the opposite curvature of the log part, i.e., the spiral comes from the top!
    let arch_a = r2 / PI;
    for i in 1..m {
        let step = smoothstep(
            i as f32,
            (1.0 - p) * (m as f32 * overshoot_grow),
            smoothness,
            step_base, //* (1.0 - p),
            step_tip,
        );
        curvature += smoothstep(
            i as f32,
            (1.0 - p) * (m as f32 * overshoot),
            smoothness,
            c2 * p,
            c1,
        );
        curvature = curvature.clamp(-10.0, 50.0);

        let arch_curvature: f32 =
            archimedean_curvature(arch_a, archimedean_phi(arch_a, (m - i) as f32 * step));

        angle += curvature.lerp(arch_curvature.min(curvature), archimedeanness) * step;
        angle = angle % (2.0 * PI);
        let q = Quat::from_rotation_x(angle);
        curve[i] = curve[i - 1] + q.mul_vec3(Vec3::Y * step);
    }

    let mut mesh = BevyMesh3d::new();
    let base: Vec<Vec3> = (0..n).map(|i| circle(r1, -i, n)).collect();
    let first = curve[0];
    let mut edge = mesh.insert_polygon(
        base.iter()
            .map(|v| BevyVertexPayload3d::from_pos(*v + first)),
    );

    let normal = base.iter().cloned().normal().normalize();

    // TODO: this can be an API function: loft_along
    // TODO: There is sometimes a bug with the rotation or normals in the spiral. Slowly move the progress while observing the downwards facing part of the spiral.
    let mut prev = first;
    for i in 1..m {
        let cur = curve[i];
        let q = Quat::from_rotation_arc(normal, (cur - prev).normalize());
        prev = cur;

        // the radius is linearly interpolated between the base and the tip
        let scale = r1.lerp(r2, i as f32 / m as f32) / r1;

        edge = mesh.loft_tri_closed(
            edge,
            base.iter()
                .map(|v| BevyVertexPayload3d::from_pos(cur + q.mul_vec3(*v * scale))),
        );

        // TODO: make the smoothstep a more general concept. Also, investigate different overshoot and offset systems
        let leaf_progress = smoothstep(
            i as f32,
            (1.0 - p + leaf_offset) * (m as f32 * overshoot_leaf),
            smoothness,
            1.0,
            0.0,
        );

        let leaf_progress2 = smoothstep(
            i as f32,
            (1.0 - p - 0.6) * (m as f32 * overshoot_leaf * 3.0),
            smoothness * 0.5,
            1.0,
            0.0,
        );

        // add the leaves
        if i > m / 4 {
            let s_scale = 1.5;
            let leaf_len_small = 1.0;
            let leaf_len_big = 5.0;
            let mut sign = 1.0;
            if i % 2 == 0 {
                sign = -1.0;
            }
            let qq = q * Quat::from_rotation_y(-PI * sign * 0.5 * (1.0 - leaf_progress));

            let tip = sign * (leaf_len_small * r1 + r1 * leaf_len_big * leaf_progress);
            let base_y = step_base * s_scale * (leaf_progress2 * 0.8 + 0.2);
            // TODO: Draw the leaflets using a bezier curve
            let mut ps = [
                Vec3::new(tip * 0.5, -base_y * 0.5, 0.0),
                Vec3::new(tip * 0.95, base_y * 0.8, 0.0),
                Vec3::new(tip, base_y * 1.4, 0.0),
                Vec3::new(tip * 0.9, base_y * 1.4, 0.0),
                Vec3::new(tip * 0.8, base_y * 1.3, 0.0),
                Vec3::new(tip * 0.5, base_y * 0.8, 0.0),
                Vec3::new(0.0, base_y, 0.0),
                Vec3::new(0.0, -base_y, 0.0),
            ];
            if i % 2 == 0 {
                ps.reverse();
            }
            mesh.insert_polygon(ps.iter().map(|v| {
                BevyVertexPayload3d::from_pos(
                    cur + qq.mul_vec3(*v * scale) + sign * Vec3::X * scale * r1,
                )
            }));
        }
    }

    mesh.translate(&(Vec3::new(0.0, -2.0, 0.0) - first));
    mesh
}

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

    /*let circle_len = 4.0 / 3.0 * (2.0f32.sqrt() - 1.0);
    PathBuilder::<BevyMeshType2d32>::start(&mut mesh2d, Vec2::new(1.0, 0.0))
        .cubic_bezier(
            Vec2::new(1.0, -circle_len),
            Vec2::new(circle_len, -1.0),
            Vec2::new(0.0, -1.0),
        )
        .cubic_bezier(
            Vec2::new(-circle_len, -1.0),
            Vec2::new(-1.0, -circle_len),
            Vec2::new(-1.0, 0.0),
        )
        .cubic_bezier(
            Vec2::new(-1.0, circle_len),
            Vec2::new(-circle_len, 1.0),
            Vec2::new(0.0, 1.0),
        )
        .close(Default::default());
        */

    println!("{:?}", mesh2d);

    let mut mesh3d = mesh2d.to_3d(0.01);
    mesh3d.extrude_boundary(Transform::from_translation(Vec3::new(0.0, 0.0, -0.2)));
    mesh3d
}

fn _read_svg(settings: &GlobalSettings) -> BevyMesh3d {
    // TODO: Handle self-intersecting svg paths etc
    /*let svg = "
    <svg xmlns='http://www.w3.org/2000/svg' width='320' height='320'>
        <path d='M 10 315
                L 110 215
                A 36 60 0 0 1 150.71 170.29
                L 172.55 152.45
                A 30 50 -45 0 1 215.1 109.9
                L 315 10' stroke='black' fill='green' stroke-width='2' fill-opacity='0.5'/>
        <circle cx='150.71' cy='170.29' r='2' fill='red'/>
        <circle cx='110' cy='215' r='2' fill='red'/>
        <ellipse cx='144.931' cy='229.512' rx='36' ry='60' fill='transparent' stroke='blue'/>
        <ellipse cx='115.779' cy='155.778' rx='36' ry='60' fill='transparent' stroke='blue'/>
    </svg>";*/

    let svg = "<path d='M913.88 548.4c-66.14 35.43-141.83-7.68-141.83-7.68-112.76-53.91-246.31-55.82-246.31-55.82-34.09-2.34-25.47-17.51-20.69-25.88 0.73-1.27 1.74-2.36 2.59-3.56a187.06 187.06 0 0 0 34.17-108.08c0-103.78-84.13-187.92-187.92-187.92C251 159.47 167.37 242.24 166 344.87c-1 3.81-42.28 9.32-73-5.06-40-18.71-25.08 73.65 42.35 95.45l-2.31-0.1c-28.06-1.52-30.8 7.68-30.8 7.68s-16.14 29.75 83.13 38.37c31.39 2.72 35.71 8.11 42 16.64 11.92 16.14 3.57 39.25-12.15 59-44.53 55.77-71.84 180.68 49.78 270.85 103.12 76.47 377.65 79.95 497.37-15.13 108-85.76 156.72-170.47 185.79-241.14 3.9-9.54 31.84-58.43-34.28-23.03z' fill='#DFEDFF'/>";

    let mut m2d = BackendSVG::<BevyMeshType2d32>::from_svg(svg);
    println!("{:?}", m2d);

    let mut m3d = m2d
        .scale(&Vec2::splat(-0.004))
        .translate(&Vec2::new(2.0, 2.0))
        .to_3d(settings.tol);
    m3d.extrude(0, Transform::from_translation(Vec3::new(0.0, 0.0, -0.2)));
    m3d
}

fn make_mesh(_settings: &GlobalSettings) -> BevyMesh3d {
    //_make_hell_8()
    //BevyMesh3d::regular_polygon(1.0, 10)
    //_make_spiral()
    //BevyMesh3d::cone(1.0, 1.0, 16)
    //BevyMesh3d::regular_antiprism(1.0, 1.0, 8)
    //BevyMesh3d::uniform_antiprism(1.0, 16)
    //BevyMesh3d::regular_prism(1.0, 1.0, 8)
    //BevyMesh3d::uniform_prism(1.0, 8)
    //BevyMesh3d::regular_frustum(1.0, 0.5, 1.0, 8, false)
    //BevyMesh3d::regular_pyramid(1.0, 1.0, 8)
    //BevyMesh3d::regular_octahedron(1.0)
    //BevyMesh3d::tetrahedron(1.0)
    //BevyMesh3d::dodecahedron(1.0) // TODO: crash?

    /*let mut mesh = BevyMesh3d::hex_plane(10, 8);
    mesh.flip_yz();
    mesh*/

    //BevyMesh3d::uv_sphere(3.0, 64, 64)
    //BevyMesh3d::geodesic_icosahedron(3.0, 64)
    //BevyMesh3d::geodesic_tetrahedron(3.0, 128)
    //BevyMesh3d::geodesic_octahedron(3.0, 128)

    //BevyMesh3d::regular_polygon(2.0, 600)
    //_make_2d_zigzag()

    //_make_hell_10()

    //_make_blechnum_spicant(_settings)

    //_make_bezier(_settings)
    _read_svg(_settings)
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full", "1", "0"

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920.0, 1080.0),
                position: WindowPosition::Centered(MonitorSelection::Index(1)),
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WireframePlugin)
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .register_type::<GlobalSettings>()
        .insert_resource(GlobalSettings::default())
        .register_type::<MeshSettings>()
        .add_plugins((
            ResourceInspectorPlugin::<GlobalSettings>::default(),
            FilterQueryInspectorPlugin::<With<MeshSettings>>::default(),
            WorldInspectorPlugin::default(),
            PanOrbitCameraPlugin,
            Text3dGizmosPlugin,
        ))
        .add_systems(Startup, setup_meshes)
        .add_systems(Update, update_meshes)
        .add_systems(Update, exit_on_esc)
        .run();
}

fn exit_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

fn update_meshes(
    //query: Query<(&Handle<Mesh>, &MeshSettings), Changed<MeshSettings>>,
    query: Query<(&Mesh3d, &MeshSettings)>,
    mut assets: ResMut<Assets<Mesh>>,
    mut global_settings: ResMut<GlobalSettings>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut texts: ResMut<Text3dGizmos>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    if let Some(ray) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
    {
        let distance = ray
            .intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
            .unwrap_or(0.0);
        let world_position = ray.get_point(distance);
        if global_settings.px != world_position.x || global_settings.py != world_position.z {
            global_settings.px = world_position.x;
            global_settings.py = world_position.z;
        }
    }

    if !global_settings.is_changed() {
        return;
    }

    for (bevy_mesh, _settings) in query.iter() {
        let mut mesh = make_mesh(&global_settings);

        // place it "on the floor"
        let min_y = mesh
            .vertices()
            .map(|v| v.pos().y)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        mesh.translate(&Vec3::new(0.0, -min_y, 0.0));

        let mut meta = TesselationMeta::default();
        mesh.generate_smooth_normals();
        mesh.bevy_set_ex(
            assets.get_mut(bevy_mesh).unwrap(),
            TriangulationAlgorithm::MinWeight,
            true,
            &mut meta,
        );

        show_tesselation_meta(&mut texts, &mesh, &meta);
    }
}

fn setup_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texts: ResMut<Text3dGizmos>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0))))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.6, 0.4),
            double_sided: true,
            cull_mode: None,
            ..default()
        })),
        Name::new("Floor"),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0))))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.9, 0.9),
            //alpha_mode: AlphaMode::Blend,
            double_sided: true,
            cull_mode: None,
            ..default()
        })),
        MeshSettings::default(),
        Name::new("Generated Shape"),
    ));

    if false {
        let mesh = make_mesh(&GlobalSettings::default());
        show_vertex_indices(&mut texts, &mesh);
        show_edges(&mut texts, &mesh, 0.1);
        show_faces(&mut texts, &mesh);
    }

    commands.insert_resource(AmbientLight::default());
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // very high quality shadows
        CascadeShadowConfigBuilder {
            num_cascades: 8,
            first_cascade_far_bound: 5.0,
            maximum_distance: 55.0,
            ..default()
        }
        .build(),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 7.0, 5.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        PanOrbitCamera::default(),
        ShadowFilteringMethod::Gaussian,
    ));
}
