use nalgebra::{Vector3, Vector4};
use crate::color::Color;

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
    pub fn new_matte(
        v_a: (f64, f64, f64),
        v_b: (f64, f64, f64),
        v_c: (f64, f64, f64),
        color: Color,
    ) -> Self {
        TriangleBuilder::new()
            .with_vertices(&[
                Vector3::new(v_a.0, v_a.1, v_a.2),
                Vector3::new(v_b.0, v_b.1, v_b.2),
                Vector3::new(v_c.0, v_c.1, v_c.2),
            ])
            .with_color(color)
            .build()
    }
}

// Idea: this general builder structure could be turned into a derive macro
// Add a lifetime to an Optional reference to each field type. 
// Add a build that has non-negotiables and defaults. Some more complex conditions may be tricky...
pub struct TriangleBuilder<'a> {
    vertices:   Option<&'a [Vector3<f64>; 3]>,
    colors:     Option<&'a [Vector3<f64>; 3]>,
    color:      Option<Color>,
    tex_coords: Option<&'a [Vector3<f64>; 3]>,
    normals:    Option<&'a [Vector3<f64>; 3]>,
}

impl<'a> TriangleBuilder<'a> {
    pub fn new() -> Self {
        TriangleBuilder { vertices: None, colors: None, color: None, tex_coords: None, normals: None }
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
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
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
        if self.colors.is_none() && self.color.is_none() && self.tex_coords.is_none() {
            panic!("Trying to build a triangle without color or texture");
        }
        let create_default_vectors = || { [Vector3::default(); 3] };
        let auto_color = self.color.map(|c| [
            c.rgb(), c.rgb(), c.rgb()
        ]);
        Triangle { 
            vertices:   self.vertices  .copied().expect("Trying to build a Triangle without any vertices!"),
            colors: if auto_color.is_some() {
                auto_color.unwrap()
            } else {
                self.colors.copied().unwrap_or_else(create_default_vectors)
            },
            // colors:     self.colors    .copied().unwrap_or_else(create_default_vectors),
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