use crate::mesh::{
    DefaultEdgePayload, EdgeBasics, FaceBasics, HalfEdge, HalfEdgeMeshType, HalfEdgeSemiBuilder,
    VertexInterpolator,
};

/// Describes how to subdivide a mesh.
#[derive(Debug, Clone, Copy)]
pub struct SubdivisionDescription {
    b: usize,
    c: usize,
}

impl SubdivisionDescription {
    /// Create a new subdivision description.
    pub fn new(b: usize, c: usize) -> Self {
        assert!(b >= 1);
        Self { b, c }
    }

    /// Get the first number of subdivisions.
    pub fn b(&self) -> usize {
        self.b
    }

    /// Get the second number of subdivisions.
    pub fn c(&self) -> usize {
        self.c
    }

    /// Frequency $v = b + c$ of the subdivision.
    pub fn frequency(&self) -> usize {
        self.b + self.c
    }

    /// Triangulation number $T = b^2 + bc + c^2$ of the subdivision.
    pub fn triangulation_number(&self) -> usize {
        self.b * self.b + self.b * self.c + self.c * self.c
    }
}

/// TODO

/// A trait for subdividing meshes.
pub trait MeshSubdivision<T: HalfEdgeMeshType<Mesh = Self>>: HalfEdgeSemiBuilder<T>
where
    T::EP: DefaultEdgePayload,
{
    /// Subdivides the mesh with frequency (2,0).
    /// Uses the `vp_builder` to create the new vertex payloads.
    /// Returns a new mesh.
    ///
    /// based on an algorithm developed by Charles Loop in 1987
    fn loop_subdivision(&mut self, vp_builder: &impl VertexInterpolator<3, T>) -> &mut Self {
        // TODO: See https://github.com/OptimisticPeach/hexasphere
        let fs = self.faces().map(|f| f.id()).collect::<Vec<_>>();
        for face in &fs {
            // get the edge chain
            let edges = self.face(*face).edges(self).collect::<Vec<_>>();
            let vs = edges.iter().map(|e| e.origin_id()).collect::<Vec<_>>();
            assert!(edges.len() == 3);

            // insert an additional vertex for each edge
            for i in 0..3 {
                if self
                    .subdivide_unsafe_try_fixup(edges[i].id(), Default::default())
                    .is_some()
                {
                    // edge is already subdivided
                    continue;
                }
                let vp = vp_builder.call(
                    self,
                    [
                        (
                            if vs[0] == edges[i].origin_id() || vs[0] == edges[i].target_id(self) {
                                1
                            } else {
                                0
                            },
                            vs[0],
                        ),
                        (
                            if vs[1] == edges[i].origin_id() || vs[1] == edges[i].target_id(self) {
                                1
                            } else {
                                0
                            },
                            vs[1],
                        ),
                        (
                            if vs[2] == edges[i].origin_id() || vs[2] == edges[i].target_id(self) {
                                1
                            } else {
                                0
                            },
                            vs[2],
                        ),
                    ],
                );
                self.subdivide_unsafe(edges[i].id(), vp, Default::default());
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
    fn subdivision_frequency(
        &mut self,
        des: SubdivisionDescription,
        vp_builder: impl VertexInterpolator<3, T>,
    ) -> &mut Self {
        // TODO: for c != 0 we have to shift the triangle. This means we have to build a completely new graph and things become much more complicated
        assert!(des.c == 0);

        // TODO: Apply this to meshes with non-triangular faces by triangulating them. Usually, you want to insert Center points / Steiner points to get nearly equilateral triangles.

        assert!(des.b & (des.b - 1) == 0, "todo: odd subdivision frequency");
        let num_faces = self.num_faces();

        let mut b = des.b;
        while b > 1 {
            self.loop_subdivision(&vp_builder);
            if b == 1 {
                break;
            }
            assert!(b % 2 == 0, "todo: odd subdivision frequency");
            b = b / 2
        }

        debug_assert_eq!(self.num_faces(), num_faces * des.triangulation_number());

        self
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::{
        bevy::BevyMesh3d,
        payload::{vertex_payload::BevyVertexPayload, HasPosition},
    };

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
*/
