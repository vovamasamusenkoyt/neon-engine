use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use crate::PlatformEvent;

pub struct AndroidAppRunner {
    pub app: AndroidApp,
}

impl AndroidAppRunner {
    pub fn new(app: AndroidApp) -> Self {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Info)
                .with_tag("NeonEngine"),
        );
        Self { app }
    }

    pub fn poll_events<F>(&self, mut handler: F) -> bool
    where
        F: FnMut(PlatformEvent),
    {
        let mut running = true;
        self.app.poll_events(Some(std::time::Duration::from_millis(0)), |event| {
            match event {
                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    if let Some(win) = self.app.native_window() {
                        handler(PlatformEvent::Resized {
                            width: win.width() as u32,
                            height: win.height() as u32,
                        });
                    }
                }
                PollEvent::Main(MainEvent::WindowResized { .. }) => {
                    if let Some(win) = self.app.native_window() {
                        handler(PlatformEvent::Resized {
                            width: win.width() as u32,
                            height: win.height() as u32,
                        });
                    }
                }
                PollEvent::Main(MainEvent::TerminateWindow { .. }) => {
                    handler(PlatformEvent::CloseRequested);
                }
                PollEvent::Main(MainEvent::Destroy) => {
                    running = false;
                }
                _ => {}
            }
        });

        if let Ok(mut iter) = self.app.input_events_iter() {
            while iter.next(|_event| InputStatus::Handled) {}
        }

        running
    }
}
