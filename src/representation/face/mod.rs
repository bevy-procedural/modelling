use crate::representation::payload::{Transform, Vector3D};

use super::{
    payload::{Payload, Scalar, Vector},
    Deletable, HalfEdge, IndexType, Mesh,
};
mod iterator;
mod tesselate;
use itertools::Itertools;

/// A face in a mesh.
///
/// If you want to handle a non-orientable mesh, you have to use double covering.
///
/// Also, if you have inner components, you have to use multiple faces!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Face<EdgeIndex, FaceIndex>
where
    EdgeIndex: IndexType,
    FaceIndex: IndexType,
{
    /// the index of the face
    id: FaceIndex,

    /// a half-edge incident to the face (outer component)
    edge: EdgeIndex,
    // No! We don't have i
    // a half-edge incident to each inner component of the face
    // inner_components: Vec<EdgeIndex>,
}

impl<E: IndexType, F: IndexType> Face<E, F> {
    /// Returns the index of the face.
    #[inline(always)]
    pub fn id(&self) -> F {
        self.id
    }

    /// Returns a half-edge incident to the face.
    #[inline(always)]
    pub fn edge<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> HalfEdge<E, V, F> {
        *mesh.edge(self.edge)
    }

    /// Returns the id of a half-edge incident to the face.
    #[inline(always)]
    pub fn edge_id(&self) -> E {
        self.edge
    }

    /// Creates a new face.
    pub fn new(edge: E) -> Self {
        assert!(edge != IndexType::max());
        Self {
            id: IndexType::max(),
            edge,
        }
    }

    fn vertices_crossed<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> impl Iterator<Item = P::Vec> + 'a + Clone + ExactSizeIterator {
        self.vertices(mesh)
            .circular_tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| (*b.vertex() - *a.vertex()).cross(&(*c.vertex() - *a.vertex())))
    }

    /// Whether the face is convex. Ignores order.
    pub fn is_convex<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> bool {
        // TODO: is this correct?
        // TODO: collinear points cause problems
        self.vertices_crossed(mesh)
            .circular_tuple_windows::<(_, _)>()
            .map(|w| w.0.dot(&w.1).is_positive())
            .all_equal()
    }

    /// Whether the face is planar.
    pub fn is_planar<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>, eps: P::S) -> bool {
        if P::Vec::dimensions() <= 2 {
            return true;
        }

        // TODO: Ignoring planar-ness for now
        return true;

        // TODO: is this correct?
        // TODO: collinear points cause problems

        let three: Vec<_> = self.vertices(mesh).take(3).map(|v| *v.vertex()).collect();
        let n = (three[1] - three[0]).cross(&(three[2] - three[0]));

        self.vertices(mesh).skip(2).all(|v| {
            let v = *v.vertex();
            (v - three[0]).dot(&n).abs() < eps
        })
    }

    /// Whether the face is planar.
    pub fn is_planar2<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> bool {
        self.is_planar(mesh, P::S::EPS * 10.0.into())
    }

    /// A fast methods to get the surface normal, but will only work for convex faces.
    pub fn normal_naive<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> P::Vec {
        assert!(self.is_planar2(mesh));
        assert!(self.is_convex(mesh));

        let three: Vec<_> = self.vertices(mesh).take(3).map(|v| *v.vertex()).collect();
        (three[1] - three[0]).cross(&(three[2] - three[0]))
    }

    /// Get the normal of the face. Assumes the face is planar.
    /// Uses Newell's method to handle concave faces.
    pub fn normal<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> P::Vec
    where
        P::Vec: Vector3D<P::S>,
    {
        // TODO: overload this in a way that allows different dimensions

        assert!(self.is_planar2(mesh));

        let mut normal = P::Vec::zero();
        for (a, b) in self
            .vertices(mesh)
            .map(|v| *v.vertex())
            .circular_tuple_windows::<(_, _)>()
        {
            normal += P::Vec::from_xyz(
                (a.z() + b.z()) * (b.y() - a.y()),
                (a.x() + b.x()) * (b.z() - a.z()),
                (a.y() + b.y()) * (b.x() - a.x()),
            );
        }

        normal * P::Vec::from_xyz(P::S::from(-0.5), P::S::from(-0.5), P::S::from(-0.5))
    }

    /*
    /// Get the normal of the face. Assumes the face is planar.
    pub fn vertices_2d<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, usize, F, P>,
    ) -> impl Iterator<Item = P::Vec> + Clone + ExactSizeIterator + 'a
    where
        P::Vec: Vector2D<P::S>,
    {
        assert!(self.is_planar2(mesh));
        assert!(P::Vec::dimensions() == 2);
        self.vertices(mesh).map(|v| *v.vertex())
    }*/

    /// Get the normal of the face. Assumes the face is planar.
    pub fn vertices_2d<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> impl Iterator<Item = (<P::Vec as Vector<P::S>>::Vec2D, V)> + Clone + ExactSizeIterator + 'a
    where
        P::Vec: Vector3D<P::S>,
    {
        // TODO: overload this in a way that allows different dimensions

        assert!(self.is_planar2(mesh));
        assert!(P::Vec::dimensions() == 3);

        let normal = self.normal(mesh);
        let z_axis = P::Vec::from_xyz(0.0.into(), 0.0.into(), 1.0.into());
        let rotation = <P::Vec as Vector<P::S>>::Transform::from_rotation_arc(
            normal.normalize(),
            z_axis.normalize(),
        );
        self.vertices(mesh)
            .map(move |v| (rotation.apply(*v.vertex()).xy(), v.id()))
    }
}

impl<E: IndexType, F: IndexType> std::fmt::Display for Face<E, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}) {}", self.id().index(), self.edge.index(),)
    }
}

impl<E: IndexType, F: IndexType> Deletable<F> for Face<E, F> {
    fn delete(&mut self) {
        assert!(self.id != IndexType::max(), "Face is already deleted");
        self.id = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: F) {
        assert!(self.id == IndexType::max());
        assert!(id != IndexType::max());
        self.id = id;
    }
}

impl<E: IndexType, F: IndexType> Default for Face<E, F> {
    /// Creates a deleted face
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
        }
    }
}
