#[derive(Default)]
pub struct Camera {
    position: [f32; 3],
    rotation: [f32; 3],
    distance: [f32; 2],
    fov: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }

    pub fn position(&self) -> [f32; 3] {
        self.position
    }

    pub fn rotation(&self) -> [f32; 3] {
        self.rotation
    }

    pub fn distance(&self) -> [f32; 2] {
        self.distance
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.rotation = rotation;
    }

    pub fn set_distance(&mut self, distance: [f32; 2]) {
        self.distance = distance;
    }
}
