use crate::opengl::{BufferObject, ShaderProgram, VertexArrayObject, VertexAttribPointer};
use crate::opengl::{BufferTarget, BufferUsage, DataType};

use std::mem::size_of;

/// A struct responsible for rendering a mesh in OpenGL.
///
/// It handles setting up vertex buffers, element buffers, and shader programs
/// for rendering a 3D object using OpenGL.
pub struct MeshRenderer {
    vertex_array: VertexArrayObject, // Vertex array object for binding
    shader_program: ShaderProgram,   // Shader program for rendering
    element_buffer: BufferObject,    // Element buffer object (index buffer)
    vertex_buffer: BufferObject,     // Vertex buffer object
    vertices: Vec<f32>,              // Vertex data (positions, normals, etc.)
    indicies: Vec<u32>,              // Index data for elements
}

impl MeshRenderer {
    /// Creates a new `MeshRenderer` with provided vertex and index data, and shader program.
    ///
    /// # Arguments
    ///
    /// * `vertices` - A vector of `f32` representing the vertex data (e.g., positions).
    /// * `indicies` - A vector of `u32` representing the index data (element indices).
    /// * `shaders` - The `ShaderProgram` to use for rendering the mesh.
    ///
    /// # Returns
    ///
    /// A new instance of `MeshRenderer`.
    pub fn new(vertices: Vec<f32>, indicies: Vec<u32>, shaders: ShaderProgram) -> MeshRenderer {
        let vertex_array = VertexArrayObject::new();
        let vertex_buffer = BufferObject::new(BufferTarget::ArrayBuffer, BufferUsage::StaticDraw);
        let element_buffer = BufferObject::new(BufferTarget::ElementArrayBuffer, BufferUsage::StaticDraw);

        vertex_array.bind();

        vertex_buffer.bind();
        vertex_buffer.data(&vertices);

        element_buffer.bind();
        element_buffer.data(&indicies);

        // Setup vertex attribute pointer
        let vertex_attrib_pointer = VertexAttribPointer::new(0, 3, DataType::Float, false, 3 * size_of::<f32>() as i32);
        vertex_attrib_pointer.enable();

        // Unbind buffers and vertex array to clean up state
        vertex_buffer.unbind();
        vertex_array.unbind();
        element_buffer.unbind();

        MeshRenderer {
            shader_program: shaders,
            element_buffer,
            vertex_buffer,
            vertex_array,
            vertices,
            indicies,
        }
    }

    /// Updates the vertex data of the mesh.
    ///
    /// # Arguments
    ///
    /// * `vertices` - A vector of `f32` representing the new vertex data.
    ///
    /// This method binds the vertex buffer, uploads the new vertex data to OpenGL,
    /// and unbinds the buffer after the update.
    pub fn set_vertices(&mut self, vertices: Vec<f32>) {
        self.vertex_buffer.bind();
        self.vertex_buffer.data(&vertices);
        self.vertex_buffer.unbind();
        self.vertices = vertices;
    }

    /// Updates the index data of the mesh.
    ///
    /// # Arguments
    ///
    /// * `indicies` - A vector of `u32` representing the new index data.
    ///
    /// This method binds the element buffer, uploads the new index data to OpenGL,
    /// and unbinds the buffer after the update.
    pub fn set_indices(&mut self, indicies: Vec<u32>) {
        self.element_buffer.bind();
        self.element_buffer.data(&indicies);
        self.element_buffer.unbind();
        self.indicies = indicies;
    }

    /// Renders the mesh using the current shader program.
    ///
    /// This method binds the shader program and vertex array, and uses OpenGL's
    /// `DrawElements` to render the mesh based on the index data.
    ///
    /// OpenGL's context is used to draw the elements in the `TRIANGLES` mode.
    pub fn render(&self) {
        self.shader_program.bind();
        self.vertex_array.bind();

        // Ensure OpenGL context is set up correctly
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indicies.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
