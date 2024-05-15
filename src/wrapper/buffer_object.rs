pub(crate) struct BufferObject {
    buffer_type: gl::types::GLenum,
    usage: gl::types::GLenum,
    id: u32,
}

impl BufferObject {
    pub(crate) fn new(buffer_type: gl::types::GLenum, usage: gl::types::GLenum) -> Self {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }

        Self {
            buffer_type,
            usage,
            id,
        }
    }

    pub(crate) fn bind(&self) {
        unsafe { gl::BindBuffer(self.buffer_type, self.id) }
    }

    pub(crate) fn unbind(&self) {
        unsafe { gl::BindBuffer(self.buffer_type, 0) }
    }

    pub(crate) fn data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                self.buffer_type,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                self.usage,
            )
        }
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}
