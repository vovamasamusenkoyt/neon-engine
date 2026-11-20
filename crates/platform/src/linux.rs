use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::{PlatformEvent, WindowSurface};

pub struct LinuxApp<F> {
    title: String,
    width: u32,
    height: u32,
    window: Option<Arc<Window>>,
    event_handler: F,
}

impl<F> LinuxApp<F>
where
    F: FnMut(&Arc<dyn WindowSurface>, PlatformEvent),
{
    pub fn new(title: impl Into<String>, width: u32, height: u32, handler: F) -> Self {
        Self {
            title: title.into(),
            width,
            height,
            window: None,
            event_handler: handler,
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl<F> ApplicationHandler for LinuxApp<F>
where
    F: FnMut(&Arc<dyn WindowSurface>, PlatformEvent),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attrs = Window::default_attributes()
                .with_title(&self.title)
                .with_inner_size(PhysicalSize::new(self.width, self.height));
            let window = Arc::new(event_loop.create_window(win_attrs).expect("create window failed"));
            self.window = Some(window.clone());
            let surface: Arc<dyn WindowSurface> = window;
            (self.event_handler)(&surface, PlatformEvent::Resized {
                width: self.width,
                height: self.height,
            });
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(window) = &self.window else { return };
        let surface: Arc<dyn WindowSurface> = window.clone();

        match event {
            WindowEvent::CloseRequested => {
                (self.event_handler)(&surface, PlatformEvent::CloseRequested);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                (self.event_handler)(
                    &surface,
                    PlatformEvent::Resized {
                        width: size.width,
                        height: size.height,
                    },
                );
            }
            WindowEvent::RedrawRequested => {
                (self.event_handler)(&surface, PlatformEvent::RedrawRequested);
            }
            WindowEvent::CursorMoved { position, .. } => {
                (self.event_handler)(
                    &surface,
                    PlatformEvent::CursorMoved {
                        x: position.x as f32,
                        y: position.y as f32,
                    },
                );
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    winit::event::MouseButton::Left => 0,
                    winit::event::MouseButton::Right => 1,
                    winit::event::MouseButton::Middle => 2,
                    winit::event::MouseButton::Other(v) => v as u32,
                    _ => 0,
                };
                (self.event_handler)(
                    &surface,
                    PlatformEvent::MouseButton {
                        button: btn,
                        pressed: state == ElementState::Pressed,
                    },
                );
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
