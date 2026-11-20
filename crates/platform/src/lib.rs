use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub trait WindowSurface: HasWindowHandle + HasDisplayHandle + Send + Sync {}
impl<T: HasWindowHandle + HasDisplayHandle + Send + Sync> WindowSurface for T {}

#[derive(Debug, Clone)]
pub enum PlatformEvent {
    Resized { width: u32, height: u32 },
    Touch { id: u64, x: f32, y: f32, pressed: bool },
    CursorMoved { x: f32, y: f32 },
    MouseButton { button: u32, pressed: bool },
    Key { code: u32, pressed: bool },
    CloseRequested,
    RedrawRequested,
}

#[cfg(not(target_os = "android"))]
pub mod linux;
#[cfg(not(target_os = "android"))]
pub use linux::*;

#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "android")]
pub use android::*;
