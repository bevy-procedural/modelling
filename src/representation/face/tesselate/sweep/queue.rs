use super::chain;
use super::point;
use super::status;
use super::vertex_type;
use super::SweepMeta;
use crate::math::{Scalar, Vector2D};
use crate::representation::{tesselate::sweep::chain::ReflexChainDirection, IndexType};
use chain::ReflexChain;
use point::{EventPoint, LocallyIndexedVertex};
use status::{EdgeData, IntervalData, SweepLineStatus};
pub use vertex_type::VertexType;

/// Central event queue of the sweep line triangulation
pub struct SweepEventQueue<Vec2: Vector2D, V: IndexType> {
    /// Sorted event queue
    queue: Vec<EventPoint<Vec2>>,

    /// Indexed vertex points
    /// TODO: do everything with the queue data structure? Can probably avoid a clone or two.
    vec2s: Vec<LocallyIndexedVertex<Vec2>>,

    /// sweep line status indexed by x-coordinate
    sls: SweepLineStatus<V, Vec2>,
}

impl<Vec2: Vector2D, V: IndexType> SweepEventQueue<Vec2, V> {
    /// Creates a new event queue from a list of indexed vertex points
    pub fn new(vec2s: &Vec<LocallyIndexedVertex<Vec2>>) -> Self {
        let mut event_queue: Vec<EventPoint<Vec2>> = Vec::new();
        for here in 0..vec2s.len() {
            println!("here: {:?}", vec2s[here]);
            event_queue.push(EventPoint::new::<V>(here, &vec2s));
        }
        event_queue.sort_unstable();

        return Self {
            queue: event_queue,
            vec2s: vec2s.clone(), // TODO: avoid clone?
            sls: SweepLineStatus::new(),
        };
    }

    pub fn extract_meta(&self) -> SweepMeta {
        SweepMeta {
            vertex_type: self.queue.iter().map(|e| (e.here, e.vertex_type)).collect(),
        }
    }

