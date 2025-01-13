/// TODO: Min-weight triangulation is very much work in progress
use std::time::Instant;

use crate::{
    math::{IndexType, Polygon, Scalar, Vector2D},
    mesh::{Face3d, IndexedVertex2D, MeshType3D, Triangulation},
    tesselate::try_min_weight_small,
};

/// The [min-weight triangulation problem](https://en.wikipedia.org/wiki/Minimum-weight_triangulation)
/// is, in general, NP-hard. However, for polygons without interior points we can
/// achieve it in O(n^3) time using dynamic programming.
pub fn minweight_dynamic<T: MeshType3D>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
) {
    // TODO: Ignore non-planar faces for now
    //debug_assert!(face.may_be_curved() || face.is_planar2(mesh));

    if try_min_weight_small::<T>(face, mesh, indices) {
        return;
    }

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

    #[inline]
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
    #[inline]
    fn index<'a>(&'a self, (i, j): (usize, usize)) -> &'a T {
        &self[self.index(i, j)]
    }
}

impl<T: Clone> std::ops::IndexMut<(usize, usize)> for TriangularStore<T> {
    #[inline]
    fn index_mut<'a>(&'a mut self, (i, j): (usize, usize)) -> &'a mut T {
        let k = self.index(i, j);
        &mut self[k]
    }
}

impl<T: Clone> std::ops::Index<TriangularStoreIndex> for TriangularStore<T> {
    type Output = T;
    #[inline]
    fn index<'a>(&'a self, i: TriangularStoreIndex) -> &'a T {
        &self.data[i.0]
    }
}

impl<T: Clone> std::ops::IndexMut<TriangularStoreIndex> for TriangularStore<T> {
    #[inline]
    fn index_mut<'a>(&'a mut self, i: TriangularStoreIndex) -> &'a mut T {
        &mut self.data[i.0]
    }
}

fn calculate_lower_bound<S: Scalar>(
    n: usize,
    pre_rendered: usize,
    m: &TriangularStore<S>,
) -> Vec<S> {
    // The lower bound for the min-weight cost of a polygon with n+1 vertices
    // we don't have to include =pre_rendered in this loop, since we will cover this
    // using k_smallest!
    let mut lower_bound = vec![S::ZERO; n - 1];
    for l in 2..pre_rendered {
        lower_bound[l] = (0..(n - l))
            .into_iter()
            .map(|i| m[(i, i + l)])
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
    }

    // To choose larger smallest bounds, we use the sum of the k_smallest possibilities
    // to combine k polygons of size pre_rendered+1
    let mut k_smallest = vec![S::ZERO; n / (pre_rendered - 1)];
    let mut sizes_of_key = (0..(n - pre_rendered))
        .into_iter()
        .map(|i| m[(i, i + pre_rendered)])
        .collect::<Vec<_>>();
    sizes_of_key.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
    for i in 1..(n / (pre_rendered - 1)) {
        k_smallest[i] = k_smallest[i - 1]
            + if sizes_of_key[i] == S::INFINITY {
                sizes_of_key[0]
            } else {
                sizes_of_key[i]
            };
    }

    // Now, we fill the lower bounds for the larger polygons by combining the above
    for l in pre_rendered..n - 1 {
        // this is the largest l that we have actual data for
        let max_num_vert = pre_rendered + 1;
        let num_vert = l + 1;

        // since each repetition of the largest structure shares two vertices with
        // the next copy, we can subtract 2 from each side
        let reps_of_largest = (num_vert - 2) / (max_num_vert - 2);

        // If we subtract the triangles covered by the above thing, we now have
        // those +2-2 vertices left. Since we have explicit data for those,
        // we can look them up!
        let left_over = num_vert - reps_of_largest * (max_num_vert - 2);

        lower_bound[l] = k_smallest[reps_of_largest] + lower_bound[left_over - 1];
    }

    lower_bound
}

