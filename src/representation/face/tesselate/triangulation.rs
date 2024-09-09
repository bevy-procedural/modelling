use crate::math::{HasZero, IndexType, LineSegment2D, Polygon, Scalar, Vector2D};
use std::collections::{HashMap, HashSet};

/// A vertex with its index in the global structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedVertex2D<V: IndexType, Vec2: Vector2D> {
    /// Position of the point
    pub vec: Vec2,
    /// Index in the global structure
    pub index: V,
}

impl<V: IndexType, Vec2: Vector2D> IndexedVertex2D<V, Vec2> {
    /// Create a new indexed vertex
    pub fn new(vec: Vec2, index: V) -> Self {
        IndexedVertex2D { vec, index }
    }

    /// Convert a vector of Vector2Ds to a vector of indexed vertices
    pub fn from_vector(vec: Vec<Vec2>) -> Vec<Self> {
        vec.into_iter()
            .enumerate()
            .map(|(i, v)| IndexedVertex2D::new(v, V::new(i)))
            .collect()
    }
}

/// A triangulation of a polygon.
/// Will borrow the index buffer and append new triangles to it.
/// Most methods will only look at the indices that are added after the borrow startet.
/// It's fine to add triangles to the index buffer directly while it is borrowed.
pub struct Triangulation<'a, V: IndexType> {
    /// The index buffer
    indices: &'a mut Vec<V>,

    /// The position of the index where _this_ `Triangulation` begins
    start: usize,
}

impl<V: IndexType> std::fmt::Debug for Triangulation<'_, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Triangulation({} triangles; start {})",
            self.len(),
            self.start
        )?;
        for i in 0..self.len() {
            let (a, b, c) = self.get_triangle(i);
            write!(f, "\n{} {} {}", a, b, c)?;
        }
        Ok(())
    }
}

impl<'a, V: IndexType> Triangulation<'a, V> {
    /// Create a new triangulation
    pub fn new(indices: &'a mut Vec<V>) -> Self {
        Triangulation {
            start: indices.len(),
            indices,
        }
    }

    /// Insert a triangle into the triangulation using global indices
    pub fn insert_triangle(&mut self, a: V, b: V, c: V) {
        self.indices.extend([a, b, c]);
    }

    /// Get the ith index that was added to the triangulation
    pub fn get_index(&self, i: usize) -> V {
        self.indices[self.start + i]
    }

    /// Get a triangle from the triangulation using the number of the triangle in the triangulation
    pub fn get_triangle(&self, i: usize) -> (V, V, V) {
        (
            self.indices[self.start + 3 * i],
            self.indices[self.start + 3 * i + 1],
            self.indices[self.start + 3 * i + 2],
        )
    }

    /// Get the area of a triangle in the triangulation
    pub fn get_triangle_area<Vec2: Vector2D>(
        &self,
        i: usize,
        vec_hm: &HashMap<V, Vec2>,
    ) -> Vec2::S {
        let (i1, i2, i3) = self.get_triangle(i);
        let v0 = vec_hm[&i1];
        let v1 = vec_hm[&i2];
        let v2 = vec_hm[&i3];

        // Use the determinant to calculate the area of the triangle
        (v1.x() - v0.x()) * (v2.y() - v0.y()) - (v1.y() - v0.y()) * (v2.x() - v0.x())
    }

    /// Insert a triangle into the triangulation using local indices
    pub fn insert_triangle_local<Vec2: Vector2D>(
        &mut self,
        a: usize,
        b: usize,
        c: usize,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) {
        self.indices
            .extend([vec2s[a].index, vec2s[b].index, vec2s[c].index]);
    }

    /// Map the indices in the triangulation using a hashmap
    pub fn map_indices(&mut self, id_map: &HashMap<V, V>) {
        for i in self.start..self.indices.len() {
            self.indices[i] = id_map[&self.indices[i]];
        }
    }

    /// Get the number of triangles inserted into the index buffer since the triangulation was created
    pub fn len(self: &Self) -> usize {
        let n = self.indices.len() - self.start;
        assert!(n % 3 == 0, "Invalid number of indices in triangulation");
        n / 3
    }

    /// Check for non-degenerate triangles (no zero-area triangles)
    pub fn verify_non_degenerate_triangle<Vec2: Vector2D>(self: &Self, vec_hm: &HashMap<V, Vec2>) {
        for i in self.start..self.len() {
            let area = self.get_triangle_area(i, vec_hm);
            assert!(
                area.abs() > Vec2::S::ZERO,
                "Triangle has zero or negative area"
            );
        }
    }

