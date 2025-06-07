use crate::{
    math::{HasPosition, Polygon, Scalar, Vector},
    mesh::{
        cursor::*, CurvedEdge, CurvedEdgeType, DefaultEdgePayload, DefaultFacePayload,
        EuclideanMeshType, HalfEdge, MeshBasics, MeshBuilder, MeshType, MeshTypeHalfEdge,
    },
};
use ttf_parser::{Face, GlyphId, OutlineBuilder};

/// A font that can be used to render text.
pub struct Font<'a> {
    //font: FontRef<'a>,
    face: Face<'a>,
    px_scale: f32,     // requested pixel height
    units_per_em: u16, // face‑internal design units
}

/// Horizontal kerning in font units (design‑units).
fn kerning_units(face: &Face<'_>, left: GlyphId, right: GlyphId) -> i16 {
    let Some(kern) = face.tables().kern else {
        return 0;
    };
    for st in kern.subtables {
        // Skip vertical or variable subtables, keep the first horizontal hit.
        if !st.horizontal || st.variable {
            continue;
        }
        if let Some(k) = st.glyphs_kerning(left, right) {
            return k;
        }
    }
    0
}

impl<'a> Font<'a> {
    /// Create a new font from the given data.
    pub fn new(data: &'a [u8], px_scale: f32) -> Self {
        let face = Face::parse(data, 0).expect("invalid font slice");
        let units_per_em = face.units_per_em();
        Self {
            face,
            px_scale,
            units_per_em,
        }
    }

    /// Set the scale of the font in pixels.
    pub fn set_scale(&mut self, px: f32) {
        self.px_scale = px;
    }

    /// Get the scale of the font in pixels.
    pub fn get_scale(&self) -> f32 {
        self.px_scale
    }

    /// Get the scaling factor of the font (not in pixels).
    fn font_scale(&self) -> f32 {
        self.px_scale / (self.units_per_em as f32)
    }

    #[inline]
    fn gid(&self, ch: char) -> GlyphId {
        self.face.glyph_index(ch).unwrap_or(GlyphId(0))
    }

    /// Get the advance of the given character.
    pub fn get_advance(&self, ch: char) -> f32 {
        let gid = self.gid(ch);
        self.face.glyph_hor_advance(gid).unwrap_or(0) as f32 * self.font_scale()
    }

    /// Layout the given text.
    pub fn layout_text<const D: usize, T: MeshType>(&self, text: &str, mesh: &mut T::Mesh)
    where
        T::Edge: CurvedEdge<D, T>,
        T: EuclideanMeshType<D> + MeshTypeHalfEdge,
        T::EP: DefaultEdgePayload,
        T::FP: DefaultFacePayload,
    {
        let s = self.font_scale();
        let mut pen_x = 0.0f32;
        let mut prev = None;

        for ch in text.chars() {
            let gid = self.gid(ch);

            // kerning
            if let Some(p) = prev {
                pen_x += kerning_units(&self.face, p, gid) as f32 * s;
            }

            let mut builder =
                MeshOutlineBuilder::<D, T>::new(mesh, T::S::from(s), T::S::from(pen_x));
            self.face.outline_glyph(gid, &mut builder);
            builder.finish_ring(); // last ring if glyph had no explicit `close`
            assert!(builder.is_done(), "outline builder not done");

            // advance
            pen_x += self.face.glyph_hor_advance(gid).unwrap_or(0) as f32 * s;
            prev = Some(gid);
        }
    }
}

struct MeshOutlineBuilder<'m, const D: usize, T>
where
    T: EuclideanMeshType<D> + MeshTypeHalfEdge,
    T::Edge: CurvedEdge<D, T> + HalfEdge<T>,
{
    mesh: &'m mut T::Mesh,
    scale: T::S,
    dx: T::S, // horizontal offset of glyph in world coords
    first_v: Option<(T::V, (f32, f32))>,
    cur_v: Option<T::V>,
    poly: T::Poly,
    unassigned_holes: Vec<(T::F, T::Poly)>,
    components: Vec<(T::F, T::Poly)>,
}

