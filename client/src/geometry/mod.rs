//! WebAssembly-optimized Geometry Routines
//!
//! Note that this code has been designed around WebAssembly. For native CPU code I always
//! use homogenous coordinates with 4-lane SIMD vectors rather than individual scalar values.

use std::ops::{Add, Div, Mul, Sub};

pub mod two_d;

pub use self::two_d::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    };
}

impl_vec_binary_op!(Add::add for Vector3);
impl_vec_binary_op!(Sub::sub for Vector3);
impl_vec_binary_op!(Mul::mul for Vector3);
impl_vec_binary_op!(Div::div for Vector3);

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Vector3 {
        self / self.norm()
    }
}

impl Point3 {
    pub const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn distance(self, other: Point3) -> f32 {
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
    #[rustfmt::skip]
    pub const IDENTITY: Matrix4 = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    #[rustfmt::skip]
    pub const ZERO: Matrix4 = Matrix4::new(
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
    );

    #[rustfmt::skip]
    pub const fn new(
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

    pub fn translate(delta: Vector3) -> Matrix4 {
        let mut mat = Matrix4::IDENTITY;

        mat.m03 = delta.x;
        mat.m13 = delta.y;
        mat.m23 = delta.z;

        mat
    }

    pub fn scale(axes: Vector3) -> Matrix4 {
        let mut mat = Matrix4::IDENTITY;

        mat.m00 = axes.x;
        mat.m11 = axes.y;
        mat.m22 = axes.z;

        mat
    }

    pub fn rotate(axis_angle: Vector3) -> Matrix4 {
        let angle = axis_angle.norm();

        let mut mat = Matrix4::IDENTITY;

        if angle.abs() < 1e-5 {
            return mat;
        }

        let Vector3 { x, y, z } = axis_angle / angle;

        let (sin_theta, cos_theta) = angle.sin_cos();

        mat.m00 = x * x + (1.0 - x * x) * cos_theta;
        mat.m01 = x * y * (1.0 - cos_theta) - z * sin_theta;
        mat.m02 = x * z * (1.0 - cos_theta) + y * sin_theta;
        mat.m10 = x * y * (1.0 - cos_theta) + z * sin_theta;
        mat.m11 = y * y + (1.0 - y * y) * cos_theta;
        mat.m12 = y * z * (1.0 - cos_theta) - x * sin_theta;
        mat.m20 = x * z * (1.0 - cos_theta) - y * sin_theta;
        mat.m21 = y * z * (1.0 - cos_theta) + x * sin_theta;
        mat.m22 = z * z + (1.0 - z * z) * cos_theta;

        mat
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
            m00:   z0,
            m01: - ( m.m01 * a2323 - m.m02 * a1323 + m.m03 * a1223 ),
            m02:   ( m.m01 * a2313 - m.m02 * a1313 + m.m03 * a1213 ),
            m03: - ( m.m01 * a2312 - m.m02 * a1312 + m.m03 * a1212 ),
            m10: - z1,
            m11:   ( m.m00 * a2323 - m.m02 * a0323 + m.m03 * a0223 ),
            m12: - ( m.m00 * a2313 - m.m02 * a0313 + m.m03 * a0213 ),
            m13:   ( m.m00 * a2312 - m.m02 * a0312 + m.m03 * a0212 ),
            m20:   z2,
            m21: - ( m.m00 * a1323 - m.m01 * a0323 + m.m03 * a0123 ),
            m22:   ( m.m00 * a1313 - m.m01 * a0313 + m.m03 * a0113 ),
            m23: - ( m.m00 * a1312 - m.m01 * a0312 + m.m03 * a0112 ),
            m30: - z3,
            m31:   ( m.m00 * a1223 - m.m01 * a0223 + m.m02 * a0123 ),
            m32: - ( m.m00 * a1213 - m.m01 * a0213 + m.m02 * a0113 ),
            m33:   ( m.m00 * a1212 - m.m01 * a0212 + m.m02 * a0112 ),
        } * inv_det)
    }

    pub fn format(&self, ident: usize) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;

        let mut buf = String::with_capacity((ident * 4 + 12) * 4);

        let ident = std::iter::repeat(' ').take(4);

