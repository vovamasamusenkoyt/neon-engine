use std::sync::Arc;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub trait WindowSurface: HasWindowHandle + HasDisplayHandle + Send + Sync {}
impl<T: HasWindowHandle + HasDisplayHandle + Send + Sync> WindowSurface for T {}

pub enum Event {
    Resized { width: u32, height: u32 },
    Touch { id: u64, x: f32, y: f32, pressed: bool },
    Key { code: u32, pressed: bool },
    CloseRequested,
}

pub trait PlatformBackend {
    fn run<F>(self, event_handler: F) where F: FnMut(Event) + 'static;
    fn window(&self) -> Arc<dyn WindowSurface>;
}
