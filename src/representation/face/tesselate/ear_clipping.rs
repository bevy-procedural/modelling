use super::{Face, Mesh, Payload};
use crate::{
    math::{Scalar, Vector2D, Vector3D},
    representation::IndexType,
};

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Use ear-clipping to triangulate the face.
    /// This is relatively slow: O(n^2).
    ///
    /// Optionally randomize the start position to search the next ear.
    /// This is slightly slower but can generate more versatile results.
    pub fn ear_clipping<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        local_indices: bool,
        randomize: bool,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        let eps = <P::S as Scalar>::EPS * 2.0.into();
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));
        debug_assert!(self.is_simple(mesh));

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
        if randomize {
            i_a = rand::random::<usize>() % n0;
        }
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

            debug_assert!(i_a != i_b);
            debug_assert!(i_b != i_c);
            debug_assert!(i_c != i_a);

            // cut the ear off
            if !vs[i_b].0.convex(vs[i_a].0, vs[i_c].0)
                || !triangle_empty(i_a, i_b, i_c)
                // if there are nearly collinear points, we can't cut the ear, because triangle_empty could block any progress afterwards
                || vs[i_b].0.collinear(vs[i_a].0, vs[i_c].0, eps)
            {
                fails_since_advance += 1;
                if fails_since_advance > n {
                    // TODO: don't panic; this could happen due to float inaccuracies
                    println!("⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️");
                    println!(
                        "Failed to advance {} {} {}",
                        vs[i_a].1, vs[i_b].1, vs[i_c].1
                    );
                    println!(
                        "clipped: {:?}",
                        clipped
                            .iter()
                            .enumerate()
                            .filter_map(|(i, &c)| if c { Some(i) } else { None })
                            .collect::<Vec<_>>()
                    );
                    break;
                }
                i_a = i_b;
                continue;
            }

            if local_indices {
                indices.extend([V::new(i_a), V::new(i_b), V::new(i_c)]);
            } else {
                indices.extend([vs[i_a].1, vs[i_b].1, vs[i_c].1]);
            }
            clipped[i_b] = true;
            n -= 1;
            fails_since_advance = 0;
            if randomize {
                i_a = rand::random::<usize>() % n0;
                while clipped[i_a] {
                    i_a = (i_a + 1) % n0;
                }
            }
        }
    }
}
