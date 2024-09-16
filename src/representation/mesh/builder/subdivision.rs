use crate::{
    math::{Scalar, Vector, Vector3D},
    representation::{
        payload::HasPosition, DefaultEdgePayload, DefaultFacePayload, HalfEdge, Mesh, MeshType,
        Vertex,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Subdivides by linear interpolation of the positions of the vertices.
    pub fn linear_subdivision_builder(
        n: usize,
        m: usize,
    ) -> impl Fn(&Mesh<T>, usize, T::V, usize, T::V, usize, T::V) -> T::VP {
        assert!(n == 1 && m == 0);
        move |mesh, i, vi, j, vj, k, vk| {
            let pi = *mesh.vertex(vi).pos();
            let pj = *mesh.vertex(vj).pos();
            let pk = *mesh.vertex(vk).pos();
            T::VP::from_pos(
                (pi * T::S::from_usize(i) + pj * T::S::from_usize(j) + pk * T::S::from_usize(k))
                    / T::S::from_usize(i + j + k),
            )
        }
    }
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Vec: Vector3D<S = T::S>,
{
    /// Subdivides by interpolating the angles of the positions assuming the
    /// center of the structure is `radius` away from the center in the opposite
    /// direction of the normal.
    pub fn spherical_subdivision_builder(
        center: T::Vec,
    ) -> impl Fn(&Mesh<T>, usize, T::V, usize, T::V, usize, T::V) -> T::VP {
        move |mesh, i, vi, j, vj, k, vk| {
            let mut pi = *mesh.vertex(vi).pos() - center;
            let scale = T::S::ONE / pi.length();
            pi = pi * scale;
            let pj = (*mesh.vertex(vj).pos() - center) * scale;
            let pk = (*mesh.vertex(vk).pos() - center) * scale;

            debug_assert!(pi.length() - T::S::ONE < T::S::EPS.sqrt());
            debug_assert!(pj.length() - T::S::ONE < T::S::EPS.sqrt());
            debug_assert!(pk.length() - T::S::ONE < T::S::EPS.sqrt());

            // slerp
            let pos = if i == 0 {
                pj.slerp(&pk, T::S::HALF)
            } else if j == 0 {
                pk.slerp(&pi, T::S::HALF)
            } else if k == 0 {
                pi.slerp(&pj, T::S::HALF)
            } else {
                todo!("slerp 3")
            };

            T::VP::from_pos(center + pos / scale)
        }
    }
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
{
    /// Will insert a new vertex inside this halfedge.
    /// After this, the mesh will be invalid since the twin is not updated!
    fn subdivide_unsafe(&mut self, e: T::E, vp: T::VP) -> T::E {
        let old_edge = self.edge(e).clone();

        let new_v = self.vertices.allocate();
        let new_edge = self.halfedges.allocate();

        self.halfedges.set(
            new_edge,
            HalfEdge::new(
                old_edge.next_id(),
                old_edge.twin_id(),
                old_edge.id(),
                new_v,
                old_edge.face_id(),
                Default::default(),
            ),
        );
        self.vertices.set(new_v, Vertex::new(new_edge, vp));

        self.edge_mut(old_edge.next_id()).set_prev(new_edge);
        self.edge_mut(old_edge.id()).set_next(new_edge);

        /*println!(
            "insert\nnew {:?}\nold {:?}",
            self.edge(new_edge),
            self.edge(old_edge.id())
        );*/

        new_edge
    }

    /// Call this on the twin of an halfedge where `subdivide_unsafe` was called
    /// and it will apply the same subdivision on this halfedge making the mesh valid again.
    /// Returns the id of the new edge. If the twin was not subdivided, it will return `None`.
    fn subdivide_unsafe_try_fixup(&mut self, e: T::E) -> Option<T::E> {
        let old_edge = self.edge(e).clone();
        let other_old = old_edge.twin(self);

        // find the "other_new". It has the characteristic property of sharing the same twin with the old edge.
        let mut other_new = other_old.next(self);
        let first_other_new_origin = other_new.origin_id();
        loop {
            if other_new.twin_id() == e {
                break;
            }
            other_new = other_new.twin(self).next(self);
            if other_new.origin_id() != first_other_new_origin {
                // Not a valid wheel
                return None;
            }
            if other_new.prev_id() == other_old.id() {
                // Went a full round
                return None;
            }
        }

        // Insert the new edge
        let new_edge = self.halfedges.allocate();
        self.halfedges.set(
            new_edge,
            HalfEdge::new(
                old_edge.next_id(),
                other_old.id(),
                old_edge.id(),
                other_new.origin_id(),
                old_edge.face_id(),
                Default::default(),
            ),
        );

        // update the neighbors
        self.edge_mut(old_edge.id()).set_twin(other_new.id());
        self.edge_mut(other_old.id()).set_twin(new_edge);
        self.edge_mut(old_edge.next_id()).set_prev(new_edge);
        self.edge_mut(old_edge.id()).set_next(new_edge);

        /*println!(
            "fixup\nnew {:?}\nold {:?}\non  {:?}\noo  {:?}",
            self.edge(new_edge),
            self.edge(old_edge.id()),
            self.edge(other_new.id()),
            self.edge(other_old.id())
        );*/

        Some(new_edge)
    }

    /// Subdivides the mesh with frequency (n,m).
    /// Uses the `vp_builder` to create the new vertex payloads.
    /// Returns a new mesh.
    pub fn subdivision_frequency_once(
        &mut self,
        vp_builder: &impl Fn(&Self, usize, T::V, usize, T::V, usize, T::V) -> T::VP,
    ) -> &mut Self {
        let fs = self.faces().map(|f| f.id()).collect::<Vec<_>>();
        for face in &fs {
            // get the edge chain
            let edges = self.face(*face).edges(self).collect::<Vec<_>>();
            let vs = edges.iter().map(|e| e.origin_id()).collect::<Vec<_>>();
            assert!(edges.len() == 3);

            // insert an additional vertex for each edge
            for i in 0..3 {
                if self.subdivide_unsafe_try_fixup(edges[i].id()).is_some() {
                    // edge is already subdivided
                    continue;
                }
                let vp = vp_builder(
                    self,
                    if vs[0] == edges[i].origin_id() || vs[0] == edges[i].target_id(self) {
                        1
                    } else {
                        0
                    },
                    vs[0],
                    if vs[1] == edges[i].origin_id() || vs[1] == edges[i].target_id(self) {
                        1
                    } else {
                        0
                    },
                    vs[1],
                    if vs[2] == edges[i].origin_id() || vs[2] == edges[i].target_id(self) {
                        1
                    } else {
                        0
                    },
                    vs[2],
                );
                self.subdivide_unsafe(edges[i].id(), vp);
            }

            // remove the original face
            let fp = self.remove_face(*face);

            // TODO: cannot clone fp like that!

            // insert the new edges and faces
            for e in &edges {
                self.insert_edge_no_check(
                    e.id(),
                    Default::default(),
                    self.edge(e.id()).prev(self).prev_id(),
                    Default::default(),
                );
                self.close_hole(e.id(), fp, false);
            }
            // fill the center hole
            self.close_hole(self.edge(edges[0].id()).next(self).twin_id(), fp, false);
        }

        self
    }

    /// Subdivides the mesh with frequency (n,m).
    /// Uses the `vp_builder` to create the new vertex payloads.
    /// Returns a new mesh.
    pub fn subdivision_frequency(
        &mut self,
        n: usize,
        m: usize,
        vp_builder: impl Fn(&Self, usize, T::V, usize, T::V, usize, T::V) -> T::VP,
    ) -> &mut Self {
        // for now
        assert!(m == 0);
        assert!(n & (n - 1) == 0, "todo: odd subdivision frequency");

        let mut o = n;
        while o > 0 {
            self.subdivision_frequency_once(&vp_builder);
            if o == 1 {
                break;
            }
            assert!(o % 2 == 0, "todo: odd subdivision frequency");
            o = o / 2
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::representation::{bevy::BevyMesh3d, payload::bevy::BevyVertexPayload};

    #[test]
    fn subdivide_and_fixup() {
        let mut mesh = BevyMesh3d::regular_polygon(1.0, 3);

        let e = mesh.edge(0).clone();
        let vp = BevyVertexPayload::from_pos(
            *e.origin(&mesh).pos() * 0.5 + *e.origin(&mesh).pos() * 0.5,
        );

        mesh.subdivide_unsafe(e.id(), vp);
        assert!(mesh.subdivide_unsafe_try_fixup(e.twin_id()).is_some());

        println!("mesh: {}", mesh);

        assert!(mesh.check().is_ok());
    }
}
