use super::{MeshHalfEdgeBuilder, MeshType, PathBuilder};
use crate::{
    math::{HasPosition, Vector},
    mesh::{CurvedEdge, DefaultEdgePayload, DefaultFacePayload, HalfEdge},
};

fn import_group<T: MeshType>(mesh: &mut T::Mesh, group: &usvg::Group)
where
    T::Edge: CurvedEdge<T> + HalfEdge<T>,
    T::Mesh: MeshHalfEdgeBuilder<T>,
    T::VP: HasPosition<T::Vec, S = T::S>,
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

fn import_path<T: MeshType>(mesh: &mut T::Mesh, path: &usvg::Path)
where
    T::Edge: CurvedEdge<T> + HalfEdge<T>,
    T::Mesh: MeshHalfEdgeBuilder<T>,
    T::VP: HasPosition<T::Vec, S = T::S>,
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

    let v = |p: usvg::tiny_skia_path::Point| {
        T::Vec::from_xy(T::S::from(p.x * 0.01), T::S::from(p.y * 0.01))
    };

    let mut pb = PathBuilder::<T>::new(mesh);

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
}

pub(crate) fn import_svg<T: MeshType>(mesh: &mut T::Mesh, svg: &str)
where
    T::Edge: CurvedEdge<T> + HalfEdge<T>,
    T::Mesh: MeshHalfEdgeBuilder<T>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::FP: DefaultFacePayload,
    T::EP: DefaultEdgePayload,
{
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).expect("Failed to parse SVG");
    import_group::<T>(mesh, tree.root());
}
