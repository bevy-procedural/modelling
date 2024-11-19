use crate::{
    math::{HasPosition, HasZero, Scalar, Transformable, Vector},
    mesh::{EdgeBasics, MeshType, VertexBasics},
};

/// The type of curve that the edge represents.
#[derive(Clone, Default, Copy, Debug, PartialEq, Hash)]
pub enum CurvedEdgeType<T: MeshType> {
    /// A linear edge
    #[default]
    Linear,
    /// A quadratic bezier edge
    QuadraticBezier(T::Vec),
    /// A cubic bezier edge
    CubicBezier(T::Vec, T::Vec),
}

impl<T: MeshType> CurvedEdgeType<T>
where
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Vec: Transformable<S = T::S>,
{
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
pub trait CurvedEdge<T: MeshType<Edge = Self>>: EdgeBasics<T>
where
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Vec: Transformable<S = T::S>,
{
    /// Returns the curve type of the edge
    fn curve_type(&self) -> CurvedEdgeType<T>;

    /// Overwrites the curve type of the edge
    fn set_curve_type(&mut self, curve_type: CurvedEdgeType<T>);

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
        fn recursive_flatten<T: MeshType>(
            curve: &CurvedEdgeType<T>,
            edge: &T::Edge,
            mesh: &T::Mesh,
            t0: T::S,
            t1: T::S,
            error: T::S,
            lines: &mut Vec<T::Vec>,
        ) where
            T::Edge: CurvedEdge<T>,
            T::VP: HasPosition<T::Vec, S = T::S>,
            T::Vec: Transformable<S = T::S>,
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

    // TODO: Add analytic version for quad bezier https://raphlinus.github.io/graphics/curves/2019/12/23/flatten-quadbez.html

    /// Converts the curved edge to a sequence of line segments with a specific mean squared error at the midpoints of the generated segments
    fn flatten_uniform_mse(&self, mse: T::S, mesh: &T::Mesh) -> Vec<T::Vec> {
        // TODO: This is not very efficient, but it works for now

        // We start with doubling the number of segments until the mse is below the threshold and then do a binary search
        let mut num_segments = 1;
        let mut mse_current = self.mse_uniform(num_segments, mesh);
        while mse_current > mse {
            num_segments *= 2;
            mse_current = self.mse_uniform(num_segments, mesh);
        }

        let mut num_segments_low = num_segments / 2;
        let mut num_segments_high = num_segments;
        while num_segments_high - num_segments_low > 1 {
            let num_segments_mid = (num_segments_low + num_segments_high) / 2;
            let mse_mid = self.mse_uniform(num_segments_mid, mesh);
            if mse_mid < mse {
                num_segments_high = num_segments_mid;
            } else {
                num_segments_low = num_segments_mid;
            }
        }

        if self.mse_uniform(num_segments_low, mesh) < mse {
            self.flatten_uniform(num_segments_low, mesh)
        } else {
            self.flatten_uniform(num_segments_high, mesh)
        }
    }

    /// Returns the mean squared error of the midpoints of the line segments
    fn mse(&self, lines: &Vec<T::Vec>, mesh: &T::Mesh) -> T::S {
        let mut lines2 = Vec::new();
        lines2.push(self.origin(mesh).pos());
        lines2.extend(lines.iter().cloned());
        lines2.push(self.target(mesh).pos());

        Scalar::stable_mean((0..lines2.len() - 1).into_iter().map(|i| {
            let mid = lines2[i].lerped(&lines2[i + 1], T::S::HALF);
            let point = self.curve_type().point_at(
                self,
                mesh,
                T::S::from_usize(2 * i + 1) / T::S::from_usize(2 * lines2.len()),
            );
            mid.distance_squared(&point) as T::S
        }))
    }

    /// Returns the mean squared error of the midpoints of the line segments when uniformly spaced
    fn mse_uniform(&self, num_segments: usize, mesh: &T::Mesh) -> T::S {
        if num_segments == 0 {
            return T::S::INFINITY;
        }
        self.mse(&self.flatten_uniform(num_segments, mesh), mesh)
    }
}