//! WebAssembly-optimized Geometry Routines
//!
//! Note that this code has been designed around WebAssembly. For native CPU code I always
//! use homogenous coordinates with 4-lane SIMD vectors rather than individual scalar values.

use std::ops::{Add, Div, Mul, Sub};

use wasm_bindgen::prelude::*;

pub mod bindings;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct Matrix4 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,

    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,

    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,

    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}

macro_rules! impl_vec_binary_op {
    ($op:ident::$method:ident for $name:ident) => {
        impl $op for $name {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self {
                $name {
                    x: $op::$method(self.x, rhs.x),
                    y: $op::$method(self.y, rhs.y),
                    z: $op::$method(self.z, rhs.z),
                }
            }
        }

        impl $op<f32> for $name {
            type Output = Self;

            fn $method(self, rhs: f32) -> Self {
                $name {
                    x: $op::$method(self.x, rhs),
                    y: $op::$method(self.y, rhs),
                    z: $op::$method(self.z, rhs),
                }
            }
        }
    };
}

impl_vec_binary_op!(Add::add for Vector3);
impl_vec_binary_op!(Sub::sub for Vector3);
impl_vec_binary_op!(Mul::mul for Vector3);
impl_vec_binary_op!(Div::div for Vector3);

macro_rules! impl_mat_binary_op {
    ($op:ident::$method:ident for $name:ident) => {
        impl $op for $name {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self {
                $name {
                    m00: $op::$method(self.m00, rhs.m00),
                    m01: $op::$method(self.m01, rhs.m01),
                    m02: $op::$method(self.m02, rhs.m02),
                    m03: $op::$method(self.m03, rhs.m03),
                    m10: $op::$method(self.m10, rhs.m10),
                    m11: $op::$method(self.m11, rhs.m11),
                    m12: $op::$method(self.m12, rhs.m12),
                    m13: $op::$method(self.m13, rhs.m13),
                    m20: $op::$method(self.m20, rhs.m20),
                    m21: $op::$method(self.m21, rhs.m21),
                    m22: $op::$method(self.m22, rhs.m22),
                    m23: $op::$method(self.m23, rhs.m23),
                    m30: $op::$method(self.m30, rhs.m30),
                    m31: $op::$method(self.m31, rhs.m31),
                    m32: $op::$method(self.m32, rhs.m32),
                    m33: $op::$method(self.m33, rhs.m33),
                }
            }
        }

        impl $op<f32> for $name {
            type Output = Self;

            fn $method(self, rhs: f32) -> Self {
                $name {
                    m00: $op::$method(self.m00, rhs),
                    m01: $op::$method(self.m01, rhs),
                    m02: $op::$method(self.m02, rhs),
                    m03: $op::$method(self.m03, rhs),
                    m10: $op::$method(self.m10, rhs),
                    m11: $op::$method(self.m11, rhs),
                    m12: $op::$method(self.m12, rhs),
                    m13: $op::$method(self.m13, rhs),
                    m20: $op::$method(self.m20, rhs),
                    m21: $op::$method(self.m21, rhs),
                    m22: $op::$method(self.m22, rhs),
                    m23: $op::$method(self.m23, rhs),
                    m30: $op::$method(self.m30, rhs),
                    m31: $op::$method(self.m31, rhs),
                    m32: $op::$method(self.m32, rhs),
                    m33: $op::$method(self.m33, rhs),
                }
            }
        }
    };
}

impl_mat_binary_op!(Add::add for Matrix4);
impl_mat_binary_op!(Sub::sub for Matrix4);
impl_mat_binary_op!(Mul::mul for Matrix4);
impl_mat_binary_op!(Div::div for Matrix4);

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    #[inline]
    pub fn normalize(mut self) -> Vector3 {
        let n = self.norm();
        self.x /= n;
        self.y /= n;
        self.z /= n;
        self
    }
}

