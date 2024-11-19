use glfw::Context;

/// Represents a window in a GLFW context.
pub struct Window {
    pwindow: glfw::PWindow,
    glfw: glfw::Glfw,
    title: String,
}

impl Window {
    /// Creates a new window with the specified title and dimensions.
    ///
    /// # Arguments
    ///
    /// * `title` - A string representing the title of the window.
    /// * `width` - The width of the window.
    /// * `height` - The height of the window.
    ///
    /// # Returns
    ///
    /// A `Window` object with the specified properties.
    ///
    /// # Panics
    ///
    /// This function will panic if GLFW initialization or window creation fails.
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        let title = String::from(title);
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Couldn't init GLFW!");

        // Set window hints for customization
        glfw.window_hint(glfw::WindowHint::X11ClassName(Some(title.clone())));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::Floating(true));

        // Create the window and capture events
        let (mut pwindow, events) = glfw
            .create_window(width, height, &title, glfw::WindowMode::Windowed)
            .expect("Couldn't create GLFW Window");

        // Load OpenGL functions
        gl::load_with(|s| pwindow.get_proc_address(s));

        // Enable key polling and make the window current
        pwindow.set_key_polling(true);
        pwindow.make_current();

        Window {
            pwindow,
            title,
            glfw,
        }
    }

    /// Updates the window, swapping buffers and polling for events.
    ///
    /// This function should be called each frame to maintain window responsiveness.
    pub fn update(&mut self) {
        self.pwindow.swap_buffers();
        self.glfw.poll_events();
    }

    /// Clears the window with the specified color.
    ///
    /// # Arguments
    ///
    /// * `red` - Red component of the clear color.
    /// * `green` - Green component of the clear color.
    /// * `blue` - Blue component of the clear color.
    /// * `alpha` - Alpha component of the clear color.
    ///
    /// # Safety
    ///
    /// This function calls OpenGL directly, which is unsafe.
    pub fn clear(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    /// Returns the current title of the window.
    ///
    /// # Returns
    ///
    /// A reference to the title of the window.
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Sets a new title for the window.
    ///
    /// # Arguments
    ///
    /// * `title` - The new title to set for the window.
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    /// Checks if the window should close.
    ///
    /// # Returns
    ///
    /// `true` if the window should close, `false` otherwise.
    pub fn should_close(&self) -> bool {
        self.pwindow.should_close()
    }

    /// Sets whether the window should close or not.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` if the window should close, `false` otherwise.
    pub fn set_should_close(&mut self, value: bool) {
        self.pwindow.set_should_close(value);
    }
}
