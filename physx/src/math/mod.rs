// Author: Tom Olsson <tom.olsson@embark-studios.com>
// Copyright © 2019, Embark Studios, all rights reserved.
// Created: 17 June 2019

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

/*!

*/

use super::transform::*;
use glam::{Mat4, Vec3, Vec4};
use physx_sys::PxBounds3;

pub struct Bounds {
    pub min_size: Vec3,
    pub max_size: Vec3,
}

impl From<PxBounds3> for Bounds {
    fn from(other: PxBounds3) -> Self {
        Self {
            min_size: px_to_na_v3(other.minimum),
            max_size: px_to_na_v3(other.maximum),
        }
    }
}

pub struct Isometry {
    pub translation: Mat4,
    pub rotation: Mat4,
}
impl Isometry {
    /// Extracts the rotation and translation matrix from a matrix.
    pub fn from_mat4(m: &Mat4) -> Self {
        let x = m.x_axis().normalize();
        let y = m.y_axis().normalize();
        let z = m.z_axis().normalize();
        let rotation = Mat4::new(x, y, z, Vec4::unit_w());
        let translation = Mat4::from_translation(m.w_axis().truncate());
        Self {
            rotation,
            translation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Isometry;
    use glam::{deg, Mat4, Vec3};
    #[test]
    fn isometry() {
        let rot = Mat4::from_rotation_y(deg(90.0));
        let trans = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let m = trans * rot;
        let iso = Isometry::from_mat4(&m);
        assert!(iso.translation == trans);
        assert!(iso.rotation == rot);
    }

}
