use super::*;

#[wasm_bindgen]
pub fn new_vector3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3::new(x, y, z)
}

#[wasm_bindgen]
pub fn new_point3(x: f32, y: f32, z: f32) -> Point3 {
    Point3::new(x, y, z)
}

#[rustfmt::skip]
#[wasm_bindgen]
pub fn new_matrix4(
    m00: f32, m01: f32, m02: f32, m03: f32,
    m10: f32, m11: f32, m12: f32, m20: f32,
    m13: f32, m21: f32, m22: f32, m23: f32,
    m30: f32, m31: f32, m32: f32, m33: f32,
) -> Matrix4 {
    Matrix4 { m00, m01, m02, m03, m10, m11, m12, m20, m13, m21, m22, m23, m30, m31, m32, m33 }
}

#[wasm_bindgen]
pub fn invert_matrix4(mat: &Matrix4) -> Option<Matrix4> {
    mat.inverse()
}
