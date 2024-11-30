use crate::{
    math::{HasZero, Scalar, Transformable, Vector},
    mesh::{EdgeBasics, EuclideanMeshType, VertexBasics},
};

/// The type of curve that the edge represents.
#[derive(Clone, Default, Copy, Debug, PartialEq, Hash)]
pub enum CurvedEdgeType<const D: usize, T: EuclideanMeshType<D>> {
    /// A linear edge
    #[default]
    Linear,
    /// A quadratic bezier edge
    QuadraticBezier(T::Vec),
    /// A cubic bezier edge
    CubicBezier(T::Vec, T::Vec),
}

impl<const D: usize, T: EuclideanMeshType<D>> CurvedEdgeType<D, T> {
    /// Returns the coordinates at a specific point on the curve
    /// The parameter `t` is in the range [0, 1]
    pub fn point_at(&self, edge: &T::Edge, mesh: &T::Mesh, t: T::S) -> T::Vec {
        let start: T::Vec = edge.origin(mesh).pos();
        let end: T::Vec = edge.target(mesh).pos();
        let res: T::Vec = match self {
            CurvedEdgeType::Linear => start.lerped(&end, t).clone(),
            CurvedEdgeType::QuadraticBezier(control_point) => {
                let tt = t * t;
                let s = T::S::ONE - t;
                let ss = s * s;
                start * ss + *control_point * T::S::TWO * s * t + end * tt
            }
            CurvedEdgeType::CubicBezier(control_point1, control_point2) => {
                let tt = t * t;
                let ttt = tt * t;
                let s = T::S::ONE - t;
                let ss = s * s;
                let sss = ss * s;
                start * sss
                    + *control_point1 * T::S::THREE * ss * t
                    + *control_point2 * T::S::THREE * s * tt
                    + end * ttt
            }
        };
        return res;
    }
}

/// Edge that can be a line or some type of curve.
pub trait CurvedEdge<const D: usize, T: EuclideanMeshType<D, Edge = Self>>: EdgeBasics<T> {
    /// Returns the curve type of the edge
    fn curve_type(&self) -> CurvedEdgeType<D, T>;

    /// Overwrites the curve type of the edge
    fn set_curve_type(&mut self, curve_type: CurvedEdgeType<D, T>);

    /// Converts the curved edge to a uniformly spaced sequence of `n` line segments
    fn flatten_uniform(&self, n: usize, mesh: &T::Mesh) -> Vec<T::Vec> {
        assert!(n > 0);
        return (0..n - 1)
            .into_iter()
            .map(|i| {
                self.curve_type().point_at(
                    self,
                    mesh,
                    T::S::from_usize(i + 1) / T::S::from_usize(n),
                )
            })
            .collect();
    }

    /// Converts the curved edge to a sequence of line segments with a specific error using De Casteljau's algorithm
    fn flatten_casteljau(&self, error: T::S, mesh: &T::Mesh) -> Vec<T::Vec> {
        fn recursive_flatten<const D: usize, T: EuclideanMeshType<D>>(
            curve: &CurvedEdgeType<D, T>,
            edge: &T::Edge,
            mesh: &T::Mesh,
            t0: T::S,
            t1: T::S,
            error: T::S,
            lines: &mut Vec<T::Vec>,
        ) where
            T::Edge: CurvedEdge<D, T>,
        {
            let p0 = curve.point_at(edge, mesh, t0);
            let p1 = curve.point_at(edge, mesh, t1);
            let tm = (t0 + t1) / T::S::TWO;
            let pm = curve.point_at(edge, mesh, tm);
            let pline = p0.lerped(&p1, T::S::HALF);
            let deviation = pm.distance(&pline);

            if deviation <= error {
                // The segment is acceptable; push p1
                lines.push(p1);
            } else {
                // Subdivide further
                recursive_flatten(curve, edge, mesh, tm, t1, error, lines);
                recursive_flatten(curve, edge, mesh, t0, tm, error, lines);
            }
        }

        let mut lines = Vec::new();
        // Start by adding the target point
        let curve = self.curve_type();
        recursive_flatten(&curve, self, mesh, T::S::ZERO, T::S::ONE, error, &mut lines);
        // Reverse the points to get them in the correct order
        lines.reverse();
        lines.pop();
        return lines;
    }
}
