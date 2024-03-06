use super::{Face, Mesh, Payload};
use crate::{
    math::{Vector, Vector3D},
    representation::IndexType,
};
use itertools::Itertools;

mod convex;
mod delaunay;
mod ear;
mod min_weight;
mod sweep;

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Converts the face into a triangle list
    pub fn tesselate<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>, indices: &mut Vec<V>)
    where
        P::Vec: Vector3D<P::S>,
    {
        let mut local_indices = Vec::new();
        //self.ear_clipping(mesh, &mut local_indices);
        self.min_weight_triangulation_stoch(mesh, &mut local_indices);
        //self.delaunayfy(mesh, &mut local_indices);
        indices.extend(local_indices);
        assert!(indices.len() % 3 == 0, "{:?}", indices.len());
        assert!(indices.iter().all(|i| i.index() < mesh.max_vertex_index()));

        // Minimize edge length
        // TODO: https://en.wikipedia.org/wiki/Minimum-weight_triangulation#Variations
    }

    /// Converts the face into a triangle list and duplicates vertices.
    pub fn tesselate_flat_normal<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        vertices: &mut Vec<P>,
        indices: &mut Vec<V>,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        let v0 = vertices.len();
        let normal = self.normal(mesh);
        self.vertices(mesh).for_each(|v| {
            let mut p = v.payload().clone();
            p.set_normal(normal);
            vertices.push(p)
        });
        let mut local_indices = Vec::new();
        self.ear_clipping(mesh, &mut local_indices, true);
        indices.extend(local_indices.iter().map(|i| V::new(v0 + i.index())));
    }

    /// Converts the face into a triangle list and duplicates vertices.
    pub fn tesselate_smooth_normal<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        vertices: &mut Vec<P>,
        indices: &mut Vec<V>,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        let v0 = vertices.len();
        let normal = self.normal(mesh);
        self.vertices(mesh)
            .circular_tuple_windows::<(_, _, _)>()
            .for_each(|(prev, v, next)| {
                let mut p = v.payload().clone();
                let mut no = v.vertex().normal(*prev.vertex(), *next.vertex());
                if no.dot(&normal) < 0.0.into() {
                    no = -no;
                }
                p.set_normal(no);
                vertices.push(p)
            });
        let mut local_indices = Vec::new();
        self.ear_clipping(mesh, &mut local_indices, true);
        let n = self.num_vertices(mesh);
        indices.extend(
            local_indices
                .iter()
                .map(|i| V::new(v0 + ((i.index() + n - 1) % n))),
        );
    }
}
