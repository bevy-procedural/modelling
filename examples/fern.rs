//! In this example, we will construct a detailled fern leaf.

use std::f32::consts::PI;

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use procedural_modelling::{extensions::bevy::*, prelude::*};
#[path = "common/bevy.rs"]
mod bevy_examples;

struct GlobalSettings {
    prog: f32,
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

fn make_blechnum_spicant(settings: &GlobalSettings) -> BevyMesh3d {
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

fn generate_path(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = make_blechnum_spicant(&GlobalSettings::default());

    commands.spawn((
        Mesh3d(meshes.add(mesh.to_bevy_ex(
            RenderAssetUsages::all(),
            TriangulationAlgorithm::Auto,
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
