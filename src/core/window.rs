extern crate glfw;

use std::time::Duration;

use glfw::{Context, OpenGlProfileHint::Core, WindowEvent};
use glfw::{Glfw, GlfwReceiver, PWindow};

type Events = GlfwReceiver<(f64, WindowEvent)>;

pub struct Window {
    target_framerate: f64,
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
            target_framerate: 60.0,
            window,
            events,
            glfw,
        }
    }

    pub(crate) fn set_vsync(&mut self, value: bool) {
        let state = if value { 1 } else { 0 };
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(state));
    }

    pub(crate) fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub(crate) fn target_frametime(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.target_framerate)
    }

    pub(crate) fn set_target_framerate(&mut self, target_framerate: f64) {
        self.target_framerate = target_framerate;
    }

    pub(crate) fn target_framerate(&self) -> f64 {
        self.target_framerate
    }

    pub(crate) fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub(crate) fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value);
    }

    pub(crate) fn update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub(crate) fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }
    }
}
