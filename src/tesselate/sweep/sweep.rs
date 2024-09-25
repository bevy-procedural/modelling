use super::{
    interval::{IntervalBoundaryEdge, SweepLineInterval},
    monotone::MonotoneTriangulator,
    point::EventPoint,
    status::SweepLineStatus,
    SweepMeta, VertexType,
};
use crate::mesh::{IndexedVertex2D, Triangulation};

/// Perform the sweep line triangulation
/// The sweep line moves from the top (positive y) to the bottom (negative y).
///
/// See [CMSC 754](https://web.archive.org/web/20240603202156/https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf) for more information on the algorithm.
///
/// `indices` is the list of indices where the new triangles are appended (in local coordinates)
/// `vec2s` is the list of 2d-vertices with local indices
/// `meta` is a structure where debug information can be stored
pub fn sweep_line_triangulation<MT: MonotoneTriangulator>(
    indices: &mut Triangulation<MT::V>,
    vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    meta: &mut SweepMeta<MT::V>,
) {
    assert!(vec2s.len() >= 3, "At least 3 vertices are required");

    let mut event_queue: Vec<EventPoint<MT::Vec2>> = Vec::new();
    for here in 0..vec2s.len() {
        event_queue.push(EventPoint::classify::<MT::V>(here, &vec2s));
    }
    event_queue.sort_unstable();

    let vt = event_queue.first().unwrap().vertex_type;
    assert!(
        vt == VertexType::Start || vt == VertexType::Regular || vt == VertexType::Undecisive,
        "The first vertex must be a start or regular vertex, but was {:?}",
        vt
    );
    let lt = event_queue.last().unwrap().vertex_type;
    assert!(
        lt == VertexType::End || lt == VertexType::Regular || lt == VertexType::Undecisive,
        "The last vertex must be an end or regular vertex, but was {:?}",
        lt
    );

    #[cfg(feature = "sweep_debug")]
    {
        meta.vertex_type = event_queue
            .iter()
            .map(|e| (vec2s[e.here].index, e.vertex_type))
            .collect();
    }

    let mut q = SweepContext::<MT>::new(indices, vec2s);

    for event in event_queue.iter() {
        #[cfg(feature = "sweep_debug_print")]
        println!("###### {:?} {}", event.vertex_type, event.here);

        match event.vertex_type {
            VertexType::Start => q.start(&event),
            VertexType::Split => assert!(q.try_split(&event)),
            VertexType::Merge => q.merge(&event),
            VertexType::End => q.end(&event),
            VertexType::Regular => q.regular(&event, meta, false),
            VertexType::Undecisive => q.regular(&event, meta, true),
            _ => {
                panic!("Unsupported vertex type {:?}", event.vertex_type);
            }
        }

        #[cfg(feature = "sweep_debug_print")]
        println!("{}", q.sls);
    }
}

/// Central event queue of the sweep line triangulation
struct SweepContext<'a, 'b, MT: MonotoneTriangulator> {
    /// sweep line status lexicographically indexed by y and then x
    sls: SweepLineStatus<MT>,

    /// The list of indices where the new triangles are appended (in local coordinates)
    tri: &'a mut Triangulation<'b, MT::V>,

    /// The list of 2d-vertices with local indices
    vec2s: &'a Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
}

impl<'a, 'b, MT: MonotoneTriangulator> SweepContext<'a, 'b, MT> {
    /// Creates a new event queue from a list of indexed vertex points
    fn new(
        tri: &'a mut Triangulation<'b, MT::V>,
        vec2s: &'a Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    ) -> Self {
        return Self {
            sls: SweepLineStatus::new(),
            tri,
            vec2s,
        };
    }

    /// Start a new sweep line at the given event
    fn start(&mut self, event: &EventPoint<MT::Vec2>) {
        // Both reflex
        self.sls.insert(
            SweepLineInterval {
                helper: event.here,
                left: IntervalBoundaryEdge::new(event.here, event.next),
                right: IntervalBoundaryEdge::new(event.here, event.prev),
                chain: MonotoneTriangulator::new(event.here),
                fixup: None,
            },
            self.vec2s,
        );
    }

