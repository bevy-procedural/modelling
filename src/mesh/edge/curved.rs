use crate::{
    math::{Scalar, Transformable, Vector},
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
        // TODO: Make this faster. Should work without reference to mesh!
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

    /// Returns if two curves are about equal (control point wise) within a certain epsilon
    pub fn is_about(&self, other: &Self, epsilon: T::S) -> bool {
        match (self, other) {
            (CurvedEdgeType::Linear, CurvedEdgeType::Linear) => true,
            (CurvedEdgeType::QuadraticBezier(c1), CurvedEdgeType::QuadraticBezier(c2)) => {
                c1.is_about(c2, epsilon)
            }
            (CurvedEdgeType::CubicBezier(c1, c2), CurvedEdgeType::CubicBezier(c3, c4)) => {
                c1.is_about(c3, epsilon) && c2.is_about(c4, epsilon)
            }
            _ => false,
        }
    }

    /// Approximates the Fréchet distance between the edge and a line segment.
    pub fn frechet_distance(
        &self,
        line_start: T::Vec,
        line_end: T::Vec,
        t0: T::S,
        t1: T::S,
        edge: &T::Edge,
        mesh: &T::Mesh,
        samples: usize,
        max_search: usize,
    ) -> T::S {
        let mut max_dist = T::S::ZERO;
        let mut bezier_pos = t0;

        // TODO: this is a performance bottleneck. Optimizing this would be great.

        for i in 1..=samples {
            // only consider inner positions
            let s = T::S::from_usize(i) / T::S::from_usize(samples + 1);
            let line_point = line_start.lerped(&line_end, s);

            let mut min_distance = T::S::INFINITY;

            // make a good first guess for the step size
            let mut step_size = (t1 - t0) / T::S::from_usize(samples);
            let mut t = bezier_pos;

            // walk forward while it gets smaller
            while t <= t1 {
                let distance = self.point_at(edge, mesh, t).distance_squared(&line_point);
                if distance >= min_distance {
                    break;
                }
                min_distance = distance;
                t += step_size;
            }

            // binary search for the local minimum around t
            for _ in 0..max_search {
                let t1 = (t + step_size).min(t1);
                let t2 = (t - step_size).max(t0);
                let d1 = self.point_at(edge, mesh, t1).distance_squared(&line_point);
                let d2 = self.point_at(edge, mesh, t2).distance_squared(&line_point);
                if d1 < d2 {
                    t = t1;
                } else {
                    t = t2;
                }
                step_size *= T::S::HALF;
                if step_size <= T::S::EPS {
                    break;
                }
            }

            min_distance = self.point_at(edge, mesh, t).distance(&line_point);

            // safe the new position for later. We cannot walk backwards!
            bezier_pos = t.min(t1);

            if min_distance > max_dist {
                max_dist = min_distance;
            }
        }

        max_dist.sqrt()
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

            /*
            // we shouldn't just test the middle but n points
            // e.g., if the curve is s-shaped, the deviation would be 0 at the middle
            let n = 4;

            // start in the middle, since it is most likely to be the worst point
            let is_acceptable = [2, 1, 3].into_iter().find(|i| {
                let t = t0 + (t1 - t0) * T::S::from_usize(*i) / T::S::from_usize(n);
                let p = curve.point_at(edge, mesh, t);
                let p_line = p0.lerped(&p1, t - t0);
                p.distance(&p_line) > error
            }).is_none();*/

            // TODO: don't hardcode the frechet params
            let is_acceptable = edge
                .curve_type()
                .frechet_distance(p0, p1, t0, t1, edge, mesh, 3, 20)
                < error;

            if is_acceptable {
                // The segment is acceptable; push p1
                lines.push(p1);
            } else {
                // Subdivide further
                let tm = (t0 + t1) / T::S::TWO;
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