    /// Check for valid indices (i.e., they should be within the bounds of the vertices)
    pub fn verify_indices<Vec2: Vector2D>(self: &Self, vec_hm: &HashMap<V, Vec2>) {
        // Check that the triangulation returns the correct number of triangles
        let num_vertices = vec_hm.len();
        let num_triangles = self.len();
        assert!(
            num_triangles == num_vertices - 2,
            "Expected {} triangles but found {}",
            num_vertices - 2,
            num_triangles
        );
        for i in self.start..self.indices.len() {
            assert!(
                vec_hm.get(&self.indices[i]).is_some(),
                "Index {} out of bounds in triangulation",
                self.indices[i]
            );
        }
    }

    /// Check that no two triangles have intersecting edges
    pub fn verify_no_intersections<Vec2: Vector2D>(self: &Self, vec_hm: &HashMap<V, Vec2>) {
        let num_vertices = vec_hm.len();
        for i in (0..num_vertices).step_by(3) {
            for j in (0..num_vertices).step_by(3) {
                if i == j {
                    continue;
                }
                for k in 0..3 {
                    for l in 0..3 {
                        let i0 = self.get_index(i + k);
                        let v0 = vec_hm[&i0];
                        let i1 = self.get_index(i + (k + 1) % 3);
                        let v1 = vec_hm[&i1];

                        let i2 = self.get_index(j + l);
                        let v2 = vec_hm[&i2];
                        let i3 = self.get_index(j + (l + 1) % 3);
                        let v3 = vec_hm[&i3];

                        // If they share a vertex, they can't intersect
                        if i0 == i2 || i0 == i3 || i1 == i2 || i1 == i3 {
                            continue;
                        }

                        let l1 = LineSegment2D::new(v0, v1);
                        let l2 = LineSegment2D::new(v2, v3);
                        let length = l1.length() + l2.length();
                        let inter = l1.intersect_line(
                            &l2,
                            Vec2::S::EPS.sqrt(), // be strict about parallel edges
                            -Vec2::S::EPS.sqrt() * length, // Allow intersections/touching at the endpoints up to a portion of sqrt(eps), i.e., 0.0345% for f32
                        );
                        assert!(
                            inter.is_none(),
                            "Edges: \n{} {:?} -> {} {:?}\n{} {:?} -> {} {:?}\nintersect in {:?} (shortest distance: {} * sqrt(eps))",
                            i0,
                            v0,
                            i1,
                            v1,
                            i2,
                            v2,
                            i3,
                            v3,
                            inter.unwrap(),
                            [v0,v1,v2,v3].iter().map(|v| inter.unwrap().distance(&v)).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() / Vec2::S::EPS.sqrt()
                        );
                    }
                }
            }
        }
    }

    /// Sum the area of all triangles added to the index buffer since the triangulation was created
    pub fn get_area<Vec2: Vector2D>(self: &Self, vec_hm: &HashMap<V, Vec2>) -> Vec2::S {
        Vec2::S::from(0.5)
            * Vec2::S::sum(
                (0..self.len())
                    .into_iter()
                    .map(|i| self.get_triangle_area(i, vec_hm).abs()),
            )
    }

    /// Calculate the area of the polygon and check if it is the same as the sum of the areas of the triangles
    pub fn verify_area<Vec2: Vector2D, Poly: Polygon<Vec2, S = Vec2::S>>(
        self: &Self,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        vec_hm: &HashMap<V, Vec2>,
    ) {
        let area = self.get_area(vec_hm);
        let reference = Poly::from_iter(vec2s.iter().map(|v| v.vec)).area();

        // Check if the area of the polygon is the same as the sum of the areas of the triangles
        assert!(
            (Vec2::S::ONE - area / reference).abs()
                <= (Vec2::S::ONE + Vec2::S::from_usize(5) * Vec2::S::EPS),
            "Area of the polygon is not equal to the sum of the areas of the triangles ({} != {})",
            area,
            reference
        );
    }

    /// Check that the set of used indices exactly matches the set of indices in the triangulation
    pub fn verify_all_indices_used<Vec2: Vector2D>(
        self: &Self,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) {
        let mut seen = HashSet::new();
        for i in self.start..self.indices.len() {
            seen.insert(self.indices[i]);
        }

        for vertex in vec2s.iter() {
            assert!(
                seen.remove(&vertex.index),
                "Vertex not used in triangulation {}",
                vertex.index.index()
            );
        }

        assert!(
            seen.is_empty(),
            "Foreign indices used in triangulation: {:?}",
            seen.iter().map(|i| i.index()).collect::<Vec<_>>()
        );
    }

    /// Runs a large number of tests on the triangulation to verify that it is well-formed
    pub fn verify_full<Vec2: Vector2D, Poly: Polygon<Vec2, S = Vec2::S>>(
        self: &Self,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) {
        let vec_hm: HashMap<V, Vec2> = vec2s.iter().map(|v| (v.index, v.vec)).collect();

        self.verify_indices(&vec_hm);
        self.verify_all_indices_used(&vec2s);
        self.verify_no_intersections(&vec_hm);
        self.verify_non_degenerate_triangle(&vec_hm);
        self.verify_area::<Vec2, Poly>(&vec2s, &vec_hm);
    }
}