    /// Split the sweep line at the given event
    fn try_split(&mut self, event: &EventPoint<MT::Vec2>) -> bool {
        let Some(i) = self
            .sls
            .find_by_position(&self.vec2s[event.here].vec, &self.vec2s)
        else {
            return false;
        };
        let line = self.sls.remove_left(i, &self.vec2s).unwrap();
        assert!(!line.is_end(), "A split vertex must not be an end vertex");

        let stacks = if let Some(mut fixup) = line.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup split: {}", fixup);

            let t = fixup.last_opposite();

            fixup.right(event.here, self.tri, self.vec2s);
            fixup.finish(self.tri, self.vec2s);

            let mut x = MT::new(t);
            x.left(event.here, self.tri, self.vec2s);
            x
        } else if line.chain.is_right() {
            let mut x = MT::new(line.helper);
            x.left(event.here, self.tri, self.vec2s);
            x
        } else {
            let mut x = MT::new(line.chain.last_opposite());
            x.left(event.here, self.tri, self.vec2s);
            x
        };

        self.sls.insert(
            SweepLineInterval {
                helper: event.here,
                left: line.left,
                right: IntervalBoundaryEdge::new(event.here, event.prev),
                chain: {
                    let mut x = line.chain;
                    x.right(event.here, self.tri, self.vec2s);
                    x
                },
                fixup: None,
            },
            self.vec2s,
        );

        self.sls.insert(
            SweepLineInterval {
                helper: event.here,
                left: IntervalBoundaryEdge::new(event.here, event.next),
                right: line.right,
                chain: stacks,
                fixup: None,
            },
            self.vec2s,
        );

