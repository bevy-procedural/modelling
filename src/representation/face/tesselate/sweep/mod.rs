use super::{Face, Mesh, Payload};
use crate::{
    math::{Scalar, Vector, Vector2D, Vector3D},
    representation::IndexType,
};
use itertools::Itertools;
use std::collections::{BTreeMap, BinaryHeap};
mod point;
mod status;
mod vertex_type;
use point::{EventPoint, IndexedVertexPoint};
use status::{EdgeData, IntervalData, OrderedFloats, SweepLineStatus};
use vertex_type::VertexType;

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
                    sls.insert(IntervalData {
                        lowest: Some(event.here),
                        left: EdgeData::new(event.here, event.next),
                        right: EdgeData::new(event.here, event.prev),
                    });

                    println!("Start {}\n{}", event.here.index, sls);
                }
                VertexType::Merge => {
                    // left and right are swapped because "remove_right" will get the left one _from_ the right (and vice versa)
                    let left = sls.remove_right(&event.here.index).unwrap();
                    let right = sls.remove_left(&event.here.index).unwrap();
                    assert!(left != right, "Mustn't be the same to merge them");
                    sls.insert(IntervalData {
                        lowest: Some(event.here),
                        left: left.left,
                        right: right.right,
                    });

                    println!("Merge {}\n{}", event.here.index, sls);
                }
                VertexType::Regular => {
                    // TODO: modify instead of remove
                    if let Some(v) = sls.remove_left(&event.here.index) {
                        todo!("Handle regular vertex")
                    } else if let Some(v) = sls.remove_right(&event.here.index) {
                        sls.insert(IntervalData {
                            lowest: Some(event.here),
                            left: v.left,
                            right: EdgeData::new(event.here, event.prev),
                        })
                    } else {
                        panic!("Regular vertex not found in sweep line status");
                    }

                    println!("Regular {}\n{}", event.here.index, sls);
                }
                VertexType::Split => {
                    let i = *sls.find_by_position(&event.here.vec).unwrap().0;
                    let line = sls.remove_left(&i).unwrap();

                    println!(
                        "Insert diagonal from {} to {}",
                        event.here.index,
                        line.lowest.unwrap().index
                    );

                    /*indices.push(event.here.index);
                    indices.push(line.lowest.unwrap().index);
                    indices.push(event.prev.index);*/

                    sls.insert(IntervalData {
                        lowest: Some(event.here),
                        left: line.left,
                        right: EdgeData::new(event.here, event.prev),
                    });
                    sls.insert(IntervalData {
                        lowest: Some(event.here),
                        left: EdgeData::new(event.here, event.next),
                        right: line.right,
                    });

                    println!("Split {}\n{}", event.here.index, sls);
                }
                VertexType::End => {
                    let line = sls.remove_left(&event.here.index).unwrap();

                    println!("End {}\n{}", event.here.index, sls);
                }
            }
        }
    }
}
