use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::ptr;

use nalgebra_glm::Mat4;

/// Represents an OpenGL shader program, providing utilities for shader management
/// and uniform variable handling.
pub struct ShaderProgram {
    /// Stores uniform variable locations by name.
    uniforms_ids: HashMap<String, i32>,
    /// OpenGL program ID.
    program: u32,
}

impl ShaderProgram {
    /// Creates a new `ShaderProgram` by compiling and linking vertex and fragment shaders.
    ///
    /// # Arguments
    ///
    /// * `vertex_src` - GLSL source code for the vertex shader.
    /// * `fragment_src` - GLSL source code for the fragment shader.
    ///
    /// # Panics
    ///
    /// This function will panic if shader compilation or program linking fails.
    pub fn new(vertex_src: &str, fragment_src: &str) -> ShaderProgram {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_src).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            Self::check_shader_compile_status(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_src).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            Self::check_shader_compile_status(fragment_shader);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            Self::check_program_link_status(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram {
                uniforms_ids: HashMap::new(),
                program,
            }
        }
    }

    /// Checks the compile status of a shader and panics if compilation failed.
    ///
    /// # Safety
    ///
    /// This function calls unsafe OpenGL functions.
    unsafe fn check_shader_compile_status(shader: u32) {
        let mut success = gl::FALSE as i32;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as i32 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut log = vec![0; len as usize];
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            panic!(
                "Shader compilation failed: {}",
                String::from_utf8_lossy(&log)
            );
        }
    }

    /// Checks the link status of the shader program and panics if linking failed.
    ///
    /// # Safety
    ///
    /// This function calls unsafe OpenGL functions.
    unsafe fn check_program_link_status(program: u32) {
        let mut success = gl::FALSE as i32;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as i32 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut log = vec![0; len as usize];
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            panic!("Program linking failed: {}", String::from_utf8_lossy(&log));
        }
    }

    /// Creates a uniform variable and stores its location for later use.
    ///
    /// # Arguments
    ///
    /// * `uniform_name` - The name of the uniform variable in the shader.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the uniform was located successfully, or an `Err(String)` if not.
    pub fn create_uniform(&mut self, uniform_name: &str) -> Result<(), String> {
        let uniform_location = unsafe {
            let name = CString::new(uniform_name).unwrap();
            gl::GetUniformLocation(self.program, name.as_ptr())
        };

        if uniform_location < 0 {
            return Err(format!("Cannot locate uniform: {}", uniform_name));
        }

        self.uniforms_ids
            .insert(String::from(uniform_name), uniform_location);

        Ok(())
    }

    /// Sets the value of a `mat4` uniform variable.
    ///
    /// # Arguments
    ///
    /// * `uniform_name` - The name of the uniform variable.
    /// * `matrix` - A reference to a `Matrix4<f32>` containing the new value.
    ///
    /// # Panics
    ///
    /// This function will panic if the uniform is not found in the `uniforms_ids` map.
    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &Mat4) {
        if let Some(&location) = self.uniforms_ids.get(uniform_name) {
            unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr()) };
        }

        panic!("Uniform '{uniform_name}' not found. Did you forget to call `create_uniform`?");
    }

    /// Binds the shader program for use in the OpenGL pipeline.
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program) }
    }

    /// Unbinds any shader program from the OpenGL pipeline.
    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) }
    }
}

impl Drop for ShaderProgram {
    /// Automatically deletes the OpenGL program when the `ShaderProgram` is dropped.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}
