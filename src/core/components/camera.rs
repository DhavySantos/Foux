use cgmath::{Vector2, Vector3};

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    distance: Vector2<f32>,
    fov: f32,
}

impl Camera {
    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn rotation(&self) -> Vector3<f32> {
        self.rotation
    }

    pub fn distance(&self) -> Vector2<f32> {
        self.distance
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
    }

    pub fn set_distance(&mut self, distance: Vector2<f32>) {
        self.distance = distance;
    }
}
