pub(crate) struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    pub(crate) fn new() -> Self {
        let mut id = 0;
        unsafe { gl::GenVertexArrays(1, &mut id) };
        Self { id }
    }

    pub(crate) fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub(crate) fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}
