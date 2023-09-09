use nalgebra::{self, Matrix4, Matrix3, Vector3, Vector4};
use std::collections::HashMap;

fn get_index(x: usize, y: usize, width: usize, height: usize) -> usize {
    (height-1-y)*width + x
}

fn to_vec4(vec3: &Vector3<f64>, w: f64) -> Vector4<f64> {
    Vector4::new(vec3.x, vec3.y, vec3.z, w)
}

fn to_vec3(vec4: &Vector4<f64>) -> Vector3<f64> {
    Vector3::new(vec4.x, vec4.y, vec4.z)
}

fn compute_barycentric_2D(x: f64, y: f64, a: &Vector3<f64>, b: &Vector3<f64>, c: &Vector3<f64>) -> (f64, f64, f64) {
    let p: Vector3<f64> = Vector3::new(x, y, 0.);
    let v0 = b - a; let v1 = c - a; let v2 = p - a;
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


fn inside_triangle(x: f64, y: f64, triangle: &Triangle) -> bool {
    let v1 = triangle.vertices[0];
    let v2 = triangle.vertices[1];
    let v3 = triangle.vertices[2];
    let (c1, c2, c3) = compute_barycentric_2D(x, y, &v1, &v2, &v3);
    c1 >= 0. && c2 >= 0. && c3 >= 0.
}

struct PosBufID(usize);
struct IndBufID(usize);
struct ColBufID(usize);


#[derive(Default)]
struct Rasterizer {
    model: Matrix4<f64>,
    view:  Matrix4<f64>, 
    projection: Matrix4<f64>,

    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

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

    pub fn load_positions(&mut self, positions: Vec<Vector3<f64>>) -> usize {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions);
        id
    }

    pub fn load_indices(&mut self, indices: Vec<Vector3<usize>>)  -> usize {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices);
        id
    }

    pub fn load_colors(&mut self, colors: Vec<Vector3<usize>>) -> usize {
        let id = self.get_next_id();
        self.ind_buf.insert(id, colors);
        id
    }

    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn draw(&mut self, pos_id: PosBufID, ind_id: IndBufID, col_id: ColBufID) {
        let buf = self.pos_buf.get(&pos_id.0).unwrap();
        let ind = self.ind_buf.get(&ind_id.0).unwrap();
        let col = self.col_buf.get(&col_id.0).unwrap();

        let f1 = (50. - 0.1) / 2.;
        let f2 = (50. + 0.1) / 2.;

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
                vert.z = vert.z * f1 * f2;
            }
            t.vertices.copy_from_slice(&v.map(|vert| to_vec3(&vert)));
            t.colors.copy_from_slice(&[col[i.x], col[i.y], col[i.z]]);
            // Rasterize 
            // rasterize_triangle(&t);
        }
    }
//     void rst::rasterizer::draw(pos_buf_id pos_buffer, ind_buf_id ind_buffer, col_buf_id col_buffer, Primitive type, bool anti_aliased)
// {
//     auto& buf = pos_buf[pos_buffer.pos_id];
//     auto& ind = ind_buf[ind_buffer.ind_id];
//     auto& col = col_buf[col_buffer.col_id];

//     float f1 = (50 - 0.1) / 2.0;
//     float f2 = (50 + 0.1) / 2.0;

//     Eigen::Matrix4f mvp = projection * view * model;
//     for (auto& i : ind)
//     {
//         Triangle t;
//         Eigen::Vector4f v[] = {
//                 mvp * to_vec4(buf[i[0]], 1.0f),
//                 mvp * to_vec4(buf[i[1]], 1.0f),
//                 mvp * to_vec4(buf[i[2]], 1.0f)
//         };
//         //Homogeneous division
//         for (auto& vec : v) {
//             vec /= vec.w();
//         }
//         //Viewport transformation
//         for (auto & vert : v)
//         {
//             vert.x() = 0.5*width*(vert.x()+1.0);
//             vert.y() = 0.5*height*(vert.y()+1.0);
//             vert.z() = vert.z() * f1 + f2;
//         }

//         for (int i = 0; i < 3; ++i)
//         {
//             t.setVertex(i, v[i].head<3>());
//             t.setVertex(i, v[i].head<3>());
//             t.setVertex(i, v[i].head<3>());
//         }

//         auto col_x = col[i[0]];
//         auto col_y = col[i[1]];
//         auto col_z = col[i[2]];

//         t.setColor(0, col_x[0], col_x[1], col_x[2]);
//         t.setColor(1, col_y[0], col_y[1], col_y[2]);
//         t.setColor(2, col_z[0], col_z[1], col_z[2]);

//         if (anti_aliased) {
//             rasterize_triangle_anti_aliased(t);
//         } else {
//             rasterize_triangle(t);
//         }
//     }
// }
}