use super::{Face, Mesh, Payload};
use crate::{
    math::Vector3D,
    representation::{tesselate::sweep::chain::SweepReflexChainDirection, IndexType},
};
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
    ) where
        P::Vec: Vector3D<P::S>,
    {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));

        let vec2s: Vec<_> = self
            .vertices_2d::<V, P>(mesh)
            .enumerate()
            .map(|(i, (p, _))| IndexedVertexPoint::new(p, i))
            .collect();

        let mut event_queue: Vec<EventPoint<P::Vec2, P::S>> = Vec::new();
        for here in 0..vec2s.len() {
            event_queue.push(EventPoint::new::<V>(here, &vec2s));
        }
        event_queue.sort_unstable();

        // sweep line status indexed by x-coordinate
        let mut sls = SweepLineStatus::new();

        while let Some(event) = event_queue.pop() {
            #[cfg(feature = "sweep_debug_print")]
            println!("###### {:?} {}", event.vertex_type, event.here.index);
            match event.vertex_type {
                VertexType::Start => {
                    sls.insert(IntervalData {
                        helper: event.here,
                        left: EdgeData::new(event.here, event.next),
                        right: EdgeData::new(event.here, event.prev),
                        stacks: SweepReflexChain::single(event.here),
                        fixup: None,
                    });
                }
                VertexType::Merge => {
                    // left and right are swapped because "remove_right" will get the left one _from_ the right (and vice versa)
                    let left = sls.remove_right(event.here).unwrap();
                    let mut right = sls.remove_left(event.here).unwrap();
                    assert!(left != right, "Mustn't be the same to merge them");
                    let mut new_stacks = if let Some(mut fixup) = left.fixup {
                        #[cfg(feature = "sweep_debug_print")]
                        println!("fixup merge l: {:?}", fixup);
                        fixup.right::<V, P>(event.here, indices, &vec2s);
                        assert!(fixup.is_done());
                        left.stacks
                    } else {
                        left.stacks
                    };
                    let mut new_fixup = if let Some(fixup) = right.fixup {
                        #[cfg(feature = "sweep_debug_print")]
                        println!("fixup merge r: {:?}", fixup);
                        right.stacks.left::<V, P>(event.here, indices, &vec2s);
                        assert!(right.stacks.is_done());
                        fixup
                    } else {
                        right.stacks
                    };
                    sls.insert(IntervalData {
                        helper: event.here,
                        left: left.left,
                        right: right.right,
                        stacks: {
                            new_stacks.right::<V, P>(event.here, indices, &vec2s);
                            new_stacks
                        },
                        fixup: Some({
                            new_fixup.left::<V, P>(event.here, indices, &vec2s);
                            new_fixup
                        }),
                    });
                }
                VertexType::Regular => {
                    // TODO: modify instead of remove
                    if let Some(mut v) = sls.remove_left(event.here) {
                        let mut stacks = if let Some(fixup) = v.fixup {
                            #[cfg(feature = "sweep_debug_print")]
                            println!("fixup regular l: {:?}", fixup);
                            v.stacks.left::<V, P>(event.here, indices, &vec2s);
                            assert!(v.stacks.is_done());
                            fixup
                        } else {
                            v.stacks
                        };
                        sls.insert(IntervalData {
                            helper: event.here,
                            left: EdgeData::new(event.here, event.next),
                            right: v.right,
                            stacks: {
                                stacks.left::<V, P>(event.here, indices, &vec2s);
                                stacks
                            },
                            fixup: None,
                        })
                    } else if let Some(mut v) = sls.remove_right(event.here) {
                        if let Some(mut fixup) = v.fixup {
                            #[cfg(feature = "sweep_debug_print")]
                            println!("fixup regular r: {:?}", fixup);
                            fixup.right::<V, P>(event.here, indices, &vec2s);
                            assert!(fixup.is_done());
                        }
                        sls.insert(IntervalData {
                            helper: event.here,
                            left: v.left,
                            right: EdgeData::new(event.here, event.prev),
                            stacks: {
                                v.stacks.right::<V, P>(event.here, indices, &vec2s);
                                v.stacks
                            },
                            fixup: None,
                        })
                    } else {
                        panic!("Regular vertex not found in sweep line status");
                    }
                }
                VertexType::Split => {
                    let i = *sls
                        .find_by_position::<V, P>(&vec2s[event.here].vec, &vec2s)
                        .unwrap()
                        .0;
                    let line = sls.remove_left(i).unwrap();

                    if let Some(_fixup) = line.fixup {
                        todo!("Handle fixup");
                    }

                    sls.insert(IntervalData {
                        helper: event.here,
                        left: line.left,
                        right: EdgeData::new(event.here, event.prev),
                        stacks: {
                            let mut x = line.stacks.clone();
                            x.right::<V, P>(event.here, indices, &vec2s);
                            x
                        },
                        fixup: None,
                    });

                    let stacks = if line.stacks.direction() == SweepReflexChainDirection::Right {
                        let mut x = SweepReflexChain::single(line.helper);
                        x.left::<V, P>(event.here, indices, &vec2s);
                        x
                    } else {
                        let mut x = SweepReflexChain::single(line.stacks.first());
                        x.left::<V, P>(event.here, indices, &vec2s);
                        x
                    };
                    sls.insert(IntervalData {
                        helper: event.here,
                        left: EdgeData::new(event.here, event.next),
                        right: line.right,
                        stacks,
                        fixup: None,
                    });
                }
                VertexType::End => {
                    let mut line = sls.remove_left(event.here).unwrap();

                    if let Some(_fixup) = line.fixup {
                        todo!("Handle fixup");
                    }

                    line.stacks.left::<V, P>(event.here, indices, &vec2s);
                    assert!(line.stacks.is_done());
                }
            }

            #[cfg(feature = "sweep_debug_print")]
            println!("{}", sls);
        }
    }
}