    /// Processes the next event in the queue. Returns true if there are more events to process.
    pub fn work(self: &mut Self, indices: &mut Vec<V>, meta: &mut SweepMeta) -> bool {
        // TODO: don't pop the queue but just iterate over the vector
        let Some(event) = self.queue.pop() else {
            return false;
        };

        #[cfg(feature = "sweep_debug_print")]
        println!("###### {:?} {}", event.vertex_type, event.here);

        match event.vertex_type {
            VertexType::Start => self.start(&event),
            VertexType::Merge => self.merge(&event, indices),
            VertexType::Regular => self.regular(&event, indices, meta),
            VertexType::Split => self.split(&event, indices),
            VertexType::End => self.end(&event, indices),
            VertexType::Skip => {
                todo!("Skip collinear vertices");
            }
            VertexType::Undefined => {
                panic!("Vertex type is Undefined");
            }
        }

        #[cfg(feature = "sweep_debug_print")]
        println!("{}", self.sls);

        return true;
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
    fn merge(self: &mut Self, event: &EventPoint<Vec2>, indices: &mut Vec<V>) {
        // left and right are swapped because "remove_right" will get the left one _from_ the right (and vice versa)
        let left = self.sls.remove_right(event.here).unwrap();
        let mut right = self.sls.remove_left(event.here).unwrap();

        assert!(left != right, "Mustn't be the same to merge them");

        let mut new_stacks = if let Some(mut fixup) = left.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup merge l: {}", fixup);

            fixup.right(event.here, indices, &self.vec2s);
            assert!(fixup.is_done());
            left.stacks
        } else {
            left.stacks
        };

        let mut new_fixup = if let Some(fixup) = right.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup merge r: {}", fixup);

            right.stacks.left(event.here, indices, &self.vec2s);
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
                new_stacks.right(event.here, indices, &self.vec2s);
                new_stacks
            },
            fixup: Some({
                new_fixup.left(event.here, indices, &self.vec2s);
                new_fixup
            }),
        });
    }

    /// Handle a regular vertex
    fn regular(
        self: &mut Self,
        event: &EventPoint<Vec2>,
        indices: &mut Vec<V>,
        meta: &mut SweepMeta,
    ) {
        // TODO: modify instead of remove
        if let Some(mut interval) = self.sls.remove_left(event.here) {
            if interval.is_end() {
                // There can be end vertices that were undetected because they
                // had the same y-coordinate as other regular vertices.
                // This routine will fix this
                // TODO: assert that the previous vertex has the same y-coordinate

                #[cfg(feature = "sweep_debug_print")]
                println!("Reinterpret as end");

                #[cfg(feature = "sweep_debug")]
                meta.update_type(event.here, VertexType::End);

                self.sls.insert(interval);
                self.end(event, indices);
                return;
            }

            let mut stacks = if let Some(fixup) = interval.fixup {
                #[cfg(feature = "sweep_debug_print")]
                println!("fixup regular l: {}", fixup);

                interval.stacks.left(event.here, indices, &self.vec2s);
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
                    stacks.left(event.here, indices, &self.vec2s);
                    stacks
                },
                fixup: None,
            })
        } else if let Some(mut interval) = self.sls.remove_right(event.here) {
            if interval.is_end() {
                // There can be end vertices that were undetected because they
                // had the same y-coordinate as other regular vertices.
                // This routine will fix this
                // TODO: assert that the previous vertex has the same y-coordinate

                #[cfg(feature = "sweep_debug_print")]
                println!("Reinterpret as end");

                #[cfg(feature = "sweep_debug")]
                meta.update_type(event.here, VertexType::End);

                self.sls.insert(interval);
                self.end(event, indices);
                return;
            }

            if let Some(mut fixup) = interval.fixup {
                #[cfg(feature = "sweep_debug_print")]
                println!("fixup regular r: {}", fixup);

                fixup.right(event.here, indices, &self.vec2s);
                assert!(fixup.is_done());
            }
            self.sls.insert(IntervalData {
                helper: event.here,
                left: interval.left,
                right: EdgeData::new(event.here, event.prev),
                stacks: {
                    interval.stacks.right(event.here, indices, &self.vec2s);
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

            assert!(self.queue.len() > 0);
            let dist = self.queue.last().unwrap().vec.y() - event.vec.y();

            // be generous with the tolerance
            assert!(
                dist.abs() <= Vec2::S::EPS * Vec2::S::from(10.0),
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
    fn split(self: &mut Self, event: &EventPoint<Vec2>, indices: &mut Vec<V>) {
        let i = *self
            .sls
            .find_by_position(&self.vec2s[event.here].vec, &self.vec2s)
            .unwrap()
            .0;
        let line = self.sls.remove_left(i).unwrap();

        let stacks = if let Some(mut fixup) = line.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup split: {}", fixup);

            let t = fixup.first();

            fixup.right(event.here, indices, &self.vec2s);
            assert!(fixup.is_done());

            let mut x = ReflexChain::single(t);
            x.left(event.here, indices, &self.vec2s);
            x
        } else if line.stacks.direction() == ReflexChainDirection::Right {
            let mut x = ReflexChain::single(line.helper);
            x.left(event.here, indices, &self.vec2s);
            x
        } else {
            let mut x = ReflexChain::single(line.stacks.first());
            x.left(event.here, indices, &self.vec2s);
            x
        };

        self.sls.insert(IntervalData {
            helper: event.here,
            left: line.left,
            right: EdgeData::new(event.here, event.prev),
            stacks: {
                let mut x = line.stacks.clone();
                x.right(event.here, indices, &self.vec2s);
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
    fn end(self: &mut Self, event: &EventPoint<Vec2>, indices: &mut Vec<V>) {
        let mut line = self.sls.remove_left(event.here).unwrap();
        assert!(line.is_end());

        if let Some(mut fixup) = line.fixup {
            #[cfg(feature = "sweep_debug_print")]
            println!("fixup end: {}", fixup);

            fixup.right(event.here, indices, &self.vec2s);
            assert!(fixup.is_done());
        }

        line.stacks.left(event.here, indices, &self.vec2s);
        assert!(line.stacks.is_done());
    }
}
