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
            min_size: px_to_gl_v3(other.minimum),
            max_size: px_to_gl_v3(other.maximum),
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
        let x = m.x_axis();
        let y = m.y_axis();
        let z = m.z_axis();
        assert!(
            x.is_normalized() && y.is_normalized() && z.is_normalized(),
            "All axis have to be of length 1"
        );
        assert!(x.w() == 0.0 && y.w() == 0.0 && z.w() == 0.0, "Unable to extract the rotation matrix because one of the W components of the axis wasn't 0.0");

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
    use glam::f32::{deg, rad, Mat4, Quat, Vec3, Vec4};
    fn compare_vec4(v1: &Vec4, v2: &nalgebra_glm::Vec4) -> bool {
        let cmp = |f1, f2| f32::abs(f1 - f2) <= std::f32::EPSILON;
        cmp(v1.x(), v2.x) && cmp(v1.y(), v2.y) && cmp(v1.z(), v2.z) && cmp(v1.w(), v2.w)
    }
    fn compare_mat(m1: &Mat4, m2: &nalgebra_glm::Mat4) -> bool {
        compare_vec4(&m1.x_axis(), &m2.column(0).into())
    }
    #[test]
    fn isometry() {
        let rot_z = Mat4::from_rotation_z(deg(40.0));
        let rot_y = Mat4::from_rotation_y(deg(30.0));
        let rot_x = Mat4::from_rotation_x(deg(20.0));
        let rot = rot_y * rot_x * rot_z;
        let trans = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let m = trans * rot;

        let iso = Isometry::from_mat4(&m);
        assert!(iso.translation == trans);
        assert!(iso.rotation == rot);

        let rot_z = nalgebra_glm::rotate_z(&nalgebra_glm::identity(), f32::to_radians(40.0));
        let rot_y = nalgebra_glm::rotate_y(&nalgebra_glm::identity(), f32::to_radians(30.0));
        let rot_x = nalgebra_glm::rotate_x(&nalgebra_glm::identity(), f32::to_radians(20.0));
        let rot = rot_y * rot_x * rot_z;

        let trans = nalgebra_glm::translate(
            &nalgebra_glm::identity(),
            &nalgebra_glm::vec3(1.0, 2.0, 3.0),
        );
        let m_nal = trans * rot;
        assert!(compare_mat(&m, &m_nal));

        // Isometry tests

        let q = Quat::from_rotation_y(rad(0.2)) * Quat::from_rotation_x(rad(0.1));
        let q_nal = nalgebra::UnitQuaternion::from_axis_angle(&nalgebra::Vector3::y_axis(), 0.2)
            * nalgebra::UnitQuaternion::from_axis_angle(&nalgebra::Vector3::x_axis(), 0.1);

        //let q = Quat::from_rotation_ypr(rad(0.3), rad(0.2), rad(0.1));
        let transform = Mat4::from_rotation_translation(q, Vec3::new(1.0, 2.0, 3.0));

        let transform_nal =
            nalgebra::Isometry3::from_parts(nalgebra_glm::vec3(1.0, 2.0, 3.0).into(), q_nal);

        let transform_nal_m = transform_nal.into();
        assert!(compare_mat(&transform, &transform_nal_m));
    }
}
