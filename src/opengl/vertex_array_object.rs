/// A struct representing a Vertex Array Object (VAO) in OpenGL.
///
/// The `VertexArrayObject` is responsible for managing the OpenGL Vertex Array Object (VAO),
/// which stores the configuration of vertex attribute pointers, element index buffers,
/// and other state related to rendering vertices.
///
/// # Example
/// ```rust
/// let vao = VertexArrayObject::new();
/// vao.bind();
/// // Perform rendering operations
/// vao.unbind();
/// ```
pub struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    /// Creates a new `VertexArrayObject`.
    ///
    /// This method initializes the VAO by calling OpenGL's `GenVertexArrays` function,
    /// which generates a unique ID for the VAO.
    ///
    /// # Returns
    /// A new `VertexArrayObject` with a generated OpenGL ID.
    ///
    /// # Example
    /// ```rust
    /// let vao = VertexArrayObject::new();
    /// ```
    pub fn new() -> VertexArrayObject {
        VertexArrayObject::default()
    }

    /// Binds the current `VertexArrayObject` to the OpenGL context.
    ///
    /// This method sets the OpenGL state to use the current `VertexArrayObject`,
    /// meaning subsequent OpenGL calls will modify or use the state of this VAO.
    ///
    /// # Example
    /// ```rust
    /// vao.bind();
    /// ```
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    /// Unbinds the current `VertexArrayObject` from the OpenGL context.
    ///
    /// This method sets the OpenGL state to use the default (unbound) VAO,
    /// which means no VAO will be active for subsequent OpenGL calls.
    ///
    /// # Example
    /// ```rust
    /// vao.unbind();
    /// ```
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

impl Default for VertexArrayObject {
    /// Creates a default `VertexArrayObject` by generating a new OpenGL VAO.
    ///
    /// This function calls OpenGL's `GenVertexArrays` to generate a unique ID for the VAO
    /// and returns a `VertexArrayObject` with that ID.
    ///
    /// # Returns
    /// A `VertexArrayObject` with a generated OpenGL ID.
    ///
    /// # Safety
    /// This method requires unsafe code because it directly interacts with the OpenGL API.
    fn default() -> Self {
        let mut id = 0;
        unsafe { gl::GenVertexArrays(1, &mut id) };
        VertexArrayObject { id }
    }
}

