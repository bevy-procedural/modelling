use super::{Face, Mesh, Payload};
use crate::{
    math::{Vector2D, Vector3D},
    representation::IndexType,
};

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Use ear-clipping to triangulate the face.
    /// This is relatively slow: O(n^2).
    pub fn ear_clipping<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        local_coordinates: bool,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        assert!(self.may_be_curved() || self.is_planar2(mesh));
        assert!(self.is_simple(mesh));

        let vs: Vec<(P::Vec2, V)> = self.vertices_2d::<V, P>(mesh).collect();

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
                    // TODO: don't panic; this could happen due to float inaccuracies
                    panic!("Failed to advance {:?} {} {} {}", vs, i_a, i_b, i_c);
                }
                i_a = i_b;
                continue;
            }

            if local_coordinates {
                indices.push(V::new(i_a));
                indices.push(V::new(i_b));
                indices.push(V::new(i_c));
            } else {
                indices.push(vs[i_a].1);
                indices.push(vs[i_b].1);
                indices.push(vs[i_c].1);
            }
            clipped[i_b] = true;
            n -= 1;
            fails_since_advance = 0;
        }
    }

    /// Ear clipping, but randomly choose the start for the search of the next ear.
    /// This is even slower than the normal ears, but it can be used to generate different triangulations.
    pub fn ear_clipping_randomized<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        // TODO: ear clipping is inefficient
        assert!(self.may_be_curved() || self.is_planar2(mesh));

        let vs: Vec<(P::Vec2, V)> = self.vertices_2d::<V, P>(mesh).collect();

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
        let mut i_a = rand::random::<usize>() % n0;
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
                    //println!("Failed to advance {:?} {} {} {}", vs, i_a, i_b, i_c);
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
            i_a = rand::random::<usize>() % n0;
            while clipped[i_a] {
                i_a = (i_a + 1) % n0;
            }
        }
    }
}