        buf.push_str("Matrix4 {\n");

        buf.extend(ident.clone());
        write!(buf, "{:.3}, {:.3}, {:.3}, {:.3},\n", self.m00, self.m01, self.m02, self.m03)?;

        buf.extend(ident.clone());
        write!(buf, "{:.3}, {:.3}, {:.3}, {:.3},\n", self.m10, self.m11, self.m12, self.m13)?;

        buf.extend(ident.clone());
        write!(buf, "{:.3}, {:.3}, {:.3}, {:.3},\n", self.m20, self.m21, self.m22, self.m23)?;

        buf.extend(ident);
        write!(buf, "{:.3}, {:.3}, {:.3}, {:.3},\n", self.m30, self.m31, self.m32, self.m33)?;

        buf.push('}');

        Ok(buf)
    }
}

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

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(mut self, rhs: f32) -> Matrix4 {
        self.m00 *= rhs;
        self.m01 *= rhs;
        self.m02 *= rhs;
        self.m03 *= rhs;
        self.m10 *= rhs;
        self.m11 *= rhs;
        self.m12 *= rhs;
        self.m13 *= rhs;
        self.m20 *= rhs;
        self.m21 *= rhs;
        self.m22 *= rhs;
        self.m23 *= rhs;
        self.m30 *= rhs;
        self.m31 *= rhs;
        self.m32 *= rhs;
        self.m33 *= rhs;
        self
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let a = self;
        let b = rhs;
        let mut m = Matrix4::ZERO;

        m.m00 = (b.m00 * a.m00) + (b.m01 * a.m10) + (b.m02 * a.m20) + (b.m03 * a.m30);
        m.m01 = (b.m00 * a.m01) + (b.m01 * a.m11) + (b.m02 * a.m21) + (b.m03 * a.m31);
        m.m02 = (b.m00 * a.m02) + (b.m01 * a.m12) + (b.m02 * a.m22) + (b.m03 * a.m32);
        m.m03 = (b.m00 * a.m03) + (b.m01 * a.m13) + (b.m02 * a.m23) + (b.m03 * a.m33);
        m.m10 = (b.m10 * a.m00) + (b.m11 * a.m10) + (b.m12 * a.m20) + (b.m13 * a.m30);
        m.m11 = (b.m10 * a.m01) + (b.m11 * a.m11) + (b.m12 * a.m21) + (b.m13 * a.m31);
        m.m12 = (b.m10 * a.m02) + (b.m11 * a.m12) + (b.m12 * a.m22) + (b.m13 * a.m32);
        m.m13 = (b.m10 * a.m03) + (b.m11 * a.m13) + (b.m12 * a.m23) + (b.m13 * a.m33);
        m.m20 = (b.m20 * a.m00) + (b.m21 * a.m10) + (b.m22 * a.m20) + (b.m23 * a.m30);
        m.m21 = (b.m20 * a.m01) + (b.m21 * a.m11) + (b.m22 * a.m21) + (b.m23 * a.m31);
        m.m22 = (b.m20 * a.m02) + (b.m21 * a.m12) + (b.m22 * a.m22) + (b.m23 * a.m32);
        m.m23 = (b.m20 * a.m03) + (b.m21 * a.m13) + (b.m22 * a.m23) + (b.m23 * a.m33);
        m.m30 = (b.m30 * a.m00) + (b.m31 * a.m10) + (b.m32 * a.m20) + (b.m33 * a.m30);
        m.m31 = (b.m30 * a.m01) + (b.m31 * a.m11) + (b.m32 * a.m21) + (b.m33 * a.m31);
        m.m32 = (b.m30 * a.m02) + (b.m31 * a.m12) + (b.m32 * a.m22) + (b.m33 * a.m32);
        m.m33 = (b.m30 * a.m03) + (b.m31 * a.m13) + (b.m32 * a.m23) + (b.m33 * a.m33);

        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_inverse() {
        let m = Matrix4::rotate(Vector3::new(1.0, -2.0, 0.5)) * Matrix4::scale(Vector3::new(1.0, 2.0, 3.0));

        let inverse = m.inverse().unwrap();

        let identity = m * inverse;

        println!("{}", identity.format(4).unwrap());
    }
}
