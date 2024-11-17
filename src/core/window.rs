use glfw::Context;

pub struct Window {
    pwindow: glfw::PWindow,
    glfw: glfw::Glfw,
    title: String,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        let title = String::from(title);
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Couldn't init GLFW!");

        let (mut pwindow, events) = glfw
            .create_window(width, height, &title, glfw::WindowMode::Windowed)
            .expect("Couldn't create GLFW Window");

        pwindow.set_key_polling(true);
        pwindow.make_current();

        Window { title, pwindow, glfw }
    }

    pub fn update(&mut self) {
        self.pwindow.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn should_close(&self) -> bool {
        self.pwindow.should_close()
    }

    pub fn set_should_close(&mut self, value: bool) {
        self.pwindow.set_should_close(value);
    }
}
