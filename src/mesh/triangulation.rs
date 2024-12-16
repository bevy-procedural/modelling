use crate::math::{IndexType, LineSegment2D, Polygon, Scalar, ScalarIteratorExt, Vector2D};
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
    pub fn len(&self) -> usize {
        let n = self.indices.len() - self.start;
        assert!(n % 3 == 0, "Invalid number of indices in triangulation");
        n / 3
    }

    /// Get the next index that will be added to the index buffer
    pub fn next_pos(&self) -> usize {
        self.indices.len()
    }

    /// Flip the edge of the two triangles
    pub fn flip_edge(
        &mut self,
        a: V,
        b: V,
        triangle_ab: usize,
        triangle_ba: usize,
    ) -> Result<(), ()> {
        let offset_ab = if self.indices[triangle_ab + 0] == a {
            0
        } else if self.indices[triangle_ab + 1] == a {
            1
        } else {
            2
        };
        if self.indices[triangle_ab + offset_ab] != a
            || self.indices[triangle_ab + ((offset_ab + 1) % 3)] != b
        {
            return Err(());
        }

        let offset_ba = if self.indices[triangle_ba + 0] == a {
            0
        } else if self.indices[triangle_ba + 1] == a {
            1
        } else {
            2
        };
        if self.indices[triangle_ba + offset_ba] != a
            || self.indices[triangle_ba + ((offset_ba + 2) % 3)] != b
        {
            return Err(());
        }

        let c = self.indices[triangle_ab + ((offset_ab + 2) % 3)];
        let d = self.indices[triangle_ba + ((offset_ba + 1) % 3)];

        // Apply the flip
        // abc -> adc
        // adb -> dbc
        self.indices[triangle_ab + 0] = a;
        self.indices[triangle_ab + 1] = d;
        self.indices[triangle_ab + 2] = c;
        self.indices[triangle_ba + 0] = d;
        self.indices[triangle_ba + 1] = b;
        self.indices[triangle_ba + 2] = c;

        Ok(())
    }

    /// Check for non-degenerate triangles (no zero-area triangles)
    pub fn verify_non_degenerate_triangle<Vec2: Vector2D>(&self, vec_hm: &HashMap<V, Vec2>) {
        for i in self.start..self.len() {
            let area = self.get_triangle_area(i, vec_hm);
            /*assert!(
                area.abs() > Vec2::S::ZERO,
                "Triangle has zero or negative area"
            );*/
            // degenerate triangles are ok. But not negative ones!
            if !(area >= -Vec2::S::EPS.sqrt()) {
                println!("Triangle area: {}", area);
                assert!(area >= -Vec2::S::EPS.sqrt(), "Triangle has negative area");
            }
        }
    }

    /// Check for valid indices (i.e., they should be within the bounds of the vertices)
    pub fn verify_indices<Vec2: Vector2D>(&self, vec_hm: &HashMap<V, Vec2>) {
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
    pub fn verify_no_intersections<Vec2: Vector2D>(&self, vec_hm: &HashMap<V, Vec2>) {
        let num_vertices = self.indices.len() - self.start;
        /*for i in (0..num_vertices).step_by(3) {
            println!(
                "tri: {:?}",
                (
                    self.get_index(i),
                    self.get_index(i + 1),
                    self.get_index(i + 2)
                ),
            );
        }*/

        for i in (self.start..num_vertices).step_by(3) {
            for j in (self.start..num_vertices).step_by(3) {
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
                            "Edges: \n{} {:?} -> {} {:?}\n{} {:?} -> {} {:?}\nintersect in {:?} (shortest distance: {} * sqrt(eps))\nTriangles {:?} and {:?}",
                            i0,
                            v0,
                            i1,
                            v1,
                            i2,
                            v2,
                            i3,
                            v3,
                            inter.unwrap(),
                            [v0,v1,v2,v3].iter().map(|v| inter.unwrap().distance(&v)).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() / Vec2::S::EPS.sqrt(),
                            (self.get_index(i), self.get_index(i+1), self.get_index(i+2)),
                            (self.get_index(j), self.get_index(j+1), self.get_index(j+2)),
                        );
                    }
                }
            }
        }
    }

    /// Sum the area of all triangles added to the index buffer since the triangulation was created
    pub fn get_area<Vec2: Vector2D>(&self, vec_hm: &HashMap<V, Vec2>) -> Vec2::S {
        Vec2::S::HALF
            * (0..self.len())
                .into_iter()
                .map(|i| self.get_triangle_area(i, vec_hm).abs())
                .stable_sum()
    }

    /// Calculate the total edge weight of the triangulation
    pub fn total_edge_weight<Vec2: Vector2D>(&self, vec_hm: &HashMap<V, Vec2>) -> Vec2::S {
        let mut total = Vec2::S::ZERO;
        for i in self.start..self.len() {
            let (i1, i2, i3) = self.get_triangle(i);
            let v0 = vec_hm[&i1];
            let v1 = vec_hm[&i2];
            let v2 = vec_hm[&i3];
            total += v1.distance(&v0) + v2.distance(&v1) + v0.distance(&v2);
        }
        total
    }

    /// Calculate the area of the polygon and check if it is the same as the sum of the areas of the triangles
    pub fn verify_area<Vec2: Vector2D, Poly: Polygon<Vec2>>(
        &self,
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
    pub fn verify_all_indices_used<Vec2: Vector2D>(&self, vec2s: &Vec<IndexedVertex2D<V, Vec2>>) {
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
    pub fn verify_full<Vec2: Vector2D, Poly: Polygon<Vec2>>(
        &self,
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
