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
        let mut sls: BTreeMap<OrderedFloats<P::S>, IntervalData<V, P::Vec2, P::S>> =
            BTreeMap::new();

        while let Some(event) = event_queue.pop() {
            match event.vertex_type {
                VertexType::Start => {
                    println!("Start {}", event.here.index);
                    sls.insert(
                        OrderedFloats::new(event.here.vec.x()),
                        IntervalData {
                            lowest: Some(event.here),
                            left: EdgeData::new(event.here, event.prev),
                            right: EdgeData::new(event.here, event.next),
                        },
                    );
                }
                VertexType::End => {
                    println!("End {}", event.here.index);
                }
                VertexType::Split => {
                    println!("Split {}", event.here.index);
                    // Find and handle the left neighbor in sweep line status
                    // Insert a diagonal if necessary
                }
                VertexType::Merge => {
                    println!("Merge {}", event.here.index);
                    // TODO
                }
                VertexType::Regular => {
                    println!("Regular {}", event.here.index);
                    // TODO
                }
            }
        }
    }
}
