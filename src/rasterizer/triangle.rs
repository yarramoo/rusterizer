use nalgebra::{Vector3, Vector4};

#[derive(Default)]
pub struct Triangle {
    pub vertices: [Vector3<f64>; 3],
    pub colors:   [Vector3<f64>; 3],
    pub tex_coords: [Vector3<f64>; 3],
    pub normals:  [Vector3<f64>; 3], 
}

impl Triangle {
    pub fn to_vector4(&self) -> [Vector4<f64>; 3] {
        let mut out = [Vector4::default(); 3];
        for (i, v) in self.vertices.iter().enumerate() {
            out[i] = Vector4::new(v.x, v.y, v.z, 1.);
        }
        out
    }
    pub fn inside_triangle(&self, x: f64, y: f64) -> bool {
        use crate::rasterizer::compute_barycentric_2D;
        let (c1, c2, c3) = compute_barycentric_2D(x, y, &self.vertices);
        c1 >= 0. && c2 >= 0. && c3 >= 0.
    }
}

pub struct TriangleBuilder {
    vertices: Option<[Vector3<f64>; 3]>,
    colors:   Option<[Vector3<f64>; 3]>,
    tex_coords: Option<[Vector3<f64>; 3]>,
    normals:  Option<[Vector3<f64>; 3]>, 
}

impl TriangleBuilder {
    pub fn new() -> Self {
        TriangleBuilder { vertices: None, colors: None, tex_coords: None, normals: None }
    }
    pub fn with_vertices(mut self, vertices: &[Vector3<f64>; 3]) -> Self 
    {
        self.vertices = Some(vertices.clone());
        self
    }
    pub fn with_colors(mut self, colors: &[Vector3<f64>; 3]) -> Self {
        self.colors = Some(colors.clone());
        self
    }
    pub fn with_tex_coords(mut self, tex_coords: &[Vector3<f64>; 3]) -> Self {
        self.tex_coords = Some(tex_coords.clone());
        self
    }
    pub fn with_normals(mut self, normals: &[Vector3<f64>; 3]) -> Self {
        self.normals = Some(normals.clone());
        self
    }
    pub fn build(self) -> Triangle {
        Triangle { 
            vertices:   self.vertices.unwrap_or_default(), 
            colors:     self.colors.unwrap_or_default(), 
            tex_coords: self.tex_coords.unwrap_or_default(), 
            normals:    self.normals.unwrap_or_default() 
        }
    }
}

