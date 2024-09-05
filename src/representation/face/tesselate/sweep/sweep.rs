use super::{
    chain::{ReflexChain, ReflexChainDirection},
    point::{EventPoint, LocallyIndexedVertex},
    status::{EdgeData, IntervalData, SweepLineStatus},
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
    let mut event_queue: Vec<EventPoint<Vec2>> = Vec::new();
    for here in 0..vec2s.len() {
        event_queue.push(EventPoint::classify::<V>(here, &vec2s));
    }
    event_queue.sort_unstable();

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
        println!("{}", self.sls);
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
        self.sls.insert(IntervalData {
            helper: event.here,
            left: EdgeData::new(event.here, event.next),
            right: EdgeData::new(event.here, event.prev),
            stacks: ReflexChain::single(event.here),
            fixup: None,
        });
    }

    /// Merge two parts of the sweep line at the given event
    fn merge(self: &mut Self, event: &EventPoint<Vec2>) {
        // left and right are swapped because "remove_right" will get the left one _from_ the right (and vice versa)
        let left = self.sls.remove_right(event.here).unwrap();
        let mut right = self.sls.remove_left(event.here).unwrap();

        assert!(left != right, "Mustn't be the same to merge them");

        let mut new_stacks = if let Some(mut fixup) = left.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup merge l: {}", fixup);

            fixup.right(event.here, self.indices, self.vec2s);
            assert!(fixup.is_done());
            left.stacks
        } else {
            left.stacks
        };

        let mut new_fixup = if let Some(fixup) = right.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup merge r: {}", fixup);

            right.stacks.left(event.here, self.indices, self.vec2s);
            assert!(right.stacks.is_done());
            fixup
        } else {
            right.stacks
        };

        self.sls.insert(IntervalData {
            helper: event.here,
            left: left.left,
            right: right.right,
            stacks: {
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
        interval: IntervalData<V, Vec2>,
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
        // TODO: modify sls instead of remove and insert
        if let Some(mut interval) = self.sls.remove_left(event.here) {
            if interval.is_end() {
                self.hidden_end(event, interval, meta, event_i, queue);
                return;
            }

            let mut stacks = if let Some(fixup) = interval.fixup {
                #[cfg(feature = "sweep_debug_print")]
                println!("fixup regular l: {}", fixup);

                interval.stacks.left(event.here, self.indices, self.vec2s);
                assert!(interval.stacks.is_done());
                fixup
            } else {
                interval.stacks
            };
            self.sls.insert(IntervalData {
                helper: event.here,
                left: EdgeData::new(event.here, event.next),
                right: interval.right,
                stacks: {
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
            self.sls.insert(IntervalData {
                helper: event.here,
                left: interval.left,
                right: EdgeData::new(event.here, event.prev),
                stacks: {
                    interval.stacks.right(event.here, self.indices, self.vec2s);
                    interval.stacks
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
            .unwrap()
            .0;
        let line = self.sls.remove_left(i).unwrap();

        let stacks = if let Some(mut fixup) = line.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup split: {}", fixup);

            let t = fixup.first();

            fixup.right(event.here, self.indices, self.vec2s);
            assert!(fixup.is_done());

            let mut x = ReflexChain::single(t);
            x.left(event.here, self.indices, self.vec2s);
            x
        } else if line.stacks.direction() == ReflexChainDirection::Right {
            let mut x = ReflexChain::single(line.helper);
            x.left(event.here, self.indices, self.vec2s);
            x
        } else {
            let mut x = ReflexChain::single(line.stacks.first());
            x.left(event.here, self.indices, self.vec2s);
            x
        };

        self.sls.insert(IntervalData {
            helper: event.here,
            left: line.left,
            right: EdgeData::new(event.here, event.prev),
            stacks: {
                let mut x = line.stacks.clone();
                x.right(event.here, self.indices, self.vec2s);
                x
            },
            fixup: None,
        });

        self.sls.insert(IntervalData {
            helper: event.here,
            left: EdgeData::new(event.here, event.next),
            right: line.right,
            stacks,
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

        line.stacks.left(event.here, self.indices, self.vec2s);
        assert!(line.stacks.is_done());
    }
}
