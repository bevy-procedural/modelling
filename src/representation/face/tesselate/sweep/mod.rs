use super::{Face, Mesh, Payload};
use crate::{
    math::{Scalar, Vector, Vector2D, Vector3D},
    representation::{tesselate::sweep::chain::SweepReflexChainDirection, IndexType},
};
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};
mod chain;
mod point;
mod status;
use chain::SweepReflexChain;
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

        //TODO: get rid of the hashmap by using better indices. Also, reduce memory footprint of data in the queue and tree by indexing into this!
        let mut vec2s = HashMap::new();

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

            vec2s.insert(
                here.1 .1,
                IndexedVertexPoint::new(here.1 .0, here.0, here.1 .1),
            );
        }

        // sweep line status indexed by x-coordinate
        let mut sls: SweepLineStatus<V, P::Vec2, P::S> = SweepLineStatus::new();

        while let Some(event) = event_queue.pop() {
            println!("###### {:?} {}", event.vertex_type, event.here.index);
            match event.vertex_type {
                VertexType::Start => {
                    sls.insert(IntervalData {
                        helper: event.here,
                        left: EdgeData::new(event.here, event.next),
                        right: EdgeData::new(event.here, event.prev),
                        stacks: SweepReflexChain::<V>::single(event.here.index),
                        fixup: None,
                    });
                }
                VertexType::Merge => {
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
                        stacks: left.stacks.right::<P>(event.here.index, indices, &vec2s),
                        fixup: Some(right.stacks.left::<P>(event.here.index, indices, &vec2s)),
                    });
                }
                VertexType::Regular => {
                    // TODO: modify instead of remove
                    if let Some(mut v) = sls.remove_left(&event.here.index) {
                        let mut stacks = if let Some(mut fixup) = v.fixup {
                            println!("fixup regular l: {:?}", fixup);
                            v.stacks.left::<P>(event.here.index, indices, &vec2s);
                            assert!(v.stacks.is_done());
                            fixup
                        } else {
                            v.stacks
                        };
                        sls.insert(IntervalData {
                            helper: event.here,
                            left: EdgeData::new(event.here, event.next),
                            right: v.right,
                            stacks: stacks.left::<P>(event.here.index, indices, &vec2s),
                            fixup: None,
                        })
                    } else if let Some(mut v) = sls.remove_right(&event.here.index) {
                        if let Some(mut fixup) = v.fixup {
                            println!("fixup regular r: {:?}", fixup);
                            fixup.right::<P>(event.here.index, indices, &vec2s);
                            assert!(fixup.is_done());
                        }
                        sls.insert(IntervalData {
                            helper: event.here,
                            left: v.left,
                            right: EdgeData::new(event.here, event.prev),
                            stacks: v.stacks.right::<P>(event.here.index, indices, &vec2s),
                            fixup: None,
                        })
                    } else {
                        panic!("Regular vertex not found in sweep line status");
                    }
                }
                VertexType::Split => {
                    let i = *sls.find_by_position(&event.here.vec).unwrap().0;
                    let line = sls.remove_left(&i).unwrap();

                    if let Some(mut fixup) = line.fixup {
                        todo!("Handle fixup");
                        fixup.left::<P>(event.here.index, indices, &vec2s);
                        assert!(fixup.is_done());
                    }

                    sls.insert(IntervalData {
                        helper: event.here,
                        left: line.left,
                        right: EdgeData::new(event.here, event.prev),
                        stacks: line
                            .stacks
                            .clone()
                            .right::<P>(event.here.index, indices, &vec2s),
                        fixup: None,
                    });

                    let stacks = if line.stacks.direction() == SweepReflexChainDirection::Right {
                        SweepReflexChain::<V>::single(line.helper.index).left::<P>(
                            event.here.index,
                            indices,
                            &vec2s,
                        )
                    } else {
                        SweepReflexChain::<V>::single(line.stacks.first()).left::<P>(
                            event.here.index,
                            indices,
                            &vec2s,
                        )
                    };
                    sls.insert(IntervalData {
                        helper: event.here,
                        left: EdgeData::new(event.here, event.next),
                        right: line.right,
                        stacks,
                        fixup: None,
                    });

                    if event.here.index == V::new(3) {
                        // break;
                    }
                }
                VertexType::End => {
                    let mut line = sls.remove_left(&event.here.index).unwrap();

                    if let Some(mut fixup) = line.fixup {
                        todo!("Handle fixup");
                        fixup.left::<P>(event.here.index, indices, &vec2s);
                        assert!(fixup.is_done());
                    }

                    line.stacks.left::<P>(event.here.index, indices, &vec2s);
                    assert!(line.stacks.is_done());
                }
            }
            //println!("{}", sls);
        }
    }
}
