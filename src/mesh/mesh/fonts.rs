use ab_glyph::{Font as AbFont, FontRef, GlyphId, ScaleFont};

use crate::{
    math::{HasPosition, IndexType, Transformable, Vector},
    mesh::{
        CurvedEdge, CurvedEdgeType, DefaultEdgePayload, DefaultFacePayload, MeshBasics, MeshBuilder,
    },
};

use super::{MeshHalfEdgeBuilder, MeshType};

/// A font that can be used to render text.
pub struct Font<'a> {
    font: FontRef<'a>,
    scale: f32,
}

impl<'a> Font<'a> {
    /// Create a new font from the given data.
    pub fn new(data: &'a [u8], scale: f32) -> Self {
        let font = FontRef::try_from_slice(data).expect("Failed to load font");
        Self { font, scale }
    }

    /// Set the scale of the font.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn font_scale(&self) -> f32 {
        self.scale //* self.font.pt_to_px_scale(1.0).map(|x| x.x).unwrap_or(1.0)
    }

    /// Get the advance of the given character.
    pub fn get_advance(&self, c: char) -> f32 {
        self.font
            .as_scaled(self.font_scale())
            .h_advance(self.font.glyph_id(c))
    }

    /// Layout the given text.
    pub fn layout_text<T: MeshType>(&self, text: &str, mesh: &mut T::Mesh)
    where
        T::Edge: CurvedEdge<T>,
        T::VP: HasPosition<T::Vec, S = T::S>,
        T::Vec: Transformable<S = T::S>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
        T::FP: DefaultFacePayload,
    {
        let mut x_pos = 0.0;
        let mut last_glyph_id = None;

        let scaled_font = self.font.as_scaled(self.font_scale());

        for c in text.chars() {
            let glyph_id = scaled_font.glyph_id(c);

            if let Some(last_id) = last_glyph_id {
                x_pos += scaled_font.kern(last_id, glyph_id);
            }

            /*let glyph = Glyph {
                id: glyph_id,
                position: Point { x: x_pos, y: 0.0 },
                scale: PxScale::from(self.scale),
            };*/

            self.draw_glyph_outlines::<T>(glyph_id, x_pos, mesh);

            x_pos += scaled_font.h_advance(glyph_id);
            last_glyph_id = Some(glyph_id);
        }
    }

    fn draw_glyph_outlines<T: MeshType>(&self, glyph: GlyphId, x_pos: f32, mesh: &mut T::Mesh)
    where
        T::Edge: CurvedEdge<T>,
        T::VP: HasPosition<T::Vec, S = T::S>,
        T::Vec: Transformable<S = T::S>,
        T::Mesh: MeshHalfEdgeBuilder<T>,
        T::EP: DefaultEdgePayload,
        T::FP: DefaultFacePayload,
    {
        // TODO: Improve stability. Detect when to close and when to insert holes

        let Some(outline) = self.font.outline(glyph) else {
            println!("No outline found for glyph");
            return;
        };

        let scale = T::S::from(
            self.font
                .pt_to_px_scale(self.scale)
                .map(|x| x.x)
                .unwrap_or(self.scale)
                / self.font.height_unscaled(),
        );
        let mut first = None;
        let mut cur = None;
        let mut cur_v = IndexType::max();
        let mut start_v = IndexType::max();
        let mut prev_v = IndexType::max();

        outline.curves.iter().for_each(|curve| {
            let (p0, p1, c0, c1) = match curve {
                ab_glyph::OutlineCurve::Line(p0, p1) => (
                    T::Vec::from_xy(T::S::from(p0.x), T::S::from(p0.y)),
                    T::Vec::from_xy(T::S::from(p1.x), T::S::from(p1.y)),
                    None,
                    None,
                ),
                ab_glyph::OutlineCurve::Quad(p0, p1, p2) => (
                    T::Vec::from_xy(T::S::from(p0.x), T::S::from(p0.y)),
                    T::Vec::from_xy(T::S::from(p2.x), T::S::from(p2.y)),
                    Some(T::Vec::from_xy(T::S::from(p1.x), T::S::from(p1.y))),
                    None,
                ),
                ab_glyph::OutlineCurve::Cubic(p0, p1, p2, p3) => (
                    T::Vec::from_xy(T::S::from(p0.x), T::S::from(p0.y)),
                    T::Vec::from_xy(T::S::from(p3.x), T::S::from(p3.y)),
                    Some(T::Vec::from_xy(T::S::from(p1.x), T::S::from(p1.y))),
                    Some(T::Vec::from_xy(T::S::from(p2.x), T::S::from(p2.y))),
                ),
            };

            if p0 == p1 && c0.is_none() && c1.is_none() {
                return;
            }

            //println!("Adding edge from {:?} to {:?} {:?} {:?}", p0, p1, c0, c1);

            let cur_e;

            let trans = T::Vec::from_x(T::S::from(x_pos));
            if let Some(_p) = cur {
                // TODO: don't use eq but similarity
                if first.unwrap() == p1 {
                    let (_, _, e) = mesh.close_face_vertices(
                        prev_v,
                        Default::default(),
                        cur_v,
                        Default::default(),
                        start_v,
                        Default::default(),
                        false,
                    );
                    cur_v = IndexType::max();
                    prev_v = IndexType::max();
                    start_v = IndexType::max();
                    first = None;
                    cur = None;
                    cur_e = e;
                } else {
                    //assert!(p0 == p || p1 == first.unwrap(), "Expected {:?} but got {:?}", p, p0);
                    let (v, e, _) = mesh
                        .add_vertex_via_vertex_default(cur_v, T::VP::from_pos(p1 * scale + trans));
                    cur_e = e;
                    prev_v = cur_v;
                    cur_v = v;
                    cur = Some(p1);
                }
            } else {
                first = Some(p0);
                cur = Some(p1);
                let (v0, v1) = mesh.add_isolated_edge_default(
                    T::VP::from_pos(p0 * scale + trans),
                    T::VP::from_pos(p1 * scale + trans),
                );
                cur_v = v1;
                prev_v = v0;
                start_v = v0;
                cur_e = mesh.shared_edge_id(v0, v1).unwrap();
            }

            if c1.is_some() {
                mesh.edge_mut(cur_e)
                    .set_curve_type(CurvedEdgeType::CubicBezier(
                        c0.unwrap() * scale + trans,
                        c1.unwrap() * scale + trans,
                    ));
            } else if c0.is_some() {
                mesh.edge_mut(cur_e)
                    .set_curve_type(CurvedEdgeType::QuadraticBezier(c0.unwrap() * scale + trans));
            }
        });
    }
}
