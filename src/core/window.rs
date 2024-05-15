extern crate glfw;

use glfw::{Context, OpenGlProfileHint::Core, WindowEvent};
use glfw::{Glfw, GlfwReceiver, PWindow};

type Events = GlfwReceiver<(f64, WindowEvent)>;

pub struct Window {
    window: PWindow,
    events: Events,
    glfw: Glfw,
}

impl Window {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        let (mut window, events) = glfw
            .create_window(600, 400, "Foux Engine", glfw::WindowMode::Windowed)
            .expect("failed to create glfw window!");

        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        window.make_current();
        window.set_key_polling(true);

        Self {
            window,
            events,
            glfw,
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value);
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(r, g, b, a);
        }
    }
}
