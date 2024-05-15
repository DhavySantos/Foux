use crate::wrapper::{BufferObject, ShaderProgram, VertexArrayObject, VertexAttribute};

pub struct Renderer {
    vertex_array_object: VertexArrayObject,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    shader: ShaderProgram,
}

impl Renderer {
    pub fn new(vertices: &[f32], indices: &[u32], shader: ShaderProgram) -> Self {
        let vertex_array_object = VertexArrayObject::new();
        let vertex_buffer_object = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        let indices_buffer_object = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);

        vertex_array_object.bind();

        vertex_buffer_object.bind();
        vertex_buffer_object.data(vertices);

        indices_buffer_object.bind();
        indices_buffer_object.data(indices);

        let vertex_attribute = VertexAttribute::new(0);

        vertex_attribute.enable();

        vertex_buffer_object.unbind();
        vertex_array_object.unbind();

        indices_buffer_object.unbind();
        vertex_array_object.unbind();

        Self {
            vertex_array_object,
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            shader,
        }
    }

    pub(crate) fn render(&self) {
        unsafe {
            self.shader.use_program();
            self.vertex_array_object.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                self.vertices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Renderer, Window},
        wrapper::ShaderProgram,
    };
    use rusty_fork::rusty_fork_test;

    rusty_fork_test! {
        #[test]
        fn draw_triangle() {
            let mut window = Window::new();

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

                vec3 hsv2rgb(vec3 c) {
                    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
                    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
                    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
                }

                void main() {
                    float hue = mod(gl_FragCoord.x + gl_FragCoord.y, 360.0) / 360.0;
                    vec3 rgbColor = hsv2rgb(vec3(hue, 1.0, 1.0));
                    FragColor = vec4(rgbColor, 1.0);
                }
            ";


            let shader = ShaderProgram::new(vertex, fragment);
            let triangle = [0.0, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];
            let indices = [0, 1, 2];

            let renderer = Renderer::new(&triangle, &indices, shader);

            let mut count = 0;

            while !window.should_close() {
                window.clear(0.0, 0.0, 0.0, 0.0);
                renderer.render();
                window.update();
                count += 1;
                if count > 60 * 3 { window.set_should_close(true) }
                std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
            }
        }
    }

    rusty_fork_test! {
        #[test]
        fn draw_square() {

            let mut window = Window::new();

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

                vec3 hsv2rgb(vec3 c) {
                    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
                    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
                    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
                }

                void main() {
                    float hue = mod(gl_FragCoord.x + gl_FragCoord.y, 360.0) / 360.0;
                    vec3 rgbColor = hsv2rgb(vec3(hue, 1.0, 1.0));
                    FragColor = vec4(rgbColor, 1.0);
                }
            ";

            let shader = ShaderProgram::new(vertex, fragment);

            let square = [
                -0.5, -0.5, 0.0,
                0.5, -0.5, 0.0,
                0.5, 0.5, 0.0,
                -0.5, 0.5, 0.0
            ];

            let indices = [0, 1, 2, 0, 2, 3];

            let renderer = Renderer::new(&square, &indices, shader);

            let mut count = 0;

            while !window.should_close() {
                window.clear(0.0, 0.0, 0.0, 1.0);
                renderer.render();
                window.update();
                count += 1;
                if count > 60 * 3 { window.set_should_close(true) }
                std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
            }
        }
    }
}
