pub struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        VertexArrayObject::default()
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

impl Default for VertexArrayObject {
    fn default() -> Self {
        let mut id = 0;
        unsafe { gl::GenVertexArrays(1, &mut id) };
        VertexArrayObject { id }
    }
}
