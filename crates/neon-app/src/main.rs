#[cfg(not(target_os = "android"))]
mod desktop {
    use std::sync::Arc;
    use engine_core::MenuAction;
    use engine_render::VulkanContext;
    use glam::Vec2;
    use neon_app::EngineApp;
    use vulkano::swapchain::Surface;
    use winit::application::ApplicationHandler;
    use winit::dpi::PhysicalSize;
    use winit::event::{ElementState, MouseButton, WindowEvent};
    use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
    use winit::keyboard::{Key, NamedKey};
    use winit::window::{Window, WindowId};

    struct DesktopApp {
        context: Option<VulkanContext>,
        window: Option<Arc<Window>>,
        engine: EngineApp,
        cursor_pos: Vec2,
    }

    impl ApplicationHandler for DesktopApp {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            if self.window.is_none() {
                let win_attrs = Window::default_attributes()
                    .with_title("Neon Engine // reVC Frontend")
                    .with_inner_size(PhysicalSize::new(1280, 720));

                let window = Arc::new(event_loop.create_window(win_attrs).expect("failed to create window"));
                self.window = Some(window.clone());

                let extensions = Surface::required_extensions(event_loop).expect("failed to get surface extensions");
                let context = VulkanContext::new(extensions).expect("failed to create VulkanContext");
                let surface = Surface::from_window(context.instance.clone(), window.clone()).expect("failed to create surface");

                let size = window.inner_size();
                self.engine.init(context.clone(), surface, [size.width, size.height]).expect("failed to init engine");
                self.context = Some(context);
            }
        }

        fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    self.engine.resize([size.width, size.height]);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let pos = [position.x as f32, position.y as f32];
                    self.cursor_pos = Vec2::new(pos[0], pos[1]);
                    self.engine.cursor_pos = Some(pos);
                    self.engine.on_cursor_move(self.cursor_pos);
                }
                WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                    let action = self.engine.on_touch(self.cursor_pos, true);
                    log::info!("Mouse click at {:?} -> action: {:?}", self.cursor_pos, action);
                    if action == MenuAction::Quit {
                        event_loop.exit();
                    }
                }
                WindowEvent::KeyboardInput { event: key_event, .. } => {
                    if key_event.state == ElementState::Pressed {
                        let action = match key_event.logical_key {
                            Key::Named(NamedKey::ArrowUp) => self.engine.on_key(38, true),
                            Key::Named(NamedKey::ArrowDown) => self.engine.on_key(40, true),
                            Key::Named(NamedKey::Enter) => self.engine.on_key(13, true),
                            Key::Named(NamedKey::Escape) => self.engine.on_key(27, true),
                            _ => MenuAction::None,
                        };
                        if action == MenuAction::Quit {
                            event_loop.exit();
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    let _ = self.engine.render_frame();
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

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();
        log::info!("Neon Engine Desktop Starting...");

        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = DesktopApp {
            context: None,
            window: None,
            engine: EngineApp::new(),
            cursor_pos: Vec2::ZERO,
        };

        event_loop.run_app(&mut app)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(target_os = "android"))]
    {
        desktop::run()
    }
    #[cfg(target_os = "android")]
    {
        Ok(())
    }
}
