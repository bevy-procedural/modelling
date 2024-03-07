use super::{Face, Mesh, Payload};
use crate::{
    math::{Scalar, Vector, Vector2D, Vector3D},
    representation::{tesselate::sweep::status::VertexSweepStack, IndexType},
};
use itertools::Itertools;
use std::collections::BinaryHeap;
mod point;
mod status;
mod vertex_type;
use point::{EventPoint, IndexedVertexPoint};
use status::{EdgeData, IntervalData, SweepLineStatus};
use vertex_type::VertexType;

// See https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Uses the sweep line triangulation
    pub fn sweep_line<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
        local_indices: bool,
    ) where
        P::Vec: Vector3D<P::S>,
    {
        assert!(!local_indices);
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));

        let vs: Vec<(P::Vec2, V)> = self.vertices_2d::<V, P>(mesh).collect();
        let mut event_queue: BinaryHeap<EventPoint<V, P::Vec2, P::S>> = BinaryHeap::new();
        for (prev, here, next) in vs.iter().enumerate().circular_tuple_windows::<(_, _, _)>() {
            event_queue.push(EventPoint {
                prev: IndexedVertexPoint::new(prev.1 .0, prev.0, prev.1 .1),
                here: IndexedVertexPoint::new(here.1 .0, here.0, here.1 .1),
                next: IndexedVertexPoint::new(next.1 .0, next.0, next.1 .1),
                vertex_type: VertexType::new::<V, P::Vec2, P::S>(
                    prev.1 .0,
                    here.1 .0,
                    next.1 .0,
                    P::S::EPS,
                ),
            });
        }

        // sweep line status indexed by x-coordinate
        let mut sls: SweepLineStatus<V, P::Vec2, P::S> = SweepLineStatus::new();

        while let Some(event) = event_queue.pop() {
            match event.vertex_type {
                VertexType::Start => {
                    println!("******* Start {}", event.here.index);
                    sls.insert(IntervalData {
                        helper: event.here,
                        left: EdgeData::new(event.here, event.next),
                        right: EdgeData::new(event.here, event.prev),
                        stacks: VertexSweepStack::<V>::first(event.here.index),
                        fixup: None,
                    });
                }
                VertexType::Merge => {
                    println!("******* Merge {}", event.here.index);

                    // left and right are swapped because "remove_right" will get the left one _from_ the right (and vice versa)
                    let mut left = sls.remove_right(&event.here.index).unwrap();
                    let mut right: IntervalData<V, <P as Payload>::Vec2, <P as Payload>::S> =
                        sls.remove_left(&event.here.index).unwrap();
                    assert!(left != right, "Mustn't be the same to merge them");
                    if let Some(fixup) = left.fixup {
                        todo!("Handle fixup");
                    }
                    if let Some(fixup) = right.fixup {
                        todo!("Handle fixup");
                    }
                    sls.insert(IntervalData {
                        helper: event.here,
                        left: left.left,
                        right: right.right,
                        stacks: left.stacks.right(event.here.index, indices),
                        fixup: Some(right.stacks.left(event.here.index, indices)),
                    });
                }
                VertexType::Regular => {
                    println!("******* Regular {}", event.here.index);

                    // TODO: modify instead of remove
                    if let Some(mut v) = sls.remove_left(&event.here.index) {
                        if let Some(mut fixup) = v.fixup {
                            fixup.left(event.here.index, indices);
                            assert!(fixup.is_done());
                        }
                        sls.insert(IntervalData {
                            helper: event.here,
                            left: EdgeData::new(event.here, event.next),
                            right: v.right,
                            stacks: v.stacks.left(event.here.index, indices),
                            fixup: None,
                        })
                    } else if let Some(mut v) = sls.remove_right(&event.here.index) {
                        if let Some(mut fixup) = v.fixup {
                            fixup.right(event.here.index, indices);
                            assert!(fixup.is_done());
                        }
                        sls.insert(IntervalData {
                            helper: event.here,
                            left: v.left,
                            right: EdgeData::new(event.here, event.prev),
                            stacks: v.stacks.right(event.here.index, indices),
                            fixup: None,
                        })
                    } else {
                        panic!("Regular vertex not found in sweep line status");
                    }
                }
                VertexType::Split => {
                    println!("******* Split {}", event.here.index);
                    let i = *sls.find_by_position(&event.here.vec).unwrap().0;
                    let line = sls.remove_left(&i).unwrap();

                    /*println!(
                        "Insert diagonal from {} to {}",
                        event.here.index, line.helper.index
                    );*/

                    if let Some(mut fixup) = line.fixup {
                        todo!("Handle fixup");
                        fixup.left(event.here.index, indices);
                        assert!(fixup.is_done());
                    }

                    sls.insert(IntervalData {
                        helper: event.here,
                        left: line.left,
                        right: EdgeData::new(event.here, event.prev),
                        stacks: line.stacks.clone().right(event.here.index, indices),
                        fixup: None,
                    });
                    
                    sls.insert(IntervalData {
                        helper: event.here,
                        left: EdgeData::new(event.here, event.next),
                        right: line.right,
                        stacks: VertexSweepStack::<V>::first(line.helper.index).left(event.here.index, indices),
                        fixup: None,
                    });

                }
                VertexType::End => {
                    println!("******* End {}", event.here.index);
                    let mut line = sls.remove_left(&event.here.index).unwrap();

                    if let Some(mut fixup) = line.fixup {
                        todo!("Handle fixup");
                        fixup.left(event.here.index, indices);
                        assert!(fixup.is_done());
                    }

                    line.stacks.left(event.here.index, indices);
                    assert!(line.stacks.is_done());
                }
            }
            // println!("{}", sls);
        }
    }
}
