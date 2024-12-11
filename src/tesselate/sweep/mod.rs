mod chain;
mod interval;
mod monotone;
mod point;
mod status;
mod sweep;
mod vertex_type;

pub use monotone::*;
pub use sweep::sweep_line_triangulation;
pub use vertex_type::VertexType;

use super::TesselationMeta;
use crate::{
    math::IndexType,
    mesh::{Face3d, FaceBasics, IndexedVertex2D, MeshType3D, Triangulation},
};

/// Meta information for debuggin the sweep algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct SweepMeta<V: IndexType> {
    #[cfg(feature = "sweep_debug")]
    /// The type of the vertex in the reflex chain
    pub vertex_type: Vec<(V, VertexType)>,

    phantom: std::marker::PhantomData<V>,
}

impl<V: IndexType> Default for SweepMeta<V> {
    fn default() -> Self {
        SweepMeta {
            #[cfg(feature = "sweep_debug")]
            vertex_type: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<V: IndexType> SweepMeta<V> {
    /// Update the type of a vertex
    #[cfg(feature = "sweep_debug")]
    pub fn update_type(&mut self, i: V, t: VertexType) {
        // TODO: Not efficient
        for (j, ty) in self.vertex_type.iter_mut() {
            if *j == i {
                *ty = t;
            }
        }
    }
}

/// Uses the sweep line triangulation
pub fn sweep_line<T: MeshType3D, Tri: MonotoneTriangulator<V = T::V, Vec2 = T::Vec2>>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
    meta: &mut TesselationMeta<T::V>,
) {
    debug_assert!(face.may_be_curved() || face.is_planar2(mesh));

    // TODO: Improve performance by directly using the nd-vertices instead of converting to 2d
    let vec2s: Vec<_> = face
        .vertices_2d(mesh)
        .map(|(p, i)| IndexedVertex2D::<T::V, T::Vec2>::new(p, i))
        .collect();

    sweep_line_triangulation::<Tri>(indices, &vec2s, &mut meta.sweep);
}

#[cfg(test)]
mod tests {

    use crate::{prelude::*, tesselate::sweep::LinearMonoTriangulator};

    fn verify_triangulation<T: MeshType3D>(mesh: &T::Mesh, f: T::F) {
        let face = mesh.face(f);
        let vec2s = face.vec2s(mesh);
        assert!(
            T::Poly::from_iter(vec2s.iter().map(|v| v.vec)).is_ccw(),
            "Polygon must be counterclockwise"
        );
        let mut indices = Vec::new();
        let mut tri = Triangulation::new(&mut indices);
        let mut meta = TesselationMeta::default();
        sweep_line::<T, LinearMonoTriangulator<T::V, T::Vec2>>(face, &mesh, &mut tri, &mut meta);
        tri.verify_full::<T::Vec2, T::Poly>(&vec2s);
    }

    /*
    #[test]
    #[cfg(feature = "nalgebra")]
    fn test_font() {
        use crate::extensions::nalgebra::*;

        let mut mesh2d = Mesh2d64Curved::new();
        Font::new(include_bytes!("../../../assets/Cochineal-Roman.otf"), 1.0)
            .layout_text::<2, MeshType2d64PNUCurved>("F", &mut mesh2d);
        self::verify_triangulation::<MeshType3d64PNU>(&mesh2d.to_nd(0.01), 0);
    }*/
}