fn fill_m<V: IndexType, Vec2: Vector2D>(
    n: usize,
    from: usize,
    to: usize,
    m: &mut TriangularStore<Vec2::S>,
    s: &mut TriangularStore<usize>,
    valid_diagonal: &TriangularStore<bool>,
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
) {
    for l in from..=to {
        // TODO: We never roll over. Is this fine? So, we ignore splits in (n-l)..n
        for i in 0..(n - l) {
            let j = i + l;

            let ij = m.index(i, j);
            let mut mij = Vec2::S::INFINITY;
            let mut sij = IndexType::max();
            for k in (i + 1)..j {
                debug_assert!(i < k && k < j);
                let ik = m.index(i, k);
                let kj = m.index(k, j);
                debug_assert!(m[ik] != Vec2::S::NEG_INFINITY);
                debug_assert!(m[kj] != Vec2::S::NEG_INFINITY);

                if !valid_diagonal[ik] || !valid_diagonal[kj] {
                    continue;
                }

                let weight = triangle_weight(&vs[i].vec, &vs[j].vec, &vs[k].vec);

                let cost = m[ik] + m[kj] + weight;
                if cost < mij {
                    mij = cost;
                    sij = k;
                }
            }

            m[ij] = mij;
            s[ij] = sij;
        }
    }
}

// TODO: find_valid_diagonals should use a O(n) algorithm instead of O(n^2) (each called n times)
// See: https://arxiv.org/pdf/1403.3905
fn find_valid_diagonals<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    n: usize,
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
) -> TriangularStore<bool> {
    let mut valid_diagonal = TriangularStore::<bool>::new(n, true);
    let poly = Poly::from_iter(vs.iter().map(|v| v.vec));
    for i in 0..n {
        for j in (i + 2)..n {
            let res = poly.valid_diagonal(i, j);
            valid_diagonal[(i, j)] = res;
        }
    }

    valid_diagonal
}

fn initialize_m<S: Scalar>(n: usize) -> TriangularStore<S> {
    let mut m = TriangularStore::<S>::new(n, S::NEG_INFINITY);
    for i in 0..n - 1 {
        // pairs of vertices have no edge length
        m[(i, i + 1)] = S::ZERO;
    }
    m
}

fn get_searched_portion<S: Scalar>(n: usize, m: &TriangularStore<S>) -> f32 {
    let mut eval = 0;
    let mut total = 0;
    for i in 0..n {
        for j in (i + 2)..n {
            total += 1;
            if m[(i, j)] != S::NEG_INFINITY {
                eval += 1;
            }
        }
    }
    eval as f32 / total as f32
}

/// Attempts to improve `minweight_dynamic_direct` by skipping large areas of the matrix
pub fn minweight_dynamic_direct2<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    indices: &mut Triangulation<V>,
) {
    /*
    TODO: Turns out, this is difficult to optimize!

    The idea was to calculated the weights of sub-polygons for l <= pre_rendered.
    Then, we add those together to get lower bounds for larger polygons.
    However, this lower bounds are usually much to low to be helpful. For them to be
    helpful, it is necessary to find a triangulation that is better than the lower bounds
    together with the newly inserted triangle. However, the new triangle isn't necessarily
    that bad, so we have to search all the newly inserted bad cases!

    In case of circle, inserting the "worst" triangle for a given ij results in the optimal triangle.
    Hence, using that logic, no matter how small our bounds are, we will always search the
    whole matrix.

     */

    let n = vs.len();
    assert!(n >= 5);
    let pre_rendered = 3;

    let mut m = initialize_m(n);
    let mut s = TriangularStore::<usize>::new(n, IndexType::max());

    let valid_diagonal = find_valid_diagonals::<V, Vec2, Poly>(n, vs);

    let now = Instant::now();

    fill_m(n, 2, pre_rendered, &mut m, &mut s, &valid_diagonal, &vs);
    let lower_bound = calculate_lower_bound(n, pre_rendered, &m);
    calculate_mij_on_demand(0, n - 1, &mut m, &mut s, &valid_diagonal, &lower_bound, &vs);

    let ela = now.elapsed();

    println!("lower_bound {:?}", lower_bound);
    // TODO: Calculate actual values and assert they are geq

    println!(
        "Dynamic programming: {:?} eval: {}%",
        ela,
        get_searched_portion(n, &m) * 100.0
    );

    traceback(n, 0, n - 1, &s, indices, &vs);

    todo!("Needs fixing!");
}

