use super::{
    chain::{ReflexChain, ReflexChainDirection},
    interval::{IntervalBoundaryEdge, SweepLineInterval},
    point::{EventPoint, LocallyIndexedVertex},
    status::SweepLineStatus,
    SweepMeta, VertexType,
};
use crate::{math::Vector2D, representation::IndexType};

/// Perform the sweep line triangulation
///
/// `indices` is the list of indices where the new triangles are appended (in local coordinates)
/// `vec2s` is the list of 2d-vertices with local indices
/// `meta` is a structure where debug information can be stored
pub fn sweep_line_triangulation<Vec2: Vector2D, V: IndexType>(
    indices: &mut Vec<V>,
    vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    meta: &mut SweepMeta,
) {
    assert!(vec2s.len() >= 3, "At least 3 vertices are required");
    let mut event_queue: Vec<EventPoint<Vec2>> = Vec::new();
    for here in 0..vec2s.len() {
        event_queue.push(EventPoint::classify::<V>(here, &vec2s));
    }
    event_queue.sort_unstable();

    println!("Event queue: {:?}", event_queue);
    let vt = event_queue.first().unwrap().vertex_type;
    assert!(
        vt == VertexType::Start || vt == VertexType::Regular,
        "The first vertex must be a start or regular vertex"
    );
    let lt = event_queue.last().unwrap().vertex_type;
    assert!(
        lt == VertexType::End || lt == VertexType::Regular,
        "The last vertex must be an end or regular vertex"
    );

    #[cfg(feature = "sweep_debug")]
    {
        meta.vertex_type = event_queue
            .iter()
            .map(|e| (e.here, e.vertex_type))
            .collect();
    }

    let mut q = SweepContext::new(indices, vec2s);

    for (event_i, event) in event_queue.iter().enumerate() {
        #[cfg(feature = "sweep_debug_print")]
        println!("###### {:?} {}", event.vertex_type, event.here);

        match event.vertex_type {
            VertexType::Start => q.start(&event),
            VertexType::Merge => q.merge(&event),
            VertexType::Regular => q.regular(&event, meta, event_i, &event_queue),
            VertexType::Split => q.split(&event),
            VertexType::End => q.end(&event),
            VertexType::Skip => {
                todo!("Skip collinear vertices");
            }
            VertexType::Undefined => {
                panic!("Vertex type is Undefined");
            }
        }

        #[cfg(feature = "sweep_debug_print")]
        println!("{}", q.sls);
    }
}

/// Central event queue of the sweep line triangulation
struct SweepContext<'a, Vec2: Vector2D, V: IndexType> {
    /// sweep line status lexicographically indexed by y and then x
    sls: SweepLineStatus<V, Vec2>,

    /// The list of indices where the new triangles are appended (in local coordinates)
    indices: &'a mut Vec<V>,

    /// The list of 2d-vertices with local indices
    vec2s: &'a Vec<LocallyIndexedVertex<Vec2>>,
}

impl<'a, Vec2: Vector2D, V: IndexType> SweepContext<'a, Vec2, V> {
    /// Creates a new event queue from a list of indexed vertex points
    fn new(indices: &'a mut Vec<V>, vec2s: &'a Vec<LocallyIndexedVertex<Vec2>>) -> Self {
        return Self {
            sls: SweepLineStatus::new(),
            indices,
            vec2s,
        };
    }

    /// Start a new sweep line at the given event
    fn start(self: &mut Self, event: &EventPoint<Vec2>) {
        // Both reflex
        self.sls.insert(SweepLineInterval {
            helper: event.here,
            left: IntervalBoundaryEdge::new(event.here, event.next),
            right: IntervalBoundaryEdge::new(event.here, event.prev),
            chain: ReflexChain::single(event.here),
            fixup: None,
        });
    }

    /// Merge two parts of the sweep line at the given event
    fn merge(self: &mut Self, event: &EventPoint<Vec2>) {
        // left and right are swapped because "remove_right" will get the left one _from_ the right (and vice versa)
        let left = self.sls.remove_right(event.here).unwrap();
        let mut right = self.sls.remove_left(event.here).unwrap();

        assert!(!left.is_end(), "Mustn't merge with an end vertex");
        assert!(!right.is_end(), "Mustn't merge with an end vertex");
        assert!(left != right, "Mustn't be the same to merge them");

        let mut new_stacks = if let Some(mut fixup) = left.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup merge l: {}", fixup);

            fixup.right(event.here, self.indices, self.vec2s);
            assert!(fixup.is_done());
            left.chain
        } else {
            left.chain
        };

