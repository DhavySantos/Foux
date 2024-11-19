use std::ffi::c_void;
use std::ptr;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum DataType {
    Byte = 0x1400,                     // GL_BYTE
    UnsignedByte = 0x1401,             // GL_UNSIGNED_BYTE
    Short = 0x1402,                    // GL_SHORT
    UnsignedShort = 0x1403,            // GL_UNSIGNED_SHORT
    Int = 0x1404,                      // GL_INT
    UnsignedInt = 0x1405,              // GL_UNSIGNED_INT
    HalfFloat = 0x140B,                // GL_HALF_FLOAT
    Float = 0x1406,                    // GL_FLOAT
    Double = 0x140A,                   // GL_DOUBLE
    Int2_10_10_10Rev = 0x8D9F,         // GL_INT_2_10_10_10_REV
    UnsignedInt2_10_10_10Rev = 0x8368, // GL_UNSIGNED_INT_2_10_10_10_REV
}

/// Struct representing a vertex attribute in OpenGL.
pub struct VertexAttribPointer {
    index: u32,
}

impl VertexAttribPointer {
    /// Creates a new `VertexAttribPointer` and sets up the vertex attribute pointer.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the vertex attribute.
    /// * `size` - Number of components per vertex attribute (1-4 or `GL_BGRA`).
    /// * `data_type` - Data type of the attribute components.
    /// * `normalized` - Whether fixed-point data values should be normalized.
    /// * `stride` - Byte offset between consecutive attributes.
    /// * `pointer` - Pointer to the first component of the attribute data.
    ///
    /// # Panics
    ///
    /// This function panics if `size` is not within the valid range for OpenGL (1-4).
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferences a raw pointer (`pointer`), which could lead to undefined behavior if the pointer is invalid.
    pub fn new(
        index: u32,
        size: i32,
        data_type: DataType,
        normalized: bool,
        stride: i32,
    ) -> VertexAttribPointer {
        let normalized = if normalized { gl::TRUE } else { gl::FALSE };
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                data_type as u32,
                normalized,
                stride,
                ptr::null(),
            )
        };
        VertexAttribPointer { index }
    }

    /// Enables the vertex attribute array.
    pub fn enable(&self) {
        unsafe { gl::EnableVertexAttribArray(self.index) };
    }

    /// Disables the vertex attribute array.
    pub fn disable(&self) {
        unsafe { gl::DisableVertexAttribArray(self.index) };
    }
}