#[inline]
fn expand_mij_k_on_demand<V: IndexType, Vec2: Vector2D>(
    i: usize,
    j: usize,
    k: usize,
    mij: Vec2::S,
    m: &mut TriangularStore<Vec2::S>,
    s: &mut TriangularStore<usize>,
    valid_diagonal: &TriangularStore<bool>,
    lower_bound: &Vec<Vec2::S>,
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
) -> Option<Vec2::S> {
    let ik = m.index(i, k);
    let kj = m.index(k, j);

    if !valid_diagonal[ik] || !valid_diagonal[kj] {
        return None;
    }

    let weight = triangle_weight(&vs[i].vec, &vs[j].vec, &vs[k].vec);

    // first, attempt to stop early solely based on the lower bounds
    let mut mik = m[ik];
    let mik_lower = if mik == Vec2::S::NEG_INFINITY {
        lower_bound[k - i]
    } else {
        mik
    };
    let mut mkj = m[kj];
    let mkj_lower = if mkj == Vec2::S::NEG_INFINITY {
        lower_bound[j - k]
    } else {
        mkj
    };
    println!(
        "{},{}@{} | {}+{} | {:.4} {} {:.4} W {:.4}",
        i,
        j,
        k,
        k - i,
        j - k,
        mij,
        if mij <= mik_lower + mkj_lower + weight {
            "SKIP"
        } else {
            "CONT"
        },
        mik_lower + mkj_lower + weight,
        weight
    );
    if mij <= mik_lower + mkj_lower + weight {
        return None;
    }

    // if still not stopped, try to calculate one of the actual costs
    if mik == Vec2::S::NEG_INFINITY {
        // TODO: use a stack or priority queue or something. This recursion can cause a stack overflow!
        mik = calculate_mij_on_demand(i, k, m, s, valid_diagonal, lower_bound, vs);
        if mij <= mik + mkj_lower + weight {
            if mkj == Vec2::S::NEG_INFINITY {
                println!("STOP LATE");
            }
            return None;
        }
    }

    // lastly, calculate the other cost
    if mkj == Vec2::S::NEG_INFINITY {
        mkj = calculate_mij_on_demand(k, j, m, s, valid_diagonal, lower_bound, vs);
    }

    let cost = mik + mkj + weight;
    Some(cost)
}

fn calculate_mij_on_demand<V: IndexType, Vec2: Vector2D>(
    i: usize,
    j: usize,
    m: &mut TriangularStore<Vec2::S>,
    s: &mut TriangularStore<usize>,
    valid_diagonal: &TriangularStore<bool>,
    lower_bound: &Vec<Vec2::S>,
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
) -> Vec2::S {
    let ij = m.index(i, j);
    let mut mij = m[ij];
    if mij != Vec2::S::NEG_INFINITY {
        // It is already calculated - no need to recalculate
        return mij;
    }

    // Search for the best k
    mij = Vec2::S::INFINITY;
    let mut sij = IndexType::max();
    let mut skipped_anything = false;

    // Try to use the first existing one as a starting point
    for k in (i + 1)..j {
        let ik = m.index(i, k);
        let kj = m.index(k, j);
        let mik = m[ik];
        if mik == Vec2::S::NEG_INFINITY {
            skipped_anything = true;
            continue;
        }
        let mkj = m[kj];
        if mkj == Vec2::S::NEG_INFINITY {
            skipped_anything = true;
            continue;
        }
        if !valid_diagonal[ik] || !valid_diagonal[kj] {
            continue;
        }

        let weight = triangle_weight(&vs[i].vec, &vs[j].vec, &vs[k].vec);
        let cost = mik + mkj + weight;
        if cost < mij {
            mij = cost;
            sij = k;
        }
    }

    if skipped_anything {
        // Now, search thoroughly and consider expanding new areas
        for k in (i + 1)..j {
            if let Some(cost) =
                expand_mij_k_on_demand(i, j, k, mij, m, s, valid_diagonal, lower_bound, vs)
            {
                if cost < mij {
                    mij = cost;
                    sij = k;
                }
            }
        }

        println!("found {} {} via {} as {}", i, j, sij, mij);
    }

    m[ij] = mij;
    s[ij] = sij;
    return mij;
}

