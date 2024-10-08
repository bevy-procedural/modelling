use super::{Face, Mesh, Payload, Triangulation};
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
        indices: &mut Triangulation<V>,
        randomize: bool,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        let eps = <P::S as Scalar>::EPS * 2.0.into();
        let mut success_since_fail = 0;
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
                || (success_since_fail >= 2 &&vs[i_b].0.collinear(vs[i_a].0, vs[i_c].0, eps))
            {
                fails_since_advance += 1;
                if fails_since_advance > n {
                    // If there are nearly collinear points, triangle_empty might not work correctly
                    if success_since_fail < 2 {
                        panic!("Ear-clipping failed to find a valid triangle due to nearly collinear points");
                    }
                    success_since_fail = 0;
                } else {
                    i_a = i_b;
                    continue;
                }
            }

            indices.insert_triangle(vs[i_a].1, vs[i_b].1, vs[i_c].1);
            clipped[i_b] = true;
            n -= 1;
            fails_since_advance = 0;
            success_since_fail += 1;

            if randomize {
                i_a = rand::random::<usize>() % n0;
                while clipped[i_a] {
                    i_a = (i_a + 1) % n0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{impls::bevy::Bevy2DPolygon, Polygon, Scalar},
        representation::{
            payload::bevy::BevyPayload, primitives::random_star, tesselate::IndexedVertex2D,
        },
    };
    use bevy::math::{Vec2, Vec3};

    fn verify_triangulation(vec2s: &Vec<IndexedVertex2D<usize, Vec2>>) {
        assert!(
            Bevy2DPolygon::from_iter(vec2s.iter().map(|v| v.vec)).is_ccw(),
            "Polygon must be counterclockwise"
        );
        let mut indices = Vec::new();
        let mut tri = Triangulation::new(&mut indices);
        let m = Mesh::<usize, usize, usize, BevyPayload>::polygon(
            &vec2s
                .iter()
                .map(|v| Vec3::new(v.vec.x, 0.0, v.vec.y))
                .collect::<Vec<_>>(),
        );
        m.face(0).ear_clipping(&m, &mut tri, false);
        tri.verify_full::<Vec2, Bevy2DPolygon>(vec2s);
    }

    fn liv_from_array(arr: &[[f32; 2]]) -> Vec<IndexedVertex2D<usize, Vec2>> {
        arr.iter()
            .enumerate()
            .map(|(i, &v)| IndexedVertex2D::new(Vec2::from_xy(v[0], v[1]), i))
            .collect()
    }

    #[test]
    fn ears_triangle() {
        verify_triangulation(&liv_from_array(&[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]));
    }

    #[test]
    fn ears_circle() {
        let n = 100; // 10000 is more interesting, but runs 5 to 10 seconds
        verify_triangulation(
            &(0..n)
                .into_iter()
                .map(|i| {
                    let a = i as f32 / (n as f32) * std::f32::consts::PI * 2.0;
                    IndexedVertex2D::new(Vec2::from_xy(a.cos(), a.sin()), i)
                })
                .collect(),
        );
    }

    #[test]
    fn numerical_hell_1() {
        verify_triangulation(&liv_from_array(&[
            [2.001453, 0.0],
            [0.7763586, 2.3893864],
            [-3.2887821, 2.3894396],
            [-2.7725635, -2.0143867],
            [0.023867942, -0.07345794],
        ]));
    }

    /// This is effective to find special examples where the triangulation fails
    /// You might want to increase the number of iterations to >= 1000000 and adjust
    /// the random_star parameters to find nastier examples
    #[test]
    fn earcut_fuzz() {
        for _ in 1..10 {
            let vec2s = IndexedVertex2D::from_vector(random_star::<Vec2>(5, 20, f32::EPS, 0.01));

            println!(
                "vec2s: {:?}",
                vec2s.iter().map(|v| [v.vec.x, v.vec.y]).collect::<Vec<_>>()
            );

            verify_triangulation(&vec2s);
        }
    }
}
