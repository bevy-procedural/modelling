use crate::{
    math::{HasPosition, HasZero, IndexType, Polygon, Scalar, Vector2D, Vector3D},
    mesh::{Face3d, FaceBasics, IndexedVertex2D, MeshType, Triangulation},
};

/// The [min-weight triangulation problem](https://en.wikipedia.org/wiki/Minimum-weight_triangulation)
/// is, in general, NP-hard. However, for polygons without interior points we can
/// achieve it in O(n^3) time using dynamic programming.
pub fn minweight_dynamic<T: MeshType>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    debug_assert!(face.may_be_curved() || face.is_planar2(mesh));

    // TODO: Improve performance by directly using the nd-vertices instead of converting to 2d
    let vec2s: Vec<_> = face
        .vertices_2d(mesh)
        .map(|(p, i)| IndexedVertex2D::<T::V, T::Vec2>::new(p, i))
        .collect();

    minweight_dynamic_direct::<T::V, T::Vec2, T::Poly>(&vec2s, indices);
}

/// Like `minweight_dynamic`, but directly on a vertex list instead of a mesh face.
pub fn minweight_dynamic_direct<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    indices: &mut Triangulation<V>,
) {
    let n = vs.len();
    let mut m = vec![vec![Vec2::S::ZERO; n]; n];
    let mut s = vec![vec![0; n]; n];

    let mut valid_diagonal = vec![true; n * n];
    let poly = Poly::from_iter(vs.iter().map(|v| v.vec));
    for i in 0..n {
        for j in (i + 2)..n {
            let res = poly.valid_diagonal(i, j);
            valid_diagonal[i * n + j] = res;
        }
    }

    for l in 2..n {
        for i in 0..(n - l) {
            let j = i + l;
            m[i][j] = Vec2::S::INFINITY;
            for k in (i + 1)..j {
                assert!(i < k && k < j);

                if !valid_diagonal[i * n + k] || !valid_diagonal[k * n + j]
                // No need to check when the diagonals or both ok
                //|| !is_valid_triangle(i, j, k, &vs)
                {
                    continue;
                }

                let weight = triangle_weight(&vs[i].vec, &vs[j].vec, &vs[k].vec);
                let cost = m[i][k] + m[k][j] + weight;
                if cost < m[i][j] {
                    m[i][j] = cost;
                    s[i][j] = k;
                }
            }
        }
    }

    traceback(0, n - 1, &s, indices, &vs);
}

fn triangle_weight<Vec2: Vector2D>(a: &Vec2, b: &Vec2, c: &Vec2) -> Vec2::S {
    let ab = a.distance(b);
    let bc = b.distance(c);
    let ca = c.distance(a);
    ab + bc + ca
}

fn traceback<V: IndexType, Vec2: Vector2D>(
    i: usize,
    j: usize,
    s: &Vec<Vec<usize>>,
    indices: &mut Triangulation<V>,
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
) {
    if j - i < 2 {
        return;
    }
    let k = s[i][j];
    // Add triangle (vi, vk, vj)
    indices.insert_triangle_local(i, k, j, vs);
    // Recurse on subpolygons
    traceback(i, k, s, indices, vs);
    traceback(k, j, s, indices, vs);
}

/*
fn is_valid_triangle<V: IndexType, Vec2: Vector2D>(
    i: usize,
    j: usize,
    k: usize,
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
) -> bool {
    // Check if triangle (v_i, v_j, v_k) is valid
    // Ensure no other vertex lies inside the triangle
    for m in (i + 1)..j {
        if m == k {
            continue;
        }
        if point_in_triangle(&vs[m].vec, &vs[i].vec, &vs[j].vec, &vs[k].vec) {
            return false;
        }
    }
    true
}

fn point_in_triangle<Vec2: Vector2D>(p: &Vec2, a: &Vec2, b: &Vec2, c: &Vec2) -> bool {
    let area = Vec2::S::HALF
        * (-b.y() * c.x() + a.y() * (-b.x() + c.x()) + a.x() * (b.y() - c.y()) + b.x() * c.y());
    let s = Vec2::S::ONE / (Vec2::S::TWO * area)
        * (a.y() * c.x() - a.x() * c.y() + (c.y() - a.y()) * p.x() + (a.x() - c.x()) * p.y());
    let t = Vec2::S::ONE / (Vec2::S::TWO * area)
        * (a.x() * b.y() - a.y() * b.x() + (a.y() - b.y()) * p.x() + (b.x() - a.x()) * p.y());
    s > Vec2::S::ZERO && t > Vec2::S::ZERO && Vec2::S::ONE - s - t > Vec2::S::ZERO
}
*/