impl<'m, const D: usize, T> MeshOutlineBuilder<'m, D, T>
where
    T: EuclideanMeshType<D> + MeshTypeHalfEdge,
    T::Edge: CurvedEdge<D, T> + HalfEdge<T>,
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    fn new(mesh: &'m mut T::Mesh, scale: T::S, dx: T::S) -> Self {
        Self {
            mesh,
            scale,
            dx,
            first_v: None,
            cur_v: None,
            poly: T::Poly::empty(),
            unassigned_holes: Vec::new(),
            components: Vec::new(),
        }
    }

    #[inline]
    fn p(&self, x: f32, y: f32) -> T::Vec {
        T::Vec::from_xy(
            T::S::from(x) * self.scale + self.dx,
            T::S::from(y) * self.scale,
        )
    }

    fn add_vertex(&mut self, x: f32, y: f32) -> Option<T::E> {
        let pos = self.p(x, y);
        self.poly
            .append_point(T::Vec2::from_xy(T::S::from(x), T::S::from(y)));
        match self.cur_v {
            None => {
                // first vertex of ring
                let v = self.mesh.insert_vertex(T::VP::from_pos(pos)).id();
                self.first_v = Some((v, (x, y)));
                self.cur_v = Some(v);
                None
            }
            Some(cur) => {
                let (_, first_pos) = self.first_v.unwrap();
                if (x, y) == first_pos {
                    Some(self.finish_ring().unwrap())
                } else {
                    let (e, v) = self
                        .mesh
                        .insert_vertex_v(cur, T::VP::from_pos(pos), Default::default())
                        .expect("insert_vertex_v failed");
                    self.cur_v = Some(v);
                    Some(e)
                }
            }
        }
    }

    /// Close contour if still open.
    fn finish_ring(&mut self) -> Option<T::E> {
        let Some(cur) = self.cur_v else {
            // no current vertex, nothing to do
            return None;
        };
        let (first_v, (x, y)) = self.first_v.unwrap();
        if cur == first_v {
            // already closed
            return None;
        }
        let (e, f) = self
            .mesh
            .close_face_vv(cur, first_v, Default::default(), Default::default())
            .expect("close_face_vv failed");
        self.cur_v = Some(first_v);

        // move poly out of self and replace with empty poly
        let mut poly = T::Poly::from_points(&[T::Vec2::from_xy(T::S::from(x), T::S::from(y))]);
        std::mem::swap(&mut self.poly, &mut poly);
        self.assign_holes(f, poly);

        return Some(e);
    }

    fn assign_holes(&mut self, f: T::F, poly: T::Poly) -> usize {
        if poly.signed_area() < T::S::ZERO {
            // add a new hole
            for (f_face, comp) in &self.components {
                if comp.contains_polygon(&poly) {
                    let Some(e) = self.mesh.face(f).edge().id() else {
                        self.unassigned_holes.push((f, poly));
                        return self.unassigned_holes.len();
                    };
                    self.mesh.face_mut(f).remove().ensure_void();
                    self.mesh.face_mut(*f_face).add_quasi_island(e).ensure();
                    return self.unassigned_holes.len();
                }
            }
            self.unassigned_holes.push((f, poly));
        } else {
            // add a new component
            // PERF: avoid cloning
            self.unassigned_holes = self
                .unassigned_holes
                .iter()
                .filter(|(f_hole, hole)| {
                    if poly.contains_polygon(hole) {
                        let Some(e) = self.mesh.face(*f_hole).edge().id() else {
                            return false;
                        };
                        self.mesh.face_mut(*f_hole).remove().ensure_void();
                        self.mesh.face_mut(f).add_quasi_island(e).ensure();
                        return false;
                    }
                    true
                })
                .cloned()
                .collect::<Vec<_>>();
            self.components.push((f, poly));
        }

        return self.unassigned_holes.len();
    }

    fn is_done(&self) -> bool {
        return self.unassigned_holes.is_empty();
    }

    fn new_shape(&mut self) {
        self.first_v = None;
        self.cur_v = None;
        self.poly = T::Poly::empty();
    }
}

impl<'m, const D: usize, T> OutlineBuilder for MeshOutlineBuilder<'m, D, T>
where
    T: EuclideanMeshType<D> + MeshTypeHalfEdge,
    T::Edge: CurvedEdge<D, T> + HalfEdge<T>,
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
        self.finish_ring(); // new contour
        self.new_shape();
        self.add_vertex(x, y);
    }

    #[inline]
    fn line_to(&mut self, x: f32, y: f32) {
        self.add_vertex(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let e = self.add_vertex(x, y).unwrap();
        self.mesh
            .edge_ref(e)
            .clone()
            .set_curve_type_in_mesh(self.mesh, CurvedEdgeType::QuadraticBezier(self.p(x1, y1)));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let e = self.add_vertex(x, y).unwrap();
        self.mesh.edge_ref_mut(e).clone().set_curve_type_in_mesh(
            self.mesh,
            CurvedEdgeType::CubicBezier(self.p(x1, y1), self.p(x2, y2)),
        );
    }

    #[inline]
    fn close(&mut self) {
        self.finish_ring();
    }
}
