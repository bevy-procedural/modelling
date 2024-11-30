use crate::math::Scalar;

/// A generic transformation matrix with N elements.
#[derive(Clone, Copy)]
pub struct TransformN<const N: usize, T: Scalar> {
    data: [[T; N]; N],
}

impl<const N: usize, S: Scalar> TransformN<N, S> {
    pub fn new(data: [[S; N]; N]) -> Self {
        Self { data }
    }

    pub fn as_ref(&self) -> &[[S; N]; N] {
        &self.data
    }

    pub fn as_mut(&mut self) -> &mut [[S; N]; N] {
        &mut self.data
    }
}

impl<const N: usize, S: Scalar> std::ops::Index<(usize, usize)> for TransformN<N, S> {
    type Output = S;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<const N: usize, S: Scalar> std::ops::IndexMut<(usize, usize)> for TransformN<N, S> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

