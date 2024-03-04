use itertools::Itertools;

use super::{Face, Mesh, Payload, Scalar};
use crate::representation::{
    payload::{Vector, Vector2D, Vector3D},
    IndexType,
};

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
        assert!(self.is_planar(mesh, P::S::EPS * 10.0.into()));
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

    /// Use ear-clipping to triangulate the face.
    pub fn ear_clipping<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        assert!(self.is_planar(mesh, P::S::EPS * 10.0.into()));

        let vs: Vec<(<P::Vec as Vector<P::S>>::Vec2D, V)> =
            self.vertices_2d::<V, P>(mesh).collect();

        let triangle_empty = |a: usize, b: usize, c: usize| {
            let av = vs[a].0;
            let bv = vs[b].0;
            let cv = vs[c].0;
            vs.iter()
                .enumerate()
                .all(|(i, v)| i == a || i == b || i == c || !v.0.is_inside_triangle(av, bv, cv))
        };

        let n0 = vs.len();
        if n0 < 3 {
            return;
        }
        let mut clipped = vec![false; n0];
        let mut i_a = 0;
        let mut n = n0;
        let mut fails_since_advance = 0;
        while n > 2 {
            let mut i_b = (i_a + 1) % n0;
            while clipped[i_b] {
                i_b = (i_b + 1) % n0;
            }
            let mut i_c = (i_b + 1) % n0;
            while clipped[i_c] {
                i_c = (i_c + 1) % n0;
            }

            // println!("i_a: {}, i_b: {}, i_c: {} {:?}", i_a, i_b, i_c, clipped);
            assert!(i_a != i_b);
            assert!(i_b != i_c);
            assert!(i_c != i_a);

            // cut the ear off
            if !vs[i_b].0.convex(vs[i_a].0, vs[i_c].0) || !triangle_empty(i_a, i_b, i_c) {
                fails_since_advance += 1;
                if fails_since_advance > n {
                    // TODO:
                    // panic!("Failed to advance");
                    println!("Failed to advance {:?} {} {} {}", vs, i_a, i_b, i_c);
                    break;
                }
                i_a = i_b;
                continue;
            }

            indices.push(vs[i_a].1);
            indices.push(vs[i_b].1);
            indices.push(vs[i_c].1);
            clipped[i_b] = true;
            n -= 1;
            fails_since_advance = 0;
        }
    }

    /*
    /// Converts the face into a triangle list
    pub fn tesselate<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
    ) {
        self.fan_triangulation(mesh, indices);
    }*/

    /// Converts the face into a triangle list
    pub fn tesselate<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>, indices: &mut Vec<V>)
    where
        P::Vec: Vector3D<P::S>,
    {
        self.ear_clipping(mesh, indices);
    }

    /*/// Converts the face into a triangle list using the delaunay triangulation.
    pub fn delaunay_triangulation<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        _indices: &mut Vec<V>,
    ) {
        assert!(self.is_planar(mesh, P::S::EPS * 10.0.into()));



        // TODO: or at least some other O(n log n) algorithm: https://en.wikipedia.org/wiki/Delaunay_triangulation
    }*/
}
