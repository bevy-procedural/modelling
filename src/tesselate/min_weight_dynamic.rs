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
    m[(0, n - 1)] = Vec2::S::ZERO;

    let mut evaluated = 0;
    for l in 2..n {
        for i in 0..(n - l) {
            let j = i + l;

            let ij = m.index(i, j);
            m[ij] = Vec2::S::INFINITY;
            for k in (i + 1)..j {
                debug_assert!(i < k && k < j);
                let ik = m.index(i, k);
                let kj = m.index(k, j);

                if !valid_diagonal[ik] || !valid_diagonal[kj] {
                    continue;
                }

                evaluated += 1;
                let weight = triangle_weight(&vs[i].vec, &vs[j].vec, &vs[k].vec);

                if weight >= m[ij] {
                    //println!("stop early");
                    continue;
                }
                let cost = m[ik] + m[kj] + weight;
                if cost < m[ij] {
                    m[ij] = cost;
                    s[ij] = k;
                }
            }
        }
    }

    let ela = now.elapsed();
    println!("Dynamic programming: {:?} eval: {}", ela, evaluated);

    traceback(n, 0, n - 1, &s, indices, &vs);
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        bevy::{Bevy2DPolygon, BevyMesh3d, BevyMeshType3d32},
        mesh::MeshBasics,
        primitives::Make2dShape,
        tesselate::{triangulate_face, TesselationMeta, TriangulationAlgorithm},
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
        assert!(false, "Edge w: {}", w);
    }
}
