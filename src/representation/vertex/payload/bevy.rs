use super::{Payload, Vector, Vector3D};
use bevy::math::Vec3;

impl Vector<f32> for Vec3 {
    fn zero() -> Self {
        Vec3::ZERO
    }

    fn dimensions() -> usize {
        3
    }

    fn distance(&self, other: &Self) -> f32 {
        Vec3::distance(*self, *other)
    }
}

impl Vector3D<f32> for Vec3 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn z(&self) -> f32 {
        self.z
    }

    fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Vec3::new(x, y, z)
    }
}

impl Payload for Vec3 {
    type S = f32;
}
