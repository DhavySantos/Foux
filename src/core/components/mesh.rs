pub struct Mesh {
    indices: Vec<u32>,
    vertices: Vec<f32>,
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Mesh {
        Mesh { vertices, indices }
    }

    pub fn vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}
