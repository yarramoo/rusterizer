use minifb::{Key, Window, WindowOptions};
use nalgebra::Vector3;

use rusterizer::{utils::*, rasterizer::*};

const WIDTH: usize = 700;
const HEIGHT: usize = 700;

fn main() {
    let mut window = Window::new(
        "Rusterizer",
        WIDTH,
        HEIGHT,
        WindowOptions { title: true, resize: true, ..Default::default() }
    )
    .expect("Unable to create window");

    let mut rasterizer = Rasterizer::new(WIDTH, HEIGHT);

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

    let angle = 0.;
    let eye_pos = Vector3::new(0.,0.,5.);

    rasterizer.set_model(get_model_matrix(&Vector3::new(0., 0., 1.), angle));
    rasterizer.set_view(get_view_matrix(&eye_pos));
    rasterizer.set_projection(get_projection_matrix(45., 1., 0.1, 50.));

    rasterizer.draw(a_pos_ind, a_ind_id, a_col_id);

    let mut buf = Vec::new();

    for v in rasterizer.frame_buf().iter() {
        buf.push(encode_col_u32(v));
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buf[..], WIDTH, HEIGHT).unwrap();
    }
}

fn encode_col_u32(v: &Vector3<f64>) -> u32 {
    (v.x.round() as u32) << 16 | (v.y.round() as u32) << 8 | v.z.round() as u32
}
