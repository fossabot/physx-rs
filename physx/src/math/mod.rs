// Author: Tom Olsson <tom.olsson@embark-studios.com>
// Copyright © 2019, Embark Studios, all rights reserved.
// Created: 17 June 2019

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

/*!

*/

use super::transform::*;
use glam::{Vec3, Vec2, Vec4};
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
