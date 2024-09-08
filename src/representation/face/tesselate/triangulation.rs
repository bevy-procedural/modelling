use crate::math::{IndexType, LineSegment2D, Polygon, Scalar, Vector2D};
use std::collections::{HashMap, HashSet};

/// A vertex with its index in the global structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedVertex2D<I: IndexType, Vec2: Vector2D> {
    /// Position of the point
    pub vec: Vec2,
    /// Index in the global structure
    pub index: I,
}

impl<I: IndexType, Vec2: Vector2D> IndexedVertex2D<I, Vec2> {
    /// Create a new indexed vertex
    pub fn new(vec: Vec2, index: I) -> Self {
        IndexedVertex2D { vec, index }
    }

    /// Convert a vector of Vector2Ds to a vector of indexed vertices
    pub fn from_vector(vec: Vec<Vec2>) -> Vec<Self> {
        vec.into_iter()
            .enumerate()
            .map(|(i, v)| IndexedVertex2D::new(v, I::new(i)))
            .collect()
    }
}

/// A triangulation of a polygon
pub struct Triangulation<'a, I: IndexType> {
    indices: &'a mut Vec<I>,
    start: usize,
}

impl<'a, I: IndexType> Triangulation<'a, I> {
    /// Create a new triangulation
    pub fn new(indices: &'a mut Vec<I>) -> Self {
        Triangulation {
            start: indices.len(),
            indices,
        }
    }

    /// Insert a triangle into the triangulation using global indices
    pub fn insert_triangle(&mut self, a: I, b: I, c: I) {
        self.indices.extend([a, b, c]);
    }

    /// Insert a triangle into the triangulation using local indices
    pub fn insert_triangle_local<V: Vector2D>(
        &mut self,
        a: usize,
        b: usize,
        c: usize,
        vec2s: &Vec<IndexedVertex2D<I, V>>,
    ) {
        self.indices
            .extend([vec2s[a].index, vec2s[b].index, vec2s[c].index]);
    }

    /// Check for non-degenerate triangles (no zero-area triangles)
    pub fn verify_non_degenerate_triangle<Vec2: Vector2D>(self: &Self, vec_hm: &HashMap<I, Vec2>) {
        for i in (0..self.indices.len()).step_by(3) {
            let v0 = vec_hm[&self.indices[i]];
            let v1 = vec_hm[&self.indices[i + 1]];
            let v2 = vec_hm[&self.indices[i + 2]];

            // Use the determinant to check if the triangle has a non-zero area
            let area =
                (v1.x() - v0.x()) * (v2.y() - v0.y()) - (v1.y() - v0.y()) * (v2.x() - v0.x());
            assert!(
                area.abs() > Vec2::S::EPS,
                "Triangle has zero or negative area"
            );
        }
    }

    /// Check for valid indices (i.e., they should be within the bounds of the vertices)
    pub fn verify_indices<Vec2: Vector2D>(self: &Self, vec2s: &Vec<IndexedVertex2D<I, Vec2>>) {
        assert!(
            self.indices.len() % 3 == 0,
            "Invalid number of indices in triangulation"
        );

        // Check that the triangulation returns the correct number of triangles
        let num_vertices = vec2s.len();
        let num_triangles = self.indices.len() / 3;
        assert_eq!(
            num_triangles,
            num_vertices - 2,
            "Invalid number of triangles generated"
        );

        // Check for valid indices (i.e., they should be within the bounds of the vertices)
        for index in self.indices.iter() {
            assert!(
                index.index() < num_vertices,
                "Index out of bounds in triangulation"
            );
        }
    }

    /// Check for valid triangulation (no intersecting edges)
    pub fn verify_no_intersections<Vec2: Vector2D>(
        self: &Self,
        vec2s: &Vec<IndexedVertex2D<I, Vec2>>,
    ) {
        let num_vertices = vec2s.len();
        for i in (0..num_vertices).step_by(3) {
            for j in (0..num_vertices).step_by(3) {
                if i == j {
                    continue;
                }
                for k in 0..3 {
                    for l in 0..3 {
                        let v0 = vec2s[self.indices[(i + k) % 3].index()].vec;
                        let v1 = vec2s[self.indices[(i + k + 1) % 3].index()].vec;

                        let v2 = vec2s[self.indices[(j + l) % 3].index()].vec;
                        let v3 = vec2s[self.indices[(j + l + 1) % 3].index()].vec;

                        assert!(
                            LineSegment2D::new(v0, v1)
                                .intersect_line(
                                    &LineSegment2D::new(v2, v3),
                                    Vec2::S::EPS,  // be strict about parallel edges
                                    -Vec2::S::EPS  // Allow intersections/touching at the endpoints
                                )
                                .is_none(),
                            "Intersecting edges in triangulation\n{:?} -> {:?}\n{:?} -> {:?}",
                            v0,
                            v1,
                            v2,
                            v3
                        );
                    }
                }
            }
        }
    }

    /// Calculate the area of the polygon and check if it is the same as the sum of the areas of the triangles
    pub fn verify_area<V: Vector2D, Poly: Polygon<V, S = V::S>>(
        self: &Self,
        vec2s: &Vec<IndexedVertex2D<I, V>>,
        vec_hm: &HashMap<I, V>,
    ) {
        let area = V::S::from(0.5)
            * V::S::sum((0..self.indices.len()).step_by(3).into_iter().map(|i| {
                let v0 = vec_hm[&self.indices[i]];
                let v1 = vec_hm[&self.indices[i + 1]];
                let v2 = vec_hm[&self.indices[i + 2]];

                // Use the determinant to calculate the area of the triangle
                (v1.x() - v0.x()) * (v2.y() - v0.y()) - (v1.y() - v0.y()) * (v2.x() - v0.x())
            }));

        let reference = Poly::from_iter(vec2s.iter().map(|v| v.vec)).area();

        // Check if the area of the polygon is the same as the sum of the areas of the triangles
        assert!(
            (V::S::ONE - area / reference).abs() <= (V::S::ONE + V::S::from_usize(5) * V::S::EPS),
            "Area of the polygon is not equal to the sum of the areas of the triangles ({} != {})",
            area,
            reference
        );
    }

    /// Check that all indices are used at least once
    pub fn verify_all_indices_used<V: Vector2D>(self: &Self, vec2s: &Vec<IndexedVertex2D<I, V>>) {
        let mut seen = HashSet::new();
        for index in self.indices.iter() {
            seen.insert(index);
        }

        for vertex in vec2s.iter() {
            assert!(
                seen.contains(&vertex.index),
                "Vertex not used in triangulation {}",
                vertex.index.index()
            );
        }
    }
}
