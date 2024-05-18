use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use super::Window;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: Window::new(),
        }
    }

    pub fn run(&mut self) {
        let mut latest_second = Instant::now();
        let mut latest_frame = Instant::now();
        let mut frames = 0;

        while !self.window.should_close() {
            let delta = Instant::now().duration_since(latest_frame);
            latest_frame = Instant::now();

            self.window.clear();

            self.window.update();

            let elapsed = Instant::now().duration_since(latest_frame);

            if let Some(time) = self.window.target_frametime().checked_sub(elapsed) {
                sleep(time);
            }

            if Instant::now() - latest_second >= Duration::from_secs_f64(1.0) {
                let title = format!(
                    "foux engine fps: {frames} | ftime: {}ms",
                    delta.as_secs_f64()
                );
                self.window.set_title(title.as_str());
                latest_second = Instant::now();
                frames = 0;
            }

            frames += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_fork::rusty_fork_test;

    rusty_fork_test! {
        #[test]
        fn app_run() {
            let mut app = App::new();

            app.run();
        }
    }
}
