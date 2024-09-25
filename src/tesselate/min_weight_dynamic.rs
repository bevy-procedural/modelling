use std::time::Instant;

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

struct TriangularStore<T: Clone> {
    data: Vec<T>,
    n: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct TriangularStoreIndex(usize);

impl<T: Clone> TriangularStore<T> {
    fn new(n: usize, default: T) -> Self {
        assert!(n > 1);
        Self {
            data: vec![default; (n * (n - 1)) / 2],
            n,
        }
    }

    #[inline(always)]
    fn index(&self, i: usize, j: usize) -> TriangularStoreIndex {
        debug_assert!(j >= 1);
        debug_assert!(i < j);
        debug_assert!(j < self.n);
        TriangularStoreIndex {
            0: ((j * (j - 1)) / 2 + i),
        }
    }
}

impl<T: Clone> std::ops::Index<(usize, usize)> for TriangularStore<T> {
    type Output = T;
    #[inline(always)]
    fn index<'a>(&'a self, (i, j): (usize, usize)) -> &'a T {
        &self[self.index(i, j)]
    }
}

impl<T: Clone> std::ops::IndexMut<(usize, usize)> for TriangularStore<T> {
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, (i, j): (usize, usize)) -> &'a mut T {
        let k = self.index(i, j);
        &mut self[k]
    }
}

impl<T: Clone> std::ops::Index<TriangularStoreIndex> for TriangularStore<T> {
    type Output = T;
    #[inline(always)]
    fn index<'a>(&'a self, i: TriangularStoreIndex) -> &'a T {
        &self.data[i.0]
    }
}

impl<T: Clone> std::ops::IndexMut<TriangularStoreIndex> for TriangularStore<T> {
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, i: TriangularStoreIndex) -> &'a mut T {
        &mut self.data[i.0]
    }
}

/// Like `minweight_dynamic`, but directly on a vertex list instead of a mesh face.
pub fn minweight_dynamic_direct<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    indices: &mut Triangulation<V>,
) {
    let n = vs.len();
    assert!(n >= 5);
    let mut m = TriangularStore::<Vec2::S>::new(n, Vec2::S::INFINITY);
    let mut s = TriangularStore::<usize>::new(n, IndexType::max());

    let now = Instant::now();

    let mut valid_diagonal = TriangularStore::<bool>::new(n, true);
    let poly = Poly::from_iter(vs.iter().map(|v| v.vec));
    for i in 0..n {
        for j in (i + 2)..n {
            let res = poly.valid_diagonal(i, j);
            valid_diagonal[(i, j)] = res;
        }
    }

    println!("Valid diagonals: {:?}", now.elapsed());
    let now = Instant::now();

    for i in 0..n - 1 {
        // pairs of vertices have no edge length
        m[(i, i + 1)] = Vec2::S::ZERO;
    }

    let mut evaluated = 0;
    for l in 2..n {
        for i in 0..(n - l) {
            let j = i + l;

            let ij = m.index(i, j);
            for k in (i + 1)..j {
                debug_assert!(i < k && k < j);
                let ik = m.index(i, k);
                let kj = m.index(k, j);

                if !valid_diagonal[ik] || !valid_diagonal[kj] {
                    continue;
                }

                let weight = triangle_weight(&vs[i].vec, &vs[j].vec, &vs[k].vec);

                /*if m[ik] + weight >= m[ij] {
                    continue;
                }
                evaluated += 1;*/

                let cost = m[ik] + m[kj] + weight;
                if cost < m[ij] {
                    m[ij] = cost;
                    s[ij] = k;
                }
            }
        }
    }

    let ela = now.elapsed();
    println!(
        "Dynamic programming: {:?} eval: {}%",
        ela,
        evaluated as f32 / binomial(n as u32, 3) as f32 * 100.0
    );

    traceback(n, 0, n - 1, &s, indices, &vs);
}

fn binomial(n: u32, k: u32) -> u32 {
    if k > n {
        return 0;
    }
    let mut res = 1;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }
    res
}

#[inline(always)]
fn triangle_weight<Vec2: Vector2D>(a: &Vec2, b: &Vec2, c: &Vec2) -> Vec2::S {
    a.distance(b) + b.distance(c) + c.distance(a)
}

fn traceback<V: IndexType, Vec2: Vector2D>(
    n: usize,
    i: usize,
    j: usize,
    s: &TriangularStore<usize>,
    indices: &mut Triangulation<V>,
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
) {
    if j - i < 2 {
        return;
    }
    let k = s[(i, j)];
    debug_assert!(i < k && k < j, "ikj {} {} {}", i, k, j);

    // Add triangle (vi, vk, vj)
    indices.insert_triangle_local(i, k, j, vs);

    // Recurse on subpolygons
    traceback(n, i, k, s, indices, vs);
    traceback(n, k, j, s, indices, vs);
}

fn minweight_dynamic_direct_naive<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    indices: &mut Triangulation<V>,
) {
    let n = vs.len();
    assert!(n >= 5);
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

                if !valid_diagonal[i * n + k] || !valid_diagonal[k * n + j] {
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
    traceback_naive(0, n - 1, &s, indices, &vs);
}

fn traceback_naive<V: IndexType, Vec2: Vector2D>(
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
    traceback_naive(i, k, s, indices, vs);
    traceback_naive(k, j, s, indices, vs);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        bevy::{Bevy2DPolygon, BevyMesh3d, BevyMeshType3d32},
        mesh::MeshBasics,
        primitives::Make2dShape,
        tesselate::{
            delaunay_triangulation, triangulate_face, TesselationMeta, TriangulationAlgorithm,
        },
    };
    use bevy::math::Vec2;

    #[test]
    fn test_minweight_dynamic_performance() {
        let poly = BevyMesh3d::regular_polygon(1.0, 1000);
        let mut meta = TesselationMeta::default();
        let mut indices = Vec::new();
        let mut tri = Triangulation::new(&mut indices);
        triangulate_face::<BevyMeshType3d32>(
            poly.face(0),
            &poly,
            &mut tri,
            TriangulationAlgorithm::SweepDynamic,
            &mut meta,
        );

        let vec2s = poly.face(0).vec2s(&poly);
        let vec_hm: HashMap<u32, Vec2> = vec2s.iter().map(|v| (v.index, v.vec)).collect();
        tri.verify_full::<Vec2, Bevy2DPolygon>(&vec2s);
        let w = tri.total_edge_weight(&vec_hm);

        let mut indices2 = Vec::new();
        let mut tri2 = Triangulation::new(&mut indices2);
        minweight_dynamic_direct_naive::<u32, Vec2, Bevy2DPolygon>(&vec2s, &mut tri2);
        tri2.verify_full::<Vec2, Bevy2DPolygon>(&vec2s);
        let w2 = tri2.total_edge_weight(&vec_hm);

        let mut indices3 = Vec::new();
        let mut tri3 = Triangulation::new(&mut indices3);
        delaunay_triangulation::<BevyMeshType3d32>(poly.face(0), &poly, &mut tri3);
        let w3 = tri3.total_edge_weight(&vec_hm);

        assert!(false, "Edge w: {} w2: {} w3: {}", w, w2, w3);
    }
}
