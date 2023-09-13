use nalgebra::{self, Matrix4, Vector3, Vector4};
use std::{collections::HashMap, ops::{Deref, Range}};

use super::triangle::{Triangle, TriangleBuilder};


fn get_index(x: usize, y: usize, width: usize, height: usize) -> usize {
    (height-1-y)*width + x
}

fn to_vec4(vec3: &Vector3<f64>, w: f64) -> Vector4<f64> {
    Vector4::new(vec3.x, vec3.y, vec3.z, w)
}

fn to_vec3(vec4: &Vector4<f64>) -> Vector3<f64> {
    Vector3::new(vec4.x, vec4.y, vec4.z)
}

pub fn compute_barycentric_2_d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let p = Vector3::new(x, y, 0.);
    let v0 = v[1] - v[0]; let v1 = v[2] - v[0]; let v2 = p - v[0];
    let den = v0.x * v1.y - v1.x * v0.y;
    let c1 = (v2.x * v1.y - v1.x * v2.y) / den;
    let c2 = (v0.x * v2.y - v2.x * v0.y) / den;
    let c3 = 1. - c1 - c2;
    (c1,c2,c3)
}


// #[proc_macro_derive(TlbormDerive)]
// pub fn tlborm_derive(item: TokenStream) -> TokenStream {
//     TokenStream::new()
// }

// #[proc_macro_derive(NewTypeDeref)]
// fn newtype_deref_derive(item: TokenStream) -> TokenStream {
    
// }

pub struct PosBufID(usize);
impl Deref for PosBufID {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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

    pub fn load_positions(&mut self, positions: &[Vector3<f64>]) -> PosBufID {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.to_vec());
        PosBufID(id)
    }

    pub fn load_indices(&mut self, indices: &[Vector3<usize>])  -> IndBufID {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.to_vec());
        IndBufID(id)
    }

    pub fn load_indices_from_range(&mut self, index_range: Range<usize>) -> IndBufID {
        let id = self.get_next_id();
        let mut indices = Vec::with_capacity((index_range.end - index_range.start) / 3);
        for i in index_range.step_by(3) {
            indices.push(Vector3::new(i, i+1, i+2));
        }
        self.ind_buf.insert(id, indices);
        IndBufID(id)
    }

    pub fn load_colors(&mut self, colors: &[Vector3<f64>]) -> ColBufID {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.to_vec());
        ColBufID(id)
    }

    pub fn load_triangle(&mut self, triangle: &Triangle) -> (PosBufID, IndBufID, ColBufID) {
        let pos_buf_id = self.load_positions(&triangle.vertices);
        let ind_buf_id = self.load_indices_from_range(0..triangle.vertices.len());
        let col_buf_id = self.load_colors(&triangle.colors);
        (pos_buf_id, ind_buf_id, col_buf_id)
    }

    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn set_pixel(
        frame_buf: &mut [Vector3<f64>], 
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
        let buf = self.pos_buf.get(&*pos_id).unwrap();
        let ind = self.ind_buf.get(&ind_id.0).unwrap();
        let col = self.col_buf.get(&col_id.0).unwrap();

        let f1 = (100. - 0.1) / 2.;
        let f2 = (100. + 0.1) / 2.;

        let mvp = self.projection * self.view * self.model;
        for i in ind.iter() {
            // Vertices
            let mut vertices = [
                mvp * to_vec4(&buf[i.x], 1.),
                mvp * to_vec4(&buf[i.y], 1.),
                mvp * to_vec4(&buf[i.z], 1.),
            ];
            for vec in vertices.iter_mut() {
                *vec /= vec.w;
            }
            for vert in vertices.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.);
                vert.z = vert.z * f1 + f2;
            }

            let t = TriangleBuilder::new()
                .with_vertices(&[
                    Vector3::new(vertices[0].x, vertices[0].y, vertices[0].z),
                    Vector3::new(vertices[1].x, vertices[1].y, vertices[1].z),
                    Vector3::new(vertices[2].x, vertices[2].y, vertices[2].z),
                ])
                .with_colors(&[col[i.x], col[i.y], col[i.z]])
                .build();
            // Rasterize 
            Rasterizer::rasterize_triangle(
                &mut self.frame_buf[..], 
                &mut self.depth_buf[..], 
                self.width, 
                self.height, 
                &t
            );
        }
    }

    fn rasterize_triangle(
        frame_buf: &mut [Vector3<f64>], 
        depth_buf: &mut [f64],
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
                if triangle.inside_triangle(x as f64, y as f64) {
                    let (alpha, beta, gamma) = compute_barycentric_2_d(x as f64, y as f64, &triangle.vertices);
                    let w_reciproal = 1./(alpha / v[0].w + beta / v[1].w + gamma / v[2].w);
                    let z_interpolated = (alpha * v[0].z / v[0].w + beta * v[1].z / v[1].w + gamma * v[2].z / v[2].w) * w_reciproal;
                    let idx = get_index(x, y, width, height);
                    if z_interpolated < depth_buf[idx] {
                        depth_buf[idx] = z_interpolated;
                        let point = Vector3::new(x as f64, y as f64, 0.);
                        // No color interpolation. TODO
                        Rasterizer::set_pixel(frame_buf, &point, &(triangle.colors[0]), width, height);
                    }
                }
            }
        }
    }
}