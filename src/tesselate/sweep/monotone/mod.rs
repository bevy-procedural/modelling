mod dynamic;
mod linear;

pub use dynamic::*;
pub use linear::*;

use crate::{
    math::{IndexType, Vector2D},
    mesh::{IndexedVertex2D, Triangulation},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChainDirection {
    /// The reflex chain is completely on the left
    Left,
    /// The reflex chain is completely on the right
    Right,
    /// The reflex chain consists of the first single item having no preference for a side or is empty
    None,
}

/// While a monotone sub-polygon is being processed, the vertices are stored in this data structure.
/// They will come as two chains, one for the left and one for the right side of the polygon.
/// It doesn't have to store all vertices - it's fine to do all the proccessing in
/// the `left` and `right` functions and not doing anything in `finish`.
pub trait MonotoneTriangulator: Clone + std::fmt::Debug {
    type V: IndexType;
    type Vec2: Vector2D;

    /// Create a new chain with a single value
    fn new(v: usize) -> Self;

    /// Get the first element of the chain (the last inserted vertex)
    fn first(&self) -> usize;

    /// Whether the chain is oriented to the right
    fn is_right(&self) -> bool;

    /// Validate the data structure
    fn sanity_check(&self, left_start: usize, right_start: usize, fixup: &Option<Self>);

    /// Add a new value to the right chain
    fn right(
        &mut self,
        value: usize,
        indices: &mut Triangulation<Self::V>,
        vec2s: &Vec<IndexedVertex2D<Self::V, Self::Vec2>>,
    );

    /// Add a new value to the left chain
    fn left(
        &mut self,
        value: usize,
        indices: &mut Triangulation<Self::V>,
        vec2s: &Vec<IndexedVertex2D<Self::V, Self::Vec2>>,
    );

    /// Finish triangulating the monotone polygon
    fn finish(
        &mut self,
        indices: &mut Triangulation<Self::V>,
        vec2s: &Vec<IndexedVertex2D<Self::V, Self::Vec2>>,
    );
}
