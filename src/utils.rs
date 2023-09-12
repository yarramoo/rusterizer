use nalgebra::{Vector3, Matrix4};

pub fn get_view_matrix(eye_pos: &Vector3<f64>) -> Matrix4<f64> {
    let view: Matrix4<f64> = Matrix4::identity();
    let translate = Matrix4::new(
        1., 0., 0., -eye_pos.x,
        0., 1., 0., -eye_pos.y,
        0., 0., 1., -eye_pos.z,
        0., 0., 0., 1.
    );
    view*translate
}

pub fn get_model_matrix(axis: &Vector3<f64>, angle: f64) -> Matrix4<f64> {
    // Using Rodrigues Rotation Formula: 
    // https://mathworld.wolfram.com/RodriguesRotationFormula.html
    let axis = axis.normalize();

    let (x, y, z) = (axis.x, axis.y, axis.z);
    let (sin, cos) = angle.sin_cos();

    let a = cos + x * x * (1. - cos);
    let b = x * y * (1. - cos) - z * sin;
    let c = y * sin + x * z * (1. - cos);
    let d = z * sin + x * y * (1. - cos);
    let e = cos + y * y * (1. - cos);
    let f = -x * sin + y * z * (1. - cos);
    let g = -y * sin + x * z * (1. - cos);
    let h = x * sin + y * z * (1. - cos);
    let i = cos + z * z * (1. - cos);

    Matrix4::new(
        a, b, c, 0.,
        d, e, f, 0.,
        g, h, i, 0.,
        0., 0., 0., 1.
    )
}

pub fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
    // Matrix for projecting 
    let eye_fov = eye_fov.to_radians();
    let top = z_near * (eye_fov / 2.).tan();
    let _bottom = -top;
    let right = top * aspect_ratio;
    let _left = -right;
    Matrix4::new(
        z_near/right, 0., 0., 0.,
        0., z_near/top, 0., 0.,
        0., 0., -((z_far+z_near)/(z_far-z_near)), -2.*z_far*z_near,
        0., 0., -1., 0.
    )
}