//! WebAssembly-optimized Geometry Routines
//!
//! Note that this code has been designed around WebAssembly. For native CPU code I always
//! use homogenous coordinates with 4-lane SIMD vectors rather than individual scalar values.

use std::ops::{Add, Div, Mul, Sub};

use wasm_bindgen::prelude::*;

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

            #[inline]
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

            #[inline]
            fn $method(self, rhs: f32) -> Self {
                $name {
                    x: $op::$method(self.x, rhs),
                    y: $op::$method(self.y, rhs),
                    z: $op::$method(self.z, rhs),
                }
            }
        }

        #[wasm_bindgen]
        impl $name {
            pub fn $method(&self, rhs: &$name) -> $name {
                $name {
                    x: $op::$method(self.x, rhs.x),
                    y: $op::$method(self.y, rhs.y),
                    z: $op::$method(self.z, rhs.z),
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

#[wasm_bindgen]
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Vector3 {
        *self / self.norm()
    }
}

impl Point3 {
    pub const ORIGIN: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

#[wasm_bindgen]
impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn distance(&self, other: &Point3) -> f32 {
        (*self - *other).norm()
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

#[wasm_bindgen]
impl Point3 {
    pub fn sub(&self, rhs: &Point3) -> Vector3 {
        *self - *rhs
    }

    pub fn add(&self, rhs: &Vector3) -> Point3 {
        *self + *rhs
    }
}

#[wasm_bindgen]
impl Matrix4 {
    #[rustfmt::skip]
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m20: f32,
        m13: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Matrix4 {
        Matrix4 { m00, m01, m02, m03, m10, m11, m12, m20, m13, m21, m22, m23, m30, m31, m32, m33 }
    }

    pub fn transpose(&self) -> Matrix4 {
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

    /// https://stackoverflow.com/a/44446912/2083075 for optimized scalar version
    #[rustfmt::skip]
    pub fn inverse(&self) -> Option<Matrix4> {
        let m = self;

        let a2323 = m.m22 * m.m33 - m.m23 * m.m32;
        let a1323 = m.m21 * m.m33 - m.m23 * m.m31;
        let a1223 = m.m21 * m.m32 - m.m22 * m.m31;
        let a0323 = m.m20 * m.m33 - m.m23 * m.m30;
        let a0223 = m.m20 * m.m32 - m.m22 * m.m30;
        let a0123 = m.m20 * m.m31 - m.m21 * m.m30;
        let a2313 = m.m12 * m.m33 - m.m13 * m.m32;
        let a1313 = m.m11 * m.m33 - m.m13 * m.m31;
        let a1213 = m.m11 * m.m32 - m.m12 * m.m31;
        let a2312 = m.m12 * m.m23 - m.m13 * m.m22;
        let a1312 = m.m11 * m.m23 - m.m13 * m.m21;
        let a1212 = m.m11 * m.m22 - m.m12 * m.m21;
        let a0313 = m.m10 * m.m33 - m.m13 * m.m30;
        let a0213 = m.m10 * m.m32 - m.m12 * m.m30;
        let a0312 = m.m10 * m.m23 - m.m13 * m.m20;
        let a0212 = m.m10 * m.m22 - m.m12 * m.m20;
        let a0113 = m.m10 * m.m31 - m.m11 * m.m30;
        let a0112 = m.m10 * m.m21 - m.m11 * m.m20;

        let z0 = m.m11 * a2323 - m.m12 * a1323 + m.m13 * a1223;
        let z1 = m.m10 * a2323 - m.m12 * a0323 + m.m13 * a0223;
        let z2 = m.m10 * a1323 - m.m11 * a0323 + m.m13 * a0123;
        let z3 = m.m10 * a1223 - m.m11 * a0223 + m.m12 * a0123;

        let det = m.m00 * z0 - m.m01 * z1 + m.m02 * z2 - m.m03 * z3;

        if !det.is_normal() {
            return None;
        }

        let inv_det = 1.0 / det;

        Some(Matrix4 {
            m00: inv_det *   z0,
            m01: inv_det * - ( m.m01 * a2323 - m.m02 * a1323 + m.m03 * a1223 ),
            m02: inv_det *   ( m.m01 * a2313 - m.m02 * a1313 + m.m03 * a1213 ),
            m03: inv_det * - ( m.m01 * a2312 - m.m02 * a1312 + m.m03 * a1212 ),
            m10: inv_det * - z1,
            m11: inv_det *   ( m.m00 * a2323 - m.m02 * a0323 + m.m03 * a0223 ),
            m12: inv_det * - ( m.m00 * a2313 - m.m02 * a0313 + m.m03 * a0213 ),
            m13: inv_det *   ( m.m00 * a2312 - m.m02 * a0312 + m.m03 * a0212 ),
            m20: inv_det *   z2,
            m21: inv_det * - ( m.m00 * a1323 - m.m01 * a0323 + m.m03 * a0123 ),
            m22: inv_det *   ( m.m00 * a1313 - m.m01 * a0313 + m.m03 * a0113 ),
            m23: inv_det * - ( m.m00 * a1312 - m.m01 * a0312 + m.m03 * a0112 ),
            m30: inv_det * - z3,
            m31: inv_det *   ( m.m00 * a1223 - m.m01 * a0223 + m.m02 * a0123 ),
            m32: inv_det * - ( m.m00 * a1213 - m.m01 * a0213 + m.m02 * a0113 ),
            m33: inv_det *   ( m.m00 * a1212 - m.m01 * a0212 + m.m02 * a0112 ),
        })
    }
}
