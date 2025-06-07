use super::{
    lighting::{calculate_vertex_coords, lighting},
    Render2SVGSettings,
};
use crate::{extensions::nalgebra::*, prelude::*};
use std::collections::{HashMap, HashSet};

pub(super) fn format6<T: Scalar>(x: T) -> String {
    if x.is_zero() {
        return "0".to_string();
    }
    let mut s = format!("{:.6}", x.as_f64());
    if s.contains('.') {
        s = s.trim_end_matches('0').to_string();
        if s.ends_with('.') {
            s.pop();
        }
    }
    s
}

fn anim<T: MeshType3D, I: Iterator<Item = String>>(
    values: I,
    attr: &str,
    settings: &Render2SVGSettings<T::S>,
) -> (String, String) {
    let vec = values.collect::<Vec<_>>();

    if vec.len() == 0 {
        return ("".to_string(), "".to_string());
    }

    // if all the same
    if vec.len() == 1 || vec.iter().all(|v| *v == vec[0]) {
        return (format!(r#" {}="{}""#, attr, vec[0]), "".to_string());
    }

    (
        "".to_string(),
        format!(
            r#"<animate attributeName="{}" dur="{}" repeatCount="indefinite" values="{}"/>"#,
            attr,
            settings.dur,
            vec.join(";"),
        ),
    )
}

fn xy_anim<T: MeshType3D>(
    vertex_coords: &HashMap<T::V, Vec<(T::Vec, T::Vec)>>,
    id: T::V,
    attr_x: &str,
    attr_y: &str,
    _vertex_should_render: &Vec<HashSet<T::V>>,
    settings: &Render2SVGSettings<T::S>,
) -> (String, String) {
    let (a, b) = anim::<T, _>(
        vertex_coords[&id].iter().enumerate().map(|(_i, (_, p))| {
            //if vertex_should_render[i].contains(&id) {
            format6(p.x())
            //} else {
            // TODO: we cannot safely send to 0, since this position might be visible!
            //    "0".to_string()
            //}
        }),
        attr_x,
        settings,
    );

    let (c, d) = anim::<T, _>(
        vertex_coords[&id].iter().enumerate().map(|(_i, (_, p))| {
            //if vertex_should_render[i].contains(&id) {
            format6(p.y())
            //} else {
            //    "0".to_string()
            //}
        }),
        attr_y,
        settings,
    );

    (a + &c, d + &b)
}

fn draw_vertices<T: MeshType3D>(
    elements: &mut Vec<(String, T::S)>,
    mesh: &T::Mesh,
    vertex_should_render: &Vec<HashSet<T::V>>,
    vertex_coords: &HashMap<T::V, Vec<(T::Vec, T::Vec)>>,
    settings: &Render2SVGSettings<T::S>,
    face_centroids: &HashMap<T::F, T::Vec>,
) where
    T::S: ScalarPlus,
{
    for v in mesh.vertices() {
        // If the vertex has adjacent to faces but those faces were culled, we don't want to draw the vertex.
        if !vertex_should_render[2 * settings.steps].contains(&v.id())
            && v.fork().faces().next().is_some()
        {
            continue;
        }

        // maximum of the face centroids
        let z: T::S = v
            .fork()
            .face_ids()
            .map(|f| face_centroids[&f].z())
            .fold(v.pos().z(), |a, b| a.max(b))
            - settings.id_offset;

        if settings.vertex_size > 0.0 {
            let (a, b) = xy_anim::<T>(
                &vertex_coords,
                v.id(),
                "cx",
                "cy",
                vertex_should_render,
                settings,
            );
            let c = if settings.steps == 0 || b.is_empty() {
                format!(r#"<circle class="vc"{}/>"#, a)
            } else {
                format!(r#"<circle class="vc"{}>{}</circle>"#, a, b)
            };
            elements.push((c, z));
        }

        if settings.vertex_id_size > 0.0 {
            let (a, b) = xy_anim::<T>(
                &vertex_coords,
                v.id(),
                "x",
                "y",
                vertex_should_render,
                settings,
            );
            elements.push((
                format!(r#"<text class="vid"{}>{}{}</text>"#, a, v.id(), b,),
                z - settings.id_offset,
            ));
        }
    }
}

fn draw_face<T: MeshType3D>(
    id: T::F,
    settings: &Render2SVGSettings<T::S>,
    centroids: &Vec<T::Vec>,
) -> String {
    let (a, b) = anim::<T, _>(centroids.iter().map(|p| format6(p.x())), "x", settings);
    let (c, d) = anim::<T, _>(centroids.iter().map(|p| format6(p.y())), "y", settings);
    format!(r#"<text class="fid"{}{}>{}{}{}</text>"#, a, c, id, d, b,)
}

fn draw_faces<T: MeshType3D>(
    elements: &mut Vec<(String, T::S)>,
    mesh: &T::Mesh,
    vertex_coords: &HashMap<T::V, Vec<(T::Vec, T::Vec)>>,
    settings: &Render2SVGSettings<T::S>,
) -> (Vec<HashSet<T::V>>, HashMap<T::F, T::Vec>)
where
    T::S: ScalarPlus,
{
    let view_direction = T::Vec::from_xyz(T::S::ZERO, T::S::ZERO, -T::S::ONE).normalize();
    let mut vertex_should_render: Vec<HashSet<T::V>> = Vec::new();
    let mut face_centroids = HashMap::<T::F, T::Vec>::new();

    for _ in 0..=(2 * settings.steps).max(1) {
        vertex_should_render.push(HashSet::<T::V>::new());
    }

    for f in mesh.faces() {
        assert!(!f.has_islands()); // TODO: Implement islands natively

        let mut ani_points = Vec::new();
        let mut ani_fill = Vec::new();
        let mut centroids = Vec::<T::Vec>::new();
        let mut any_frontfacing = false;
        let mut frontfacing = vec![false; (2 * settings.steps).max(1)];
        for i in 0..(2 * settings.steps).max(1) {
            let mut pos_list = Vec::<T::Vec>::new();
            let mut raw_pos_list = Vec::<T::Vec>::new();
            for v in f.vertices() {
                let (raw_pos, pos) = vertex_coords[&v.id()][i];
                raw_pos_list.push(raw_pos);
                pos_list.push(pos);
            }

            centroids.push(Vector::stable_mean(pos_list.iter().cloned()));
            let normal = raw_pos_list.clone().into_iter().normal().normalize();
            let fill = lighting(
                settings.light_direction,
                settings.diffuse_color,
                settings.ambient_color,
                Vec3::<f32>::from_xyz(
                    normal.x().as_f64() as f32,
                    normal.y().as_f64() as f32,
                    normal.z().as_f64() as f32,
                ),
            );
            ani_fill.push(rgb2hex(fill.x(), fill.y(), fill.z()));

            let t_normal = pos_list.clone().into_iter().normal().normalize();

            // check whether frontfacing
            frontfacing[i] = t_normal.dot(&view_direction) > T::S::ZERO;
            any_frontfacing |= frontfacing[i];

            ani_points.push(pos_list);
        }

        let c = centroids[0]; // TODO
        face_centroids.insert(f.id(), c.clone());

        // backface culling
        if !any_frontfacing {
            continue;
        }

        for v in f.vertices() {
            // the last one indicates whether any was frontfacing
            vertex_should_render[2 * settings.steps].insert(v.id());
            // don't render a vertex at timesteps where the face is not visible for any adjacent step and face
            for i in 0..(2 * settings.steps) {
                if frontfacing[i]
                    || frontfacing[(i + 1) % (2 * settings.steps)]
                    || frontfacing[(i + 2) % (2 * settings.steps)]
                {
                    vertex_should_render[(i + 1) % (2 * settings.steps)].insert(v.id());
                }
            }
        }

        // do some backface culling in the animation by collapsing the backfacing faces to the most distant vertex
        for i in 0..(2 * settings.steps) {
            let i1 = (i + 1) % (2 * settings.steps);
            if settings.show_only_hidden_faces
                == (frontfacing[i]
                    || frontfacing[i1]
                    || frontfacing[(i + 2) % (2 * settings.steps)])
            {
                // TODO: this is ony a heuristic. The best vertex is the one that is adjacent to a group of faces that are in front of this backface and completely cover the backface.
                let s0 = ani_points[i1].iter().fold(ani_points[i1][0], |a, &b| {
                    if a.z() < b.z() {
                        a
                    } else {
                        b
                    }
                });
                let l = ani_points[i1].len();
                ani_points[i1] = vec![s0; l];
                ani_fill[i1] = "transparent".to_string();
            }
        }

        let (points1, points2) = anim::<T, _>(
            ani_points.iter().map(|vp| {
                vp.iter()
                    .map(|p| format!("{},{}", format6(p.x()), format6(p.y())))
                    .collect::<Vec<_>>()
                    .join(" ")
            }),
            "points",
            settings,
        );
        let (fill1, fill2) = anim::<T, _>(ani_fill.iter().cloned(), "fill", settings);
        let poly = if settings.steps == 0 || (points2.is_empty() && fill2.is_empty()) {
            format!(r#"<polygon class="pid"{}{}/>"#, points1, fill1,)
        } else {
            format!(
                r#"<polygon class="pid"{}{}>{}{}</polygon>"#,
                points1, fill1, points2, fill2
            )
        };
        elements.push((poly, c.z()));

        if settings.face_id_size > 0.0 {
            // add face id, center on the polygon
            elements.push((
                draw_face::<T>(f.id(), &settings, &centroids),
                c.z() - settings.id_offset,
            ));
        }
    }

    (vertex_should_render, face_centroids)
}

/// Draw edges that are not part of any face.
fn draw_edges<T: MeshType3D>(
    elements: &mut Vec<(String, T::S)>,
    vertex_should_render: &mut Vec<HashSet<T::V>>,
    mesh: &T::Mesh,
    vertex_coords: &HashMap<T::V, Vec<(T::Vec, T::Vec)>>,
    settings: &Render2SVGSettings<T::S>,
) where
    T::S: ScalarPlus,
{
    // index that stores the “always-render” set created in `draw_faces`
    let always = vertex_should_render.len() - 1;

    for e in mesh.edges() {
        // ignore edges already represented by (at least one) face
        if e.has_faces() {
            continue;
        }

        let v0 = e.origin_id();
        let v1 = e.target_id();

        // ensure the two vertices will be rendered
        vertex_should_render[always].insert(v0);
        vertex_should_render[always].insert(v1);

        // animated SVG attributes
        let (a1, b1) = xy_anim::<T>(
            vertex_coords,
            v0,
            "x1",
            "y1",
            vertex_should_render,
            settings,
        );
        let (a2, b2) = xy_anim::<T>(
            vertex_coords,
            v1,
            "x2",
            "y2",
            vertex_should_render,
            settings,
        );

        // z-sorting key: foremost of the two end points
        let p0z = vertex_coords[&v0][0].1.z();
        let p1z = vertex_coords[&v1][0].1.z();
        let z = p0z.max(p1z) - settings.id_offset;

        let svg = if settings.steps == 0 || (b1.is_empty() && b2.is_empty()) {
            // static line
            format!(r#"<line class="pid"{}{} />"#, a1, a2)
        } else {
            // animated line
            format!(r#"<line class="pid"{}{}>{}{}</line>"#, a1, a2, b1, b2)
        };

        elements.push((svg, z));
    }
}

/// Like [render2svg] but with a preconfigured wiggle animation.
///
/// Choose a small wiggle_angle, e.g., 0.3 to avoid artifacts (the SVG cannot handle changing z-ordering!).
pub fn render2svg_wiggle<S: ScalarPlus, T: MeshType3D<S = S, Trans = NdHomography<S, 3>>>(
    mesh: &T::Mesh,
    wiggle_angle: S,
    settings: &Render2SVGSettings<S>,
) -> String {
    let perspective = NdHomography::<S, 3>::from_perspective(
        settings.aspect,
        settings.fov_y,
        settings.z_near,
        settings.z_far,
    );
    let lookat = NdHomography::<S, 3>::look_at_lh(&settings.eye, &settings.target, &settings.up);

    render2svg::<T, _>(mesh, &perspective.chain(&lookat), &settings, |t: S| {
        let angle = wiggle_angle * (S::PI * t).sin(); // ease-in-out
        NdHomography::from_rotation(NdRotate::from_axis_angle(Vec3::<T::S>::y_axis(), angle))
    })
}

/// A tiny 3d renderer producing animated SVGs.
/// Mostly for debugging purposes, since it has zero dependencies and gives you very fast results.
/// Use [Render2SVGSettings] to configure the pipeline.
///
/// When you set `steps = 0`, the mesh will be rendered as a static SVG instead of an animation.
///
///
/// If you set `steps > 0` the svg will be animated. It will be animated using the transform function `f`, e.g.,
///
/// ```rust
/// |t: f64| {
///    let angle = wiggle_angle * (std::f64::consts::PI * t).sin(); // ease-in-out
///    NdAffine::from_rotation(NdRotate::from_axis_angle(Vec3::<f64>::y_axis(), angle))
/// }
/// ```
///
/// Be careful to only use small movements. SVG doesn't support z-ordering, hence, we are using a rather sketchy workaround
/// that relies on dynamically shrinking backfaces to a single point.
/// However, this has its limits and fast big movements result in significant artifacts.
/// Increasing the number of steps decreases artifacts but also increases the size of the SVG.
///
/// Also, be aware that a large svg animation is not very performant in the browser.
/// You should usually go for less than 100 faces in your mesh when animating it.
pub fn render2svg<T: MeshType3D, F: Fn(T::S) -> T::Trans>(
    mesh: &T::Mesh,
    transform: &T::Trans,
    settings: &Render2SVGSettings<T::S>,
    f: F,
) -> String
where
    T::S: ScalarPlus,
{
    let mut elements = Vec::<(String, T::S)>::new();
    let vertex_coords = calculate_vertex_coords::<T, F>(&mesh, settings.steps, transform, f);

    //debug_assert_eq!(mesh.check(), Ok(()));

    let (mut vertex_should_render, face_centroids) =
        draw_faces::<T>(&mut elements, &mesh, &vertex_coords, settings);

    draw_edges::<T>(
        &mut elements,
        &mut vertex_should_render,
        &mesh,
        &vertex_coords,
        settings,
    );

    draw_vertices::<T>(
        &mut elements,
        &mesh,
        &vertex_should_render,
        &vertex_coords,
        settings,
        &face_centroids,
    );

    // sort the elements by z value
    elements.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let vid = if settings.vertex_id_size > 0.0 {
        format!(
            ".vid{{font-size:{}px;fill:{};text-anchor:middle;alignment-baseline:middle}}",
            format6(settings.vertex_id_size),
            settings.vertex_id_color
        )
    } else {
        "".to_string()
    };
    let pid = if settings.stroke_width > 0.0 {
        format!(
            ".pid{{stroke:{};stroke-width:{};stroke-linejoin:bevel}}",
            settings.stroke_color,
            format6(settings.stroke_width)
        )
    } else {
        "".to_string()
    };
    let vc = if settings.vertex_size > 0.0 {
        format!(
            ".vc{{r:{};fill:{}}}",
            format6(settings.vertex_size),
            settings.vertex_color
        )
    } else {
        "".to_string()
    };
    let fid = if settings.face_id_size > 0.0 {
        format!(
            ".fid{{font-size:{}px;fill:{};text-anchor:middle;alignment-baseline:middle}}",
            format6(settings.face_id_size),
            settings.face_id_color
        )
    } else {
        "".to_string()
    };

    return format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1.1 -1.1 2.2 2.2" width="1000" height="1000"><style>{}{}{}{}</style>{}</svg>"#,
        vid,
        pid,
        vc,
        fid,
        elements
            .iter()
            .map(|(s, _)| s.clone())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