impl Point3 {
    pub const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).norm()
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Point3 {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Matrix4 {
    pub fn transpose(self) -> Self {
        Matrix4 {
            m00: self.m00,
            m01: self.m10,
            m02: self.m20,
            m03: self.m30,
            m10: self.m01,
            m11: self.m11,
            m12: self.m21,
            m13: self.m31,
            m20: self.m02,
            m21: self.m12,
            m22: self.m22,
            m23: self.m32,
            m30: self.m03,
            m31: self.m13,
            m32: self.m23,
            m33: self.m33,
        }
    }

    #[rustfmt::skip]
    pub fn inverse(&self) -> Option<Self> {
        let a = self;

        let inv = Matrix4 {
            m00:  a.m11*a.m22*a.m33 - a.m11*a.m23*a.m32 - a.m21*a.m12*a.m33 + a.m21*a.m13*a.m32 + a.m31*a.m12*a.m23 - a.m31*a.m13*a.m22,
            m01: -a.m01*a.m22*a.m33 + a.m01*a.m23*a.m32 + a.m21*a.m02*a.m33 - a.m21*a.m03*a.m32 - a.m31*a.m02*a.m23 + a.m31*a.m03*a.m22,
            m02:  a.m01*a.m12*a.m33 - a.m01*a.m13*a.m32 - a.m11*a.m02*a.m33 + a.m11*a.m03*a.m32 + a.m31*a.m02*a.m13 - a.m31*a.m03*a.m12,
            m03: -a.m01*a.m12*a.m23 + a.m01*a.m13*a.m22 + a.m11*a.m02*a.m23 - a.m11*a.m03*a.m22 - a.m21*a.m02*a.m13 + a.m21*a.m03*a.m12,
            m10: -a.m10*a.m22*a.m33 + a.m10*a.m23*a.m32 + a.m20*a.m12*a.m33 - a.m20*a.m13*a.m32 - a.m30*a.m12*a.m23 + a.m30*a.m13*a.m22,
            m11:  a.m00*a.m22*a.m33 - a.m00*a.m23*a.m32 - a.m20*a.m02*a.m33 + a.m20*a.m03*a.m32 + a.m30*a.m02*a.m23 - a.m30*a.m03*a.m22,
            m12: -a.m00*a.m12*a.m33 + a.m00*a.m13*a.m32 + a.m10*a.m02*a.m33 - a.m10*a.m03*a.m32 - a.m30*a.m02*a.m13 + a.m30*a.m03*a.m12,
            m20:  a.m10*a.m21*a.m33 - a.m10*a.m23*a.m31 - a.m20*a.m11*a.m33 + a.m20*a.m13*a.m31 + a.m30*a.m11*a.m23 - a.m30*a.m13*a.m21,
            m13:  a.m00*a.m12*a.m23 - a.m00*a.m13*a.m22 - a.m10*a.m02*a.m23 + a.m10*a.m03*a.m22 + a.m20*a.m02*a.m13 - a.m20*a.m03*a.m12,
            m21: -a.m00*a.m21*a.m33 + a.m00*a.m23*a.m31 + a.m20*a.m01*a.m33 - a.m20*a.m03*a.m31 - a.m30*a.m01*a.m23 + a.m30*a.m03*a.m21,
            m22:  a.m00*a.m11*a.m33 - a.m00*a.m13*a.m31 - a.m10*a.m01*a.m33 + a.m10*a.m03*a.m31 + a.m30*a.m01*a.m13 - a.m30*a.m03*a.m11,
            m23: -a.m00*a.m11*a.m23 + a.m00*a.m13*a.m21 + a.m10*a.m01*a.m23 - a.m10*a.m03*a.m21 - a.m20*a.m01*a.m13 + a.m20*a.m03*a.m11,
            m30: -a.m10*a.m21*a.m32 + a.m10*a.m22*a.m31 + a.m20*a.m11*a.m32 - a.m20*a.m12*a.m31 - a.m30*a.m11*a.m22 + a.m30*a.m12*a.m21,
            m31:  a.m00*a.m21*a.m32 - a.m00*a.m22*a.m31 - a.m20*a.m01*a.m32 + a.m20*a.m02*a.m31 + a.m30*a.m01*a.m22 - a.m30*a.m02*a.m21,
            m32: -a.m00*a.m11*a.m32 + a.m00*a.m12*a.m31 + a.m10*a.m01*a.m32 - a.m10*a.m02*a.m31 - a.m30*a.m01*a.m12 + a.m30*a.m02*a.m11,
            m33:  a.m00*a.m11*a.m22 - a.m00*a.m12*a.m21 - a.m10*a.m01*a.m22 + a.m10*a.m02*a.m21 + a.m20*a.m01*a.m12 - a.m20*a.m02*a.m11,
        };

        let det = a.m00 * inv.m00 + a.m01 * inv.m10 + a.m02 * inv.m20 + a.m03 * inv.m30;

        if !det.is_normal() {
            return None;
        }

        let inv_det = 1.0 / det;

        Some(inv * inv_det)
    }
}