/// Like `minweight_dynamic`, but directly on a vertex list instead of a mesh face.
pub fn minweight_dynamic_direct<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    indices: &mut Triangulation<V>,
) {
    let n = vs.len();
    assert!(n >= 4, "n={} < 4", n);
    let mut m = initialize_m(n);
    let mut s = TriangularStore::<usize>::new(n, IndexType::max());

    //let now = Instant::now();

    let valid_diagonal = find_valid_diagonals::<V, Vec2, Poly>(n, &vs);

    //println!("Valid diagonals: {:?}", now.elapsed());
    //let now = Instant::now();

    fill_m(n, 2, n - 1, &mut m, &mut s, &valid_diagonal, &vs);

    //println!("Dynamic programming: {:?}", now.elapsed());

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

#[inline]
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

/*
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        bevy::{Bevy2DPolygon, BevyMesh3d, BevyMeshType3d32, BevyVertexPayload},
        mesh::MeshBasics,
        primitives::{generate_zigzag, Make2dShape},
        tesselate::{
            delaunay_triangulation, triangulate_face, TesselationMeta, TriangulationAlgorithm,
        },
    };
    use bevy::math::{Vec2, Vec3};

    #[test]
    fn test_minweight_dynamic_performance() {
        /*let mesh = BevyMesh3d::polygon(
            generate_zigzag::<Vec2>(100)
                .map(|v| BevyVertexPayload::from_pos(Vec3::new(v.x, 0.0, v.y))),
        );*/
        let mesh = BevyMesh3d::regular_polygon(1.0, 10);
        let mut meta = TesselationMeta::default();
        let mut indices = Vec::new();
        let mut tri = Triangulation::new(&mut indices);
        triangulate_face::<BevyMeshType3d32>(
            mesh.face(0),
            &mesh,
            &mut tri,
            TriangulationAlgorithm::MinWeight,
            &mut meta,
        );
        let vec2s = mesh.face(0).vec2s(&mesh);
        let vec_hm: HashMap<u32, Vec2> = vec2s.iter().map(|v| (v.index, v.vec)).collect();
        tri.verify_full::<Vec2, Bevy2DPolygon>(&vec2s);
        let w = tri.total_edge_weight(&vec_hm);

        let mut indices3 = Vec::new();
        let mut tri3 = Triangulation::new(&mut indices3);
        delaunay_triangulation::<BevyMeshType3d32>(mesh.face(0), &mesh, &mut tri3);
        let w3 = tri3.total_edge_weight(&vec_hm);

        let mut indices4 = Vec::new();
        let mut tri4 = Triangulation::new(&mut indices4);
        minweight_dynamic_direct2::<u32, Vec2, Bevy2DPolygon>(&vec2s, &mut tri4);
        tri4.verify_full::<Vec2, Bevy2DPolygon>(&vec2s);
        let w4 = tri4.total_edge_weight(&vec_hm);

        assert!(false, "Edge w: {} w4: {} delaunay: {}", w, w4, w3);
    }
}
*/
