/// based on petgraph::csr::IndexType;
use std::{fmt, hash::Hash};

/// Trait for the unsigned integer type used for node and edge indices.
pub trait IndexType:
    Copy + Default + Hash + Ord + fmt::Debug + 'static + std::fmt::Display
{
    /// Create a new index from a usize. Panics if the usize is out of range.
    fn new(x: usize) -> Self;
    /// Convert the index to a usize.
    fn index(&self) -> usize;
    /// Return the maximum value of the index type.
    fn max() -> Self;
}

impl IndexType for usize {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x
    }
    #[inline(always)]
    fn index(&self) -> Self {
        *self
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::usize::MAX
    }
}

impl IndexType for u32 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        assert!(x <= ::std::u32::MAX as usize, "Index out of range: {}", x);
        x as u32
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u32::MAX
    }
}

impl IndexType for u16 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        assert!(x <= ::std::u16::MAX as usize, "Index out of range: {}", x);
        x as u16
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u16::MAX
    }
}

impl IndexType for u8 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        assert!(x <= ::std::u8::MAX as usize, "Index out of range: {}", x);
        x as u8
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u8::MAX
    }
}