        let mut new_fixup = if let Some(fixup) = right.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup merge r: {}", fixup);

            right.chain.left(event.here, self.indices, self.vec2s);
            assert!(right.chain.is_done());
            fixup
        } else {
            right.chain
        };

        self.sls.insert(SweepLineInterval {
            helper: event.here,
            left: left.left,
            right: right.right,
            chain: {
                new_stacks.right(event.here, self.indices, self.vec2s);
                new_stacks
            },
            fixup: Some({
                new_fixup.left(event.here, self.indices, self.vec2s);
                new_fixup
            }),
        });
    }

    /// There can be end vertices that were undetected because they
    /// had the same y-coordinate as other regular vertices.
    /// This routine will fix this
    #[inline]
    fn hidden_end(
        self: &mut Self,
        event: &EventPoint<Vec2>,
        interval: SweepLineInterval<V, Vec2>,
        meta: &mut SweepMeta,
        event_i: usize,
        queue: &Vec<EventPoint<Vec2>>,
    ) {
        let Some(previous) = queue.get(event_i - 1) else {
            panic!("Convergent sweep line at the first event");
        };
        assert!(
            previous.vec.y() == event.vec.y(),
            "Expected an end vertex, but found no evidence"
        );
        debug_assert!(interval.is_end());

        #[cfg(feature = "sweep_debug_print")]
        println!("Reinterpret as end");

        #[cfg(feature = "sweep_debug")]
        meta.update_type(event.here, VertexType::End);

        self.sls.insert(interval);
        self.end(event);
    }

    /// Handle a regular vertex
    fn regular(
        self: &mut Self,
        event: &EventPoint<Vec2>,
        meta: &mut SweepMeta,
        event_i: usize,
        queue: &Vec<EventPoint<Vec2>>,
    ) {
        // PERF: optional; modify sls instead of remove and insert
        if let Some(mut interval) = self.sls.remove_left(event.here) {
            if interval.is_end() {
                self.hidden_end(event, interval, meta, event_i, queue);
                return;
            }

            let mut stacks = if let Some(fixup) = interval.fixup {
                #[cfg(feature = "sweep_debug_print")]
                println!("fixup regular l: {}", fixup);

                interval.chain.left(event.here, self.indices, self.vec2s);
                assert!(interval.chain.is_done());
                fixup
            } else {
                interval.chain
            };
            self.sls.insert(SweepLineInterval {
                helper: event.here,
                left: IntervalBoundaryEdge::new(event.here, event.next),
                right: interval.right,
                chain: {
                    stacks.left(event.here, self.indices, self.vec2s);
                    stacks
                },
                fixup: None,
            })
        } else if let Some(mut interval) = self.sls.remove_right(event.here) {
            if interval.is_end() {
                self.hidden_end(event, interval, meta, event_i, queue);
                return;
            }

            if let Some(mut fixup) = interval.fixup {
                #[cfg(feature = "sweep_debug_print")]
                println!("fixup regular r: {}", fixup);

                fixup.right(event.here, self.indices, self.vec2s);
                assert!(fixup.is_done());
            }
            self.sls.insert(SweepLineInterval {
                helper: event.here,
                left: interval.left,
                right: IntervalBoundaryEdge::new(event.here, event.prev),
                chain: {
                    interval.chain.right(event.here, self.indices, self.vec2s);
                    interval.chain
                },
                fixup: None,
            })
        } else {
            #[cfg(feature = "sweep_debug_print")]
            println!("Reinterpret as start");

            // If there are two or more vertices with the same y-coordinate, they will all be labeled "regular"
            // In that case, the first one must be treated as a start
            // We start with some checks whether this is plausible and not a bug

            let Some(next) = queue.get(event_i + 1) else {
                panic!("Regular vertex not found in sweep line status");
            };
            assert!(
                next.vec.y() == event.vec.y(),
                "Regular vertex not found in sweep line status"
            );

            // treat this one as a start
            self.start(event);

            // update the meta info
            #[cfg(feature = "sweep_debug")]
            meta.update_type(event.here, VertexType::Start);
        }
    }

    /// Split the sweep line at the given event
    fn split(self: &mut Self, event: &EventPoint<Vec2>) {
        let i = *self
            .sls
            .find_by_position(&self.vec2s[event.here].vec, self.vec2s)
            .unwrap_or_else(|| {
                panic!(
                    "Split vertex not found in sweep line status\nEvent: {:?}\nPos: {:?}\nStatus: {}",
                    event,
                    self.vec2s[event.here].vec,
                    self.sls
                )
            })
            .0;
        let line = self.sls.remove_left(i).unwrap();
        assert!(!line.is_end(), "A split vertex must not be an end vertex");

        let stacks = if let Some(mut fixup) = line.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup split: {}", fixup);

            let t = fixup.first();

            fixup.right(event.here, self.indices, self.vec2s);
            assert!(fixup.is_done());

            let mut x = ReflexChain::single(t);
            x.left(event.here, self.indices, self.vec2s);
            x
        } else if line.chain.direction() == ReflexChainDirection::Right {
            let mut x = ReflexChain::single(line.helper);
            x.left(event.here, self.indices, self.vec2s);
            x
        } else {
            let mut x = ReflexChain::single(line.chain.first());
            x.left(event.here, self.indices, self.vec2s);
            x
        };

        self.sls.insert(SweepLineInterval {
            helper: event.here,
            left: line.left,
            right: IntervalBoundaryEdge::new(event.here, event.prev),
            chain: {
                let mut x = line.chain.clone();
                x.right(event.here, self.indices, self.vec2s);
                x
            },
            fixup: None,
        });

        self.sls.insert(SweepLineInterval {
            helper: event.here,
            left: IntervalBoundaryEdge::new(event.here, event.next),
            right: line.right,
            chain: stacks,
            fixup: None,
        });
    }

    /// End a sweep line at the given event
    #[inline]
    fn end(self: &mut Self, event: &EventPoint<Vec2>) {
        let mut line = self.sls.remove_left(event.here).unwrap();
        assert!(line.is_end());

        if let Some(mut fixup) = line.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup end: {}", fixup);

            fixup.right(event.here, self.indices, self.vec2s);
            assert!(fixup.is_done());
        }

        line.chain.left(event.here, self.indices, self.vec2s);
        assert!(line.chain.is_done());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{impls::bevy::Bevy2DPolygon, LineSegment2D, Polygon, Scalar};
    use bevy::math::Vec2;
    use rand::Rng;

    /// Check for non-degenerate triangles (no zero-area triangles)
    fn verify_non_degenerate_triangle<Vec2: Vector2D>(
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
        indices: &Vec<usize>,
    ) {
        for i in (0..indices.len()).step_by(3) {
            let v0 = vec2s[indices[i]].vec;
            let v1 = vec2s[indices[i + 1]].vec;
            let v2 = vec2s[indices[i + 2]].vec;

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
    fn verify_indices<Vec2: Vector2D>(
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
        indices: &Vec<usize>,
    ) {
        // Check that the triangulation returns the correct number of triangles
        let num_vertices = vec2s.len();
        let num_triangles = indices.len() / 3;
        assert_eq!(
            num_triangles,
            num_vertices - 2,
            "Invalid number of triangles generated"
        );

        // Check for valid indices (i.e., they should be within the bounds of the vertices)
        for &index in indices {
            assert!(index < num_vertices, "Index out of bounds in triangulation");
        }
    }

    /// Check, that all indices are used at lest once
    fn verify_all_indices_used(indices: &Vec<usize>, num_vertices: usize) {
        let mut used = vec![false; num_vertices];
        for &index in indices {
            used[index] = true;
        }
        assert!(
            used.iter().all(|&u| u),
            "Not all vertices are used in triangulation"
        );
    }

    /// Check for valid triangulation (no intersecting edges)
    fn verify_no_intersections<Vec2: Vector2D>(
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
        indices: &Vec<usize>,
    ) {
        let num_vertices = vec2s.len();
        for i in (0..num_vertices).step_by(3) {
            for j in (0..num_vertices).step_by(3) {
                if i == j {
                    continue;
                }
                for k in 0..3 {
                    for l in 0..3 {
                        let v0 = vec2s[indices[(i + k) % 3]].vec;
                        let v1 = vec2s[indices[(i + k + 1) % 3]].vec;

                        let v2 = vec2s[indices[(j + l) % 3]].vec;
                        let v3 = vec2s[indices[(j + l + 1) % 3]].vec;

                        assert!(
                            LineSegment2D::new(v0, v1)
                                .intersect_line(
                                    &LineSegment2D::new(v2, v3),
                                    Vec2::S::EPS,  // be strict about parallel edges
                                    -Vec2::S::EPS  // Allow intersections at the endpoints
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
    fn verify_area(vec2s: &Vec<LocallyIndexedVertex<bevy::math::Vec2>>, indices: &Vec<usize>) {
        let mut area = 0.0;
        // PERF: better summing algorithm?
        for i in (0..indices.len()).step_by(3) {
            let v0 = vec2s[indices[i]].vec;
            let v1 = vec2s[indices[i + 1]].vec;
            let v2 = vec2s[indices[i + 2]].vec;

            // Use the determinant to calculate the area of the triangle
            let triangle_area =
                0.5 * ((v1.x - v0.x) * (v2.y - v0.y) - (v1.y - v0.y) * (v2.x - v0.x));
            area += triangle_area;
        }

        let reference = Bevy2DPolygon::from_iter(vec2s.iter().map(|v| v.vec)).area();

        // Check if the area of the polygon is the same as the sum of the areas of the triangles
        assert!(
            (1.0 - area / reference).abs() <= (1.0 + 5.0 * f32::EPS),
            "Area of the polygon is not equal to the sum of the areas of the triangles ({} != {})",
            area,
            reference
        );
    }

    fn verify_triangulation(vec2s: &Vec<LocallyIndexedVertex<Vec2>>) {
        assert!(
            Bevy2DPolygon::from_iter(vec2s.iter().map(|v| v.vec)).is_ccw(),
            "Polygon must be counterclockwise"
        );

        let mut indices = Vec::<usize>::new();
        let mut meta = SweepMeta::default();
        let num_vertices = vec2s.len();
        sweep_line_triangulation(&mut indices, &vec2s, &mut meta);

        verify_indices(&vec2s, &indices);
        verify_all_indices_used(&indices, num_vertices);
        verify_no_intersections(&vec2s, &indices);
        verify_non_degenerate_triangle(&vec2s, &indices);
        verify_area(&vec2s, &indices);
    }

    fn random_star(
        min_vert: usize,
        max_vert: usize,
        min_r: f32,
        max_r: f32,
    ) -> Vec<LocallyIndexedVertex<Vec2>> {
        let mut vec2s = Vec::new();
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(min_vert..max_vert);
        for i in 0..n {
            let phi = i as f32 / n as f32 * 2.0 * std::f32::consts::PI;
            let r = rng.gen_range(min_r..max_r);
            let x = r * phi.cos();
            let y = r * phi.sin();
            vec2s.push(LocallyIndexedVertex::new(Vec2::from_xy(x, y), vec2s.len()));
        }

        vec2s
    }

    fn liv_from_array(arr: &[[f32; 2]]) -> Vec<LocallyIndexedVertex<Vec2>> {
        arr.iter()
            .enumerate()
            .map(|(i, &v)| LocallyIndexedVertex::new(Vec2::from_xy(v[0], v[1]), i))
            .collect()
    }

    #[test]
    fn sweep_triangle() {
        verify_triangulation(&liv_from_array(&[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]));
    }

    #[test]
    fn sweep_square() {
        verify_triangulation(&liv_from_array(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ]));
    }

    #[test]
    fn sweep_tricky_square() {
        verify_triangulation(&liv_from_array(&[
            [1.0, 0.0],
            [0.0, 1.0],
            [-1.0, 0.0],
            [0.0, 0.9],
        ]));
    }

    #[test]
    fn sweep_tricky_shape() {
        verify_triangulation(&liv_from_array(&[
            // front
            [1.0, 1.0],
            [0.5, -0.9],
            [0.0, 0.8],
            [-0.6, 1.0],
            [-0.8, 0.8],
            [-1.0, 1.0],
            // back
            [-1.0, -1.0],
            [0.0, -0.8],
            [0.6, -1.0],
            [0.8, -0.8],
            [1.0, -1.0],
        ]));
    }
    
    #[test]
    fn sweep_fuzz() {
        for _ in 1..1 {
            let vec2s = random_star(3, 100, 0.01, 10.0);

            println!(
                "vec2s: {:?}",
                vec2s.iter().map(|v| [v.vec.x, v.vec.y]).collect::<Vec<_>>()
            );

            verify_triangulation(&vec2s);
        }
    }

    // Problematic cases:
    // [[2.001453, 0.0], [0.7763586, 2.3893864], [-3.2887821, 2.3894396], [-2.7725635, -2.0143867], [0.023867942, -0.07345794]]
    // [[2.8768363, 0.0], [1.6538008, 2.0738008], [-0.5499903, 2.4096634], [-6.9148006, 3.3299913], [-7.8863497, -3.7978687], [-0.8668613, -3.7979746], [1.1135457, -1.3963413]]
}
