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
        use crate::rasterizer::compute_barycentric_2_d;
        let (c1, c2, c3) = compute_barycentric_2_d(x, y, &self.vertices);
        c1 >= 0. && c2 >= 0. && c3 >= 0.
    }
}

pub struct TriangleBuilder<'a> {
    vertices:   Option<&'a [Vector3<f64>; 3]>,
    colors:     Option<&'a [Vector3<f64>; 3]>,
    tex_coords: Option<&'a [Vector3<f64>; 3]>,
    normals:    Option<&'a [Vector3<f64>; 3]>, 
}

impl<'a> TriangleBuilder<'a> {
    pub fn new() -> Self {
        TriangleBuilder { vertices: None, colors: None, tex_coords: None, normals: None }
    }
    pub fn with_vertices(mut self, vertices: &'a [Vector3<f64>; 3]) -> Self 
    {
        self.vertices = Some(vertices);
        self
    }
    pub fn with_colors(mut self, colors: &'a [Vector3<f64>; 3]) -> Self {
        self.colors = Some(colors);
        self
    }
    pub fn with_tex_coords(mut self, tex_coords: &'a [Vector3<f64>; 3]) -> Self {
        self.tex_coords = Some(tex_coords);
        self
    }
    pub fn with_normals(mut self, normals: &'a [Vector3<f64>; 3]) -> Self {
        self.normals = Some(normals);
        self
    }
    pub fn build(self) -> Triangle {
        if self.colors.is_none() && self.tex_coords.is_none() {
            panic!("Trying to build a triangle without color or texture");
        }
        let create_default_vectors = || { [Vector3::default(); 3] };
        Triangle { 
            vertices:   self.vertices  .copied().expect("Trying to build a Triangle without any vertices!"),
            colors:     self.colors    .copied().unwrap_or_else(create_default_vectors),
            tex_coords: self.tex_coords.copied().unwrap_or_else(create_default_vectors),
            normals:    self.normals   .copied().unwrap_or_else(create_default_vectors),
        }
    }
}

impl<'a> Default for TriangleBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}