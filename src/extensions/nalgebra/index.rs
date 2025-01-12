use crate::math::IndexType;
use num_traits::Zero;

/// Macro to generate similar types and their implementations with a customizable inner type.
macro_rules! define_index_type {
    ($name:ident, $inner:ty, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Default)]
        pub struct $name($inner);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Add<$name> for $name {
            type Output = Self;

            fn add(self, rhs: $name) -> Self {
                $name(self.0 + rhs.0)
            }
        }

        impl Zero for $name {
            fn zero() -> Self {
                $name(0 as $inner)
            }

            fn is_zero(&self) -> bool {
                self.0 == 0
            }

            fn set_zero(&mut self) {
                self.0 = 0 as $inner;
            }
        }

        impl IndexType for $name {
            fn index(&self) -> usize {
                self.0 as usize
            }

            fn max() -> Self {
                $name(<$inner>::MAX)
            }

            fn new(x: usize) -> Self {
                $name(x as $inner)
            }
        }
    };
}

define_index_type!(EU, usize, "An usize edge type.");
define_index_type!(VU, usize, "An usize vertex type.");
define_index_type!(FU, usize, "An usize face type.");

define_index_type!(E64, u64, "A 64-bit edge type.");
define_index_type!(V64, u64, "A 64-bit vertex type.");
define_index_type!(F64, u64, "A 64-bit face type.");

define_index_type!(E32, u32, "A 32-bit edge type.");
define_index_type!(V32, u32, "A 32-bit vertex type.");
define_index_type!(F32, u32, "A 32-bit face type.");

define_index_type!(E16, u16, "A 16-bit edge type.");
define_index_type!(V16, u16, "A 16-bit vertex type.");
define_index_type!(F16, u16, "A 16-bit face type.");
