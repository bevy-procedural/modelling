use super::{Face, Mesh, Payload};
use crate::representation::IndexType;
use itertools::Itertools;

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Converts the face into a triangle fan. Only works for convex planar faces.
    pub fn fan_triangulation<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
    ) {
        assert!(self.may_be_curved() || self.is_planar2(mesh));
        assert!(self.is_convex(mesh));

        let center = self.vertices(mesh).next().unwrap();
        self.vertices(mesh)
            .skip(1)
            .tuple_windows::<(_, _)>()
            .for_each(|(a, b)| {
                indices.push(center.id());
                indices.push(a.id());
                indices.push(b.id());
            });
    }
}
