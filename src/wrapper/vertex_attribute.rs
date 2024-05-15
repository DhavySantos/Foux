use gl::types::{GLboolean, GLenum, GLsizei, GLuint};
use std::os::raw::c_void;

pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    pub fn new(index: u32) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(
                index,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
        }

        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

impl Drop for VertexAttribute {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.index);
        }
    }
}
