use minifb::{Key, Window, WindowOptions};

mod rasterizer;
use nalgebra::{Vector3, Matrix4, iter::MatrixIter};
use rasterizer::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn get_view_matrix(eye_pos: &Vector3<f64>) -> Matrix4<f64> {
    let mut view: Matrix4<f64> = Matrix4::identity();
    let translate = Matrix4::new(
        1., 0., 0., -eye_pos.x,
        0., 1., 0., -eye_pos.y,
        0., 0., 1., -eye_pos.z,
        0., 0., 0., 1.
    );
    view*translate
}

fn get_model_matrix(axis: &Vector3<f64>, angle: f64) -> Matrix4<f64> {
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

fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, zNear: f64, zFar: f64) -> Matrix4<f64> {
    // Matrix for projecting 
    let top = zNear * (eye_fov / 2.).tan();
    let bottom = -top;
    let right = top * aspect_ratio;
    let left = -right;
    let projection = Matrix4::new(
        zNear/right, 0., 0., 0.,
        0., zNear/top, 0., 0.,
        0., 0., -((zFar+zNear)/(zFar-zNear)), -2.*zFar*zNear,
        0., 0., -1., 0.
    );
    projection
}

fn main() {
    let mut window = Window::new(
        "Line drawing test",
        WIDTH,
        HEIGHT,
        WindowOptions { title: true, resize: true, ..Default::default() }
    )
    .expect("Unable to create window");

    let mut rasterizer = Rasterizer::new(WIDTH, HEIGHT);

    let eye_pos: Vector3<f64> = Vector3::new(0., 0., 5.);

    let triangle_a: Vec<Vector3<f64>> = vec![
        Vector3::new(2., 0., -2.),
        Vector3::new(0., 2., -2.),
        Vector3::new(-2., 0., -2.)
    ];
    let triangle_a_ind: Vec<Vector3<usize>> = vec![
        Vector3::new(0, 1, 2)
    ];
    let triangle_a_col: Vec<Vector3<f64>> = vec![
        Vector3::new(217.0, 238.0, 185.0),
        Vector3::new(217.0, 238.0, 185.0),
        Vector3::new(217.0, 238.0, 185.0),
    ];

    let a_pos_ind = rasterizer.load_positions(triangle_a);
    let a_ind_id = rasterizer.load_indices(triangle_a_ind);
    let a_col_id = rasterizer.load_colors(triangle_a_col);

    rasterizer.clear_depth_buf();
    rasterizer.clear_frame_buf();

}

fn fill_pattern(buffer: &mut [u32], width: usize, height: usize, gap: usize) {
    assert_eq!(buffer.len(), width * height);

    buffer
        .iter_mut()
        .enumerate()
        .for_each(|(i, p)| {
            if i % gap == 0 {
                *p = 0xffffffff;
            } else {
                *p = 0x0;
            }
        });
}
