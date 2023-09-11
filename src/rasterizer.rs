use nalgebra::{self, Matrix4, Matrix3, Vector3, Vector4, Vector};
use std::{collections::HashMap, path::PrefixComponent};

fn get_index(x: usize, y: usize, width: usize, height: usize) -> usize {
    (height-1-y)*width + x
}

fn to_vec4(vec3: &Vector3<f64>, w: f64) -> Vector4<f64> {
    Vector4::new(vec3.x, vec3.y, vec3.z, w)
}

fn to_vec3(vec4: &Vector4<f64>) -> Vector3<f64> {
    Vector3::new(vec4.x, vec4.y, vec4.z)
}

fn compute_barycentric_2D(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let p = Vector3::new(x, y, 0.);
    let v0 = v[1] - v[0]; let v1 = v[2] - v[0]; let v2 = p - v[0];
    let den = v0.x * v1.y - v1.x * v0.y;
    let c1 = (v2.x * v1.y - v1.x * v2.y) / den;
    let c2 = (v0.x * v2.y - v2.x * v0.y) / den;
    let c3 = 1. - c1 - c2;
    (c1,c2,c3)
}

#[derive(Default)]
struct Triangle {
    vertices: [Vector3<f64>; 3],
    colors:   [Vector3<f64>; 3],
    tex_coords: [Vector3<f64>; 3],
    normals:  [Vector3<f64>; 3], 
}

impl Triangle {
    fn to_vector4(&self) -> [Vector4<f64>; 3] {
        let mut out = [Vector4::default(); 3];
        for (i, v) in self.vertices.iter().enumerate() {
            out[i] = Vector4::new(v.x, v.y, v.z, 1.);
        }
        out
    }
}


fn inside_triangle(x: f64, y: f64, triangle: &Triangle) -> bool {
    let (c1, c2, c3) = compute_barycentric_2D(x, y, &triangle.vertices);
    c1 >= 0. && c2 >= 0. && c3 >= 0.
}

pub struct PosBufID(usize);
pub struct IndBufID(usize);
pub struct ColBufID(usize);


#[derive(Default)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view:  Matrix4<f64>, 
    projection: Matrix4<f64>,

    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    // Buffers for rendering
    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,

    width: usize,
    height: usize,

    next_id: usize,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Rasterizer {
            frame_buf: vec![Vector3::new(0., 0., 0.,); width * height],
            depth_buf: vec![f64::MAX; width * height],
            width,
            height,
            next_id: 0,
            ..Default::default()
        }
    }

    pub fn frame_buf(&self) -> &[Vector3<f64>] {
        &self.frame_buf[..]
    }

    pub fn load_positions(&mut self, positions: Vec<Vector3<f64>>) -> PosBufID {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions);
        PosBufID(id)
    }

    pub fn load_indices(&mut self, indices: Vec<Vector3<usize>>)  -> IndBufID {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices);
        IndBufID(id)
    }

    pub fn load_colors(&mut self, colors: Vec<Vector3<f64>>) -> ColBufID {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors);
        ColBufID(id)
    }

    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn set_pixel(
        frame_buf: &mut Vec<Vector3<f64>>, 
        point: &Vector3<f64>, 
        color: &Vector3<f64>,
        width: usize,
        height: usize,
    ) {
        let idx = get_index(point.x.round() as usize, point.y.round() as usize, width, height);
        frame_buf[idx] = *color;
    }

    fn get_pixel(&self, point: &Vector3<f64>) -> &Vector3<f64> {
        let idx = get_index(point.x.round() as usize, point.y.round() as usize, self.width, self.height);
        &self.frame_buf[idx]
    }

    pub fn clear_frame_buf(&mut self) {
        self.frame_buf.fill(Vector3::zeros());
    }

    pub fn clear_depth_buf(&mut self) {
        self.depth_buf.fill(f64::MAX);
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    pub fn draw(&mut self, pos_id: PosBufID, ind_id: IndBufID, col_id: ColBufID) {
        let buf = self.pos_buf.get(&pos_id.0).unwrap();
        let ind = self.ind_buf.get(&ind_id.0).unwrap();
        let col = self.col_buf.get(&col_id.0).unwrap();

        let f1 = (100. - 0.1) / 2.;
        let f2 = (100. + 0.1) / 2.;

        let mvp = self.projection * self.view * self.model;
        for i in ind.iter() {
            let mut t = Triangle::default();
            // Vertices
            let mut v = [
                mvp * to_vec4(&buf[i.x], 1.),
                mvp * to_vec4(&buf[i.y], 1.),
                mvp * to_vec4(&buf[i.z], 1.),
            ];
            for vec in v.iter_mut() {
                *vec /= vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.);
                vert.z = vert.z * f1 + f2;
            }
            t.vertices.copy_from_slice(&v.map(|vert| to_vec3(&vert)));
            t.colors.copy_from_slice(&[col[i.x], col[i.y], col[i.z]]);
            // Rasterize 
            Rasterizer::rasterize_triangle(&mut self.frame_buf, &mut self.depth_buf, self.width, self.height, &t);
        }
    }

    fn rasterize_triangle(
        frame_buf: &mut Vec<Vector3<f64>>, 
        depth_buf: &mut Vec<f64>,
        width: usize,
        height: usize,
        triangle: &Triangle
    ) {
        let v = triangle.to_vector4();

        let (mut bottom, mut top, mut left, mut right) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for vec in v.iter() {
            bottom = bottom.min(vec.y);
            top    = top.max(vec.y);
            left   = left.min(vec.x);
            right  = right.max(vec.x);
        }

        let (i_bottom, i_top, i_left, i_right) =
            (bottom.floor() as usize, top.floor() as usize, left.floor() as usize, right.floor() as usize);
        for y in i_bottom..=i_top {
            for x in i_left..=i_right {
                if inside_triangle(x as f64, y as f64, triangle) {
                    let (alpha, beta, gamma) = compute_barycentric_2D(x as f64, y as f64, &triangle.vertices);
                    let w_reciproal = 1./(alpha / v[0].w + beta / v[1].w + gamma / v[2].w);
                    let z_interpolated = (alpha * v[0].z / v[0].w + beta * v[1].z / v[1].w + gamma * v[2].z / v[2].w) * w_reciproal;
                    let idx = get_index(x, y, width, height);
                    if z_interpolated < depth_buf[idx] {
                        depth_buf[idx] = z_interpolated;
                        let point = Vector3::new(x as f64, y as f64, 0.);
                        Rasterizer::set_pixel(frame_buf, &point, &(triangle.colors[0] * 255.), width, height);
                    }
                }
            }
        }
    }
}