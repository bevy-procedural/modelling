use crate::{
    math::{Scalar, TransformTrait, Transformable, Vector},
    mesh::{
        cursor::*, DefaultEdgePayload, DefaultFacePayload, MeshImport, MeshType3D,
        MeshTypeHalfEdge, TransformableMesh,
    },
    operations::MeshLoft,
    primitives::{MakePrismatoid, MakeSphere},
};

use super::{MeshExtrude, MeshSubdivision};

/// Extrude operations for meshes.
pub trait DebuggingConstruction<T: MeshTypeHalfEdge<Mesh = Self>>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Generates a new mesh with spheres for vertices and cylinders for edges.
    fn build_cage_mesh(&self, sphere_radius: T::S, cylinder_radius: T::S) -> Self
    where
        T: MeshType3D<Mesh = Self>,
        Self: MakeSphere<T>
            + MeshLoft<T>
            + MeshExtrude<T>
            + MeshSubdivision<T>
            + MakePrismatoid<T>
            + MeshImport<T>
            + MakePrismatoid<T>,
        T::Mesh: TransformableMesh<3, T>,
        T::VP: Transformable<3, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::EP: Transformable<3, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::FP: Transformable<3, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::MP: Transformable<3, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        // TODO: Target MeshType may be different from original
        // TODO: Generalize to nd

        let mut mesh = Self::empty();

        // TODO: Self::icosphere(sphere_radius, 4);
        let sphere = Self::fake_uv_sphere(sphere_radius, 6, 8);

        // TODO: Self::cylinder(cylinder_radius, <T::S as Scalar>::ONE, 8);
        let mut cylinder = Self::regular_frustum(
            cylinder_radius,
            cylinder_radius,
            <T::S as Scalar>::ONE,
            8,
            false,
        );
        cylinder.transform(&T::Trans::from_rotation_arc(
            T::Vec::from_xy(T::S::ZERO, T::S::ONE),
            T::Vec::from_x(T::S::ONE),
        ));

        for v in self.vertices() {
            let pos = v.pos();
            mesh.insert_transformed_mesh(&sphere, &T::Trans::from_translation(pos));
        }

        for e in self.edges() {
            let v0 = e.fork().origin().unwrap().pos();
            let v1 = e.target().unwrap().pos();
            mesh.insert_transformed_mesh(&cylinder, &T::Trans::through_2_points(v0, v1));
            
        }

        mesh
    }
}
