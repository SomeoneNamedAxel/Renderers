use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl V3 {
    pub fn normalize(&self) -> V3 {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        V3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}