        return true;
    }

    /// Detects and handles either a start or split vertex in the situation where it's difficult to distinguish
    fn start_or_split(
        &mut self,
        event: &EventPoint<MT::Vec2>,
        _meta: &mut SweepMeta<MT::V>,
    ) -> bool {
        /*
        let Some(next) = queue.get(event_i + 1) else {
            panic!("Regular vertex not found in sweep line status");
        };

        // Generally, this should only happen when they are extremely close to each other.
        // But due to numerical instabilities, this is hard to test.
        debug_assert!(
            (next.vec.y() - event.vec.y()).abs() <= Vec2::S::EPS * 2.0.into(),
            "Expected a start vertex, but found no evidence {} != {}",
            next.vec.y(),
            event.vec.y()
        );*/

        if self.try_split(event) {
            #[cfg(feature = "sweep_debug_print")]
            println!("Reinterpret as split");

            // update the meta info
            #[cfg(feature = "sweep_debug")]
            _meta.update_type(self.vec2s[event.here].index, VertexType::SplitLate);
        } else {
            #[cfg(feature = "sweep_debug_print")]
            println!("Reinterpret as start");

            // treat this one as a start
            self.start(event);

            // update the meta info
            #[cfg(feature = "sweep_debug")]
            _meta.update_type(self.vec2s[event.here].index, VertexType::StartLate);
        }

        return true;
    }

    /// End a sweep line at the given event
    #[inline]
    fn end(&mut self, event: &EventPoint<MT::Vec2>) {
        let mut line = self.sls.remove_left(event.here, &self.vec2s).unwrap();
        assert!(line.is_end());

        if let Some(mut fixup) = line.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup end: {}", fixup);

            fixup.right(event.here, self.tri, self.vec2s);
            fixup.finish(self.tri, self.vec2s);
        }

        line.chain.left(event.here, self.tri, self.vec2s);
        line.chain.finish(self.tri, self.vec2s);
    }

    /// Merge two parts of the sweep line at the given event
    fn merge(&mut self, event: &EventPoint<MT::Vec2>) {
        // left and right are swapped because "remove_right" will get the left one _from_ the right (and vice versa)
        let left = self.sls.remove_right(event.here, &self.vec2s).unwrap();
        let mut right: SweepLineInterval<MT> =
            self.sls.remove_left(event.here, &self.vec2s).unwrap();

        assert!(!left.is_end(), "Mustn't merge with an end vertex");
        assert!(!right.is_end(), "Mustn't merge with an end vertex");
        //assert!(left != right, "Mustn't be the same to merge them");

        let mut new_stacks = if let Some(mut fixup) = left.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup merge l: {}", fixup);

            fixup.right(event.here, self.tri, self.vec2s);
            fixup.finish(self.tri, self.vec2s);
            left.chain
        } else {
            left.chain
        };

        let mut new_fixup = if let Some(fixup) = right.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup merge r: {}", fixup);

            right.chain.left(event.here, self.tri, self.vec2s);
            right.chain.finish(self.tri, self.vec2s);
            fixup
        } else {
            right.chain
        };

        self.sls.insert(
            SweepLineInterval {
                helper: event.here,
                left: left.left,
                right: right.right,
                chain: {
                    new_stacks.right(event.here, self.tri, self.vec2s);
                    new_stacks
                },
                fixup: Some({
                    new_fixup.left(event.here, self.tri, self.vec2s);
                    new_fixup
                }),
            },
            self.vec2s,
        );
    }

    /// Handle a regular vertex
    fn regular(
        &mut self,
        event: &EventPoint<MT::Vec2>,
        meta: &mut SweepMeta<MT::V>,
        undecisive: bool,
    ) {
        // PERF: find whether to expect the left or right side beforehand. The lookup is expensive.

        if let Some(mut interval) = self.sls.remove_left(event.here, &self.vec2s) {
            if undecisive {
                if interval.is_end() {
                    #[cfg(feature = "sweep_debug_print")]
                    println!("Reinterpret as end");
                    #[cfg(feature = "sweep_debug")]
                    meta.update_type(self.vec2s[event.here].index, VertexType::EndLate);
                    // re-insert is faster than peeking since late vertex classification is rare
                    self.sls.insert(interval, self.vec2s);
                    self.end(event);
                    return;
                }
                if self.sls.peek_right(event.here).is_some() {
                    #[cfg(feature = "sweep_debug_print")]
                    println!("Reinterpret as merge");
                    #[cfg(feature = "sweep_debug")]
                    meta.update_type(self.vec2s[event.here].index, VertexType::MergeLate);
                    self.sls.insert(interval, self.vec2s);
                    self.merge(event);
                    return;
                }
            }

            let mut stacks = if let Some(fixup) = interval.fixup {
                #[cfg(feature = "sweep_debug_print")]
                println!("fixup regular l: {}", fixup);

                interval.chain.left(event.here, self.tri, self.vec2s);
                interval.chain.finish(self.tri, self.vec2s);
                fixup
            } else {
                interval.chain
            };
            self.sls.insert(
                SweepLineInterval {
                    helper: event.here,
                    left: IntervalBoundaryEdge::new(event.here, event.next),
                    right: interval.right,
                    chain: {
                        stacks.left(event.here, self.tri, self.vec2s);
                        stacks
                    },
                    fixup: None,
                },
                self.vec2s,
            )
        } else if let Some(mut interval) = self.sls.remove_right(event.here, &self.vec2s) {
            if undecisive {
                if interval.is_end() {
                    #[cfg(feature = "sweep_debug_print")]
                    println!("Reinterpret as end");
                    #[cfg(feature = "sweep_debug")]
                    meta.update_type(self.vec2s[event.here].index, VertexType::EndLate);
                    // re-insert is faster than peeking since late vertex classification is rare
                    self.sls.insert(interval, self.vec2s);
                    self.end(event);
                    return;
                }
                if self.sls.peek_left(event.here).is_some() {
                    #[cfg(feature = "sweep_debug_print")]
                    println!("Reinterpret as merge");
                    #[cfg(feature = "sweep_debug")]
                    meta.update_type(self.vec2s[event.here].index, VertexType::MergeLate);
                    self.sls.insert(interval, self.vec2s);
                    self.merge(event);
                    return;
                }
            }

            if let Some(mut fixup) = interval.fixup {
                #[cfg(feature = "sweep_debug_print")]
                println!("fixup regular r: {}", fixup);

                fixup.right(event.here, self.tri, self.vec2s);
                fixup.finish(self.tri, self.vec2s);
            }
            self.sls.insert(
                SweepLineInterval {
                    helper: event.here,
                    left: interval.left,
                    right: IntervalBoundaryEdge::new(event.here, event.prev),
                    chain: {
                        interval.chain.right(event.here, self.tri, self.vec2s);
                        interval.chain
                    },
                    fixup: None,
                },
                self.vec2s,
            )
        } else {
            self.start_or_split(event, meta);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bevy::Bevy2DPolygon,
        math::{Polygon, Scalar},
        primitives::{generate_zigzag, random_star},
        tesselate::sweep::{monotone::LinearMonoTriangulator, DynamicMonoTriangulator},
    };

    use super::*;
    use bevy::math::Vec2;

    fn verify_triangulation_i<MT: MonotoneTriangulator<V = usize, Vec2 = Vec2>>(
        vec2s: &Vec<IndexedVertex2D<usize, Vec2>>,
    ) {
        assert!(
            Bevy2DPolygon::from_iter(vec2s.iter().map(|v| v.vec)).is_ccw(),
            "Polygon must be counterclockwise"
        );
        let mut indices = Vec::new();
        let mut tri = Triangulation::new(&mut indices);
        let mut meta = SweepMeta::default();
        sweep_line_triangulation::<MT>(&mut tri, &vec2s, &mut meta);
        tri.verify_full::<Vec2, Bevy2DPolygon>(vec2s);
    }

    fn verify_triangulation(vec2s: &Vec<IndexedVertex2D<usize, Vec2>>) {
        println!("LINEAR");
        verify_triangulation_i::<LinearMonoTriangulator<usize, Vec2>>(vec2s);
        println!("DYNAMIC");
        verify_triangulation_i::<DynamicMonoTriangulator<usize, Vec2, Bevy2DPolygon>>(vec2s);
    }

    fn liv_from_array(arr: &[[f32; 2]]) -> Vec<IndexedVertex2D<usize, Vec2>> {
        arr.iter()
            .enumerate()
            .map(|(i, &v)| IndexedVertex2D::new(Vec2::new(v[0], v[1]), i))
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
    fn sweep_tricky_quad() {
        verify_triangulation(&liv_from_array(&[
            [1.0, 0.0],
            [0.0, 1.0],
            [-1.0, 0.0],
            [0.0, 0.9],
        ]));
    }

    /*
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
    }*/

    #[test]
    fn sweep_zigzag() {
        verify_triangulation(
            &generate_zigzag::<Vec2>(100)
                .enumerate()
                .map(|(i, v)| IndexedVertex2D::new(v, i))
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

    #[test]
    fn numerical_hell_2() {
        verify_triangulation(&liv_from_array(&[
            [2.8768363, 0.0],
            [1.6538008, 2.0738008],
            [-0.5499903, 2.4096634],
            [-6.9148006, 3.3299913],
            [-7.8863497, -3.7978687],
            [-0.8668613, -3.7979746],
            [1.1135457, -1.3963413],
        ]));
    }

    #[test]
    fn numerical_hell_3() {
        // has a hidden end vertex
        verify_triangulation(&liv_from_array(&[
            [7.15814, 0.0],
            [2.027697, 2.542652],
            [-1.5944574, 6.98577],
            [-0.36498743, 0.17576863],
            [-2.3863406, -1.149202],
            [-0.11696472, -0.5124569],
            [0.40876004, -0.5125686],
        ]));
    }

    #[test]
    fn numerical_hell_4() {
        // has a hidden merge vertex
        verify_triangulation(&liv_from_array(&[
            [5.1792994, 0.0],
            [0.46844417, 0.5874105],
            [-0.13406669, 0.58738416],
            [-7.662568, 3.6900969],
            [-2.7504041, -1.3245257],
            [-0.4468068, -1.9575921],
            [0.7220693, -0.90544575],
        ]));
    }

    #[test]
    fn numerical_hell_5() {
        // has a undecisive end vertex
        verify_triangulation(&liv_from_array(&[
            [9.576968, 0.0],
            [-3.2991974e-7, 7.5476837],
            [-0.9634365, -8.422629e-8],
            [5.8283815e-14, -4.887581e-6],
        ]));
    }

    #[test]
    fn numerical_hell_6() {
        // has vertices with quite different y that still cause problems with being to parallel to the sweep line
        // vertex 2 might appear to be a start or split, but it turns out to be a merge. Quite tricky.
        verify_triangulation(&liv_from_array(&[
            [1.9081093, 0.0],
            [0.0056778197, 0.007119762],
            [-0.0015940086, 0.0069838036],
            [-0.018027846, 0.00868175],
            [-8.513409, -4.0998445],
            [-0.63087374, -2.7640438],
            [0.28846893, -0.36172837],
        ]));
    }

    #[test]
    fn numerical_hell_7() {
        // this will provoke intersecting edges with almost collinear edges
        verify_triangulation(&liv_from_array(&[
            [3.956943, 0.0],
            [0.42933345, 1.3213526],
            [-4.2110167, 3.059482],
            [-5.484937, -3.985043],
            [1.8108786, -5.573309],
        ]));
    }

    /*
    #[test]
    fn numerical_hell_8() {
        // TODO: how to make this numerically stable? This is due to numerical instability, but sorting them differently could probably avoid this. At which point of the algorithm does this happen? During monotone polygon triangulation?
        // see https://www.desmos.com/calculator/stf8nkndr7
        // this will provoke intersecting edges where they actually intersect!
        verify_triangulation(&liv_from_array(&[
            [4.5899906, 0.0],
            [0.7912103, 0.7912103],
            [-4.2923173e-8, 0.9819677],
            [-1.2092295, 1.2092295],
            [-0.835097, -7.30065e-8],
        ]));
    }*/

    #[test]
    fn numerical_hell_9() {
        verify_triangulation(&liv_from_array(&[
            [1.877369, 0.0],
            [0.72744876, 0.912192],
            [-0.037827354, 0.16573237],
            [-1.0770108, 0.51866084],
            [-0.040608216, -0.0195559],
            [-0.3308545, -1.449571],
            [1.1276244, -1.4139954],
        ]));
    }

    #[test]
    fn numerical_hell_10() {
        verify_triangulation(&liv_from_array(&[
            [0.8590163, 0.0],
            [0.52688754, 0.52688754],
            [-3.721839e-8, 0.8514575],
            [-0.41275758, 0.41275758],
            [-0.13604999, -1.1893867e-8],
            [-0.45389745, -0.4538976],
            [1.8924045e-9, -0.15869379],
            [0.28799793, -0.28799775],
        ]));
    }

    
    /*
    /// This is effective to find special examples where the triangulation fails
    /// You might want to increase the number of iterations to >= 1000000 and adjust
    /// the random_star parameters to find nastier examples
    #[test]
    fn sweep_fuzz() {
        for _ in 1..100000 {
            let vec2s =
                IndexedVertex2D::from_vector(random_star::<Vec2>(5, 10, f32::EPS, 1.0).collect());

            println!(
                "vec2s: {:?}",
                vec2s.iter().map(|v| [v.vec.x, v.vec.y]).collect::<Vec<_>>()
            );

            verify_triangulation(&vec2s);
        }
    }
    */
}
