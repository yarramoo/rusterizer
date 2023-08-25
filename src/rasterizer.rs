use nalgebra::{self, Matrix4, Matrix3, Vector3, Vector4};
use std::collections::HashMap;

fn get_index(x: usize, y: usize, width: usize, height: usize) -> usize {
    (height-1-y)*width + x
}

fn to_vec4(vec3: &Vector3<f64>, w: f64) -> Vector4 {
    Vector4::new(vec3.x, vec3.y, vec3.z, w)
}

fn compute_barycentric_2D(x: f64, y: f64, a: &Vector3<f64>, b: &Vector3<f64>, c: &Vector3<f64>) -> (f64, f64, f64) {
    let p: Vector3<f64> = Vector3::new(x, y, 0);
    let v0 = b - a; let v1 = c - a; let v2 = p - a;
    let den = v0.x() * v1.y() - v1.x() * v0.y();
    let c1 = (v2.x() * v1.y() - v1.x() * v2.y()) / den;
    let c2 = (v0.x() * v2.y() - v2.x() * v0.y()) / den;
    let c3 = 1.0f - c1 - c2;
    (c1,c2,c3)
}

#[derive(Default)]
struct Rasterizer {
    model: Matrix4<f64>,
    view:  Matrix4<f64>, 
    projection: Matrix4<f64>,

    pos_buf: HashMap<usize, Vec<Matrix3<f64>>>,
    ind_buf: HashMap<usize, Vec<Matrix3<usize>>>,
    col_buf: HashMap<usize, Vec<Matrix3<f64>>>,

    // Buffers for rendering
    frame_buf: Vec<Matrix3<f64>>,
    depth_buf: Vec<Matrix3<f64>>,

    width: usize,
    height: usize,

    next_id: usize,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Rasterizer {
            frame_buf: Vec::with_capacity(width * height),
            depth_buf: Vec::with_capacity(width * height),
            width,
            height,
            next_id: 0,
            ..Default::default()
        }
    }

    pub fn frame_buf(&self) -> &[Matrix3<f64>] {
        &self.frame_buf[..]
    }

    pub fn load_positions(&mut self, positions: Vec<Matrix3<f64>>) -> usize {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions);
        id
    }

    pub fn load_indices(&mut self, indices: Vec<Matrix3<usize>>)  -> usize {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices);
        id
    }

    pub fn load_colors(&mut self, colors: Vec<Matrix3<usize>>) -> usize {
        let id = self.get_next_id();
        self.ind_buf.insert(id, colors);
        id
    }

    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}