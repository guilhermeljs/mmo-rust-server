#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Position {
    pub fn zero()-> Self{
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn new(x: f32, y: f32, z: f32)-> Self {
        Position {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn distance(&self, other: &Position) -> f32 {
        ((self.x - other.x).abs().powi(2) + (self.y - other.y).abs().powi(2) + (self.z - other.z).abs().powi(2)).sqrt()
    }
}