use gl::types::*;

/// Represents the target types that can be used for OpenGL buffer objects.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum BufferTarget {
    ArrayBuffer = 0x8892,             // GL_ARRAY_BUFFER
    CopyReadBuffer = 0x8F36,          // GL_COPY_READ_BUFFER
    CopyWriteBuffer = 0x8F37,         // GL_COPY_WRITE_BUFFER
    ElementArrayBuffer = 0x8893,      // GL_ELEMENT_ARRAY_BUFFER
    PixelPackBuffer = 0x88EB,         // GL_PIXEL_PACK_BUFFER
    PixelUnpackBuffer = 0x88EC,       // GL_PIXEL_UNPACK_BUFFER
    TextureBuffer = 0x8C2A,           // GL_TEXTURE_BUFFER
    TransformFeedbackBuffer = 0x8C8E, // GL_TRANSFORM_FEEDBACK_BUFFER
    UniformBuffer = 0x8A11,           // GL_UNIFORM_BUFFER
}

/// Represents the usage types for OpenGL buffer objects.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum BufferUsage {
    StreamDraw = 0x88E0,  // GL_STREAM_DRAW
    StreamRead = 0x88E1,  // GL_STREAM_READ
    StreamCopy = 0x88E2,  // GL_STREAM_COPY
    StaticDraw = 0x88E4,  // GL_STATIC_DRAW
    StaticRead = 0x88E5,  // GL_STATIC_READ
    StaticCopy = 0x88E6,  // GL_STATIC_COPY
    DynamicDraw = 0x88E8, // GL_DYNAMIC_DRAW
    DynamicRead = 0x88E9, // GL_DYNAMIC_READ
    DynamicCopy = 0x88EA, // GL_DYNAMIC_COPY
}

/// Represents an OpenGL buffer object.
pub struct BufferObject {
    target: BufferTarget,
    usage: BufferUsage,
    id: u32,
}

impl BufferObject {
    /// Creates a new buffer object with the specified target and usage.
    ///
    /// # Arguments
    ///
    /// * `target` - The target type for the buffer (e.g., `ArrayBuffer`, `ElementArrayBuffer`).
    /// * `usage` - The usage type for the buffer (e.g., `StaticDraw`, `DynamicDraw`).
    ///
    /// # Returns
    ///
    /// A `BufferObject` instance initialized with the given target and usage.
    ///
    /// # Panics
    ///
    /// This function will panic if OpenGL's `GenBuffers` function fails.
    pub fn new(target: BufferTarget, usage: BufferUsage) -> BufferObject {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) };
        BufferObject { id, target, usage }
    }

    /// Binds the buffer object to its target, making it the active buffer.
    ///
    /// This function must be called before any buffer operations like `BufferData` or drawing.
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.target as u32, self.id) }
    }

    /// Unbinds the buffer object by binding it to a target with id `0`.
    ///
    /// This ensures that no buffer is currently bound to the target.
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(self.target as u32, 0) }
    }

    /// Uploads the provided data to the buffer object.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of data to be stored in the buffer. The type `T` must implement
    ///           the `Copy` trait, as the function copies the data into the buffer.
    ///
    /// # Safety
    ///
    /// This function uses raw OpenGL calls that are unsafe and may cause undefined behavior
    /// if the data provided is invalid.
    pub fn data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                self.target as GLenum,                     // Target
                std::mem::size_of_val(data) as GLsizeiptr, // Size of data
                data.as_ptr() as *const GLvoid,            // Data pointer
                self.usage as GLenum,                      // Usage
            );
        }
    }
}

impl Drop for BufferObject {
    /// Cleans up and deletes the buffer object when it goes out of scope.
    ///
    /// This function is automatically called when the `BufferObject` is dropped.
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

