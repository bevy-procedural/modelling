use crate::{
    math::Vector,
    mesh::{
        CurvedEdge, DefaultEdgePayload, DefaultFacePayload, EuclideanMeshType, MeshTypeHalfEdge,
        PathBuilder,
    },
};

fn import_group<T: EuclideanMeshType<2> + MeshTypeHalfEdge>(mesh: &mut T::Mesh, group: &usvg::Group)
where
    T::Edge: CurvedEdge<2, T>,
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    for c in group.children() {
        match c {
            usvg::Node::Group(g) => {
                import_group::<T>(mesh, g.as_ref());
            }
            usvg::Node::Path(p) => {
                import_path::<T>(mesh, p.as_ref());
            }
            usvg::Node::Text(t) => {
                println!("Text: {:#?}", t);
                todo!();
            }
            usvg::Node::Image(i) => {
                println!("Image: {:#?}", i);
                todo!();
            }
        }
    }
}

fn import_path<T: EuclideanMeshType<2> + MeshTypeHalfEdge>(mesh: &mut T::Mesh, path: &usvg::Path)
where
    T::Edge: CurvedEdge<2, T>,
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    if !path.is_visible() {
        return;
    }

    // let fill = path.fill();
    // let stroke = path.stroke();
    // let po = path.paint_order();
    // let transform = path.abs_transform();

    let v = |p: usvg::tiny_skia_path::Point| T::Vec::from_xy(T::S::from(p.x), T::S::from(p.y));

    let mut pb = PathBuilder::<T, T::Trans>::new(mesh);

    let mut is_first = true;
    for s in path.data().segments() {
        match s {
            usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                assert!(is_first);
                pb.move_to_new(v(p));
                //pb = PathBuilder::<T>::start(mesh, v(p));
            }
            usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                let end = pb.add_vertex_autoclose(v(p));
                pb.line_to(end);
            }
            usvg::tiny_skia_path::PathSegment::QuadTo(c1, p) => {
                let end = pb.add_vertex_autoclose(v(p));
                pb.quad_to(v(c1), end);
            }
            usvg::tiny_skia_path::PathSegment::CubicTo(c1, c2, p) => {
                let end = pb.add_vertex_autoclose(v(p));
                //pb.line_to(end);
                pb.cubic_to(v(c1), v(c2), end);
            }
            usvg::tiny_skia_path::PathSegment::Close => {
                pb.close(Default::default());
            }
        }
        is_first = false;
    }

    if !pb.has_face() {
        pb.close(Default::default());
    }
}

pub(super) fn import_svg<T: EuclideanMeshType<2> + MeshTypeHalfEdge>(mesh: &mut T::Mesh, svg: &str)
where
    T::Edge: CurvedEdge<2, T>,
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    let res = usvg::Tree::from_str(&svg, &usvg::Options::default());
    if let Err(e) = res {
        match e {
            usvg::Error::ParsingFailed(reason) => match reason {
                usvg::roxmltree::Error::NoRootNode => {
                    import_svg::<T>(
                        mesh,
                        ("<svg xmlns='http://www.w3.org/2000/svg'>".to_string() + svg + "</svg>")
                            .as_str(),
                    );
                    return;
                }
                _ => {
                    panic!("Failed to parse SVG: {:#?}", reason);
                }
            },
            _ => {
                panic!("Failed to parse SVG: {:#?}", e);
            }
        }
    }
    let tree = res.expect("Failed to parse SVG");
    import_group::<T>(mesh, tree.root());
}
