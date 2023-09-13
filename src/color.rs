use nalgebra::Vector3;

#[derive(Clone, Copy)]
pub enum Color {
    Red, Green, Blue
}

impl Color {
    pub fn rgb(&self) -> Vector3<f64> {
        match self {
            Self::Red   => Vector3::new(255., 0., 0.),
            Self::Green => Vector3::new(0., 255., 0.),
            Self::Blue  => Vector3::new(0., 0., 255.),
        }
    }
}