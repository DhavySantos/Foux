pub(crate) struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram {
    pub(crate) fn new(vertex: &str, fragment: &str) -> Self {
        unsafe {
            let id = gl::CreateProgram();

            let vertex = compile_shader(vertex, gl::VERTEX_SHADER);
            let fragment = compile_shader(fragment, gl::FRAGMENT_SHADER);

            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);

            gl::LinkProgram(id);

            gl::DetachShader(id, vertex);
            gl::DetachShader(id, fragment);

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            return Self { id };
        }
    }

    pub(crate) fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) }
    }
}

fn compile_shader(src: &str, ty: gl::types::GLenum) -> gl::types::GLuint {
    unsafe {
        let shader = gl::CreateShader(ty);
        gl::ShaderSource(
            shader,
            1,
            &src.as_bytes().as_ptr().cast(),
            &src.len().try_into().unwrap(),
        );
        gl::CompileShader(shader);

        let mut success = 0;

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;

            gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());

            panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }

        return shader;
    }
}

#[cfg(test)]
pub mod tests {
    use rusty_fork::rusty_fork_test;

    rusty_fork_test! {
        #[test]
        fn shader_new() {
            let _window = crate::core::Window::new();

            let vertex: &str = r"
                #version 330 
                in vec3 position;
                void main() {
                    gl_Position = vec4(position, 1.0);
                }
            ";

            let fragment: &str = r"
                #version 330 core

                out vec4 FragColor;
                in vec3 color;

                void main() {
                    FragColor = vec4(color, 1.0f);
                }
            ";

            let _shader = crate::wrapper::ShaderProgram::new(vertex, fragment);
        }
    }
}

