use std::sync::Arc;
use std::time::Instant;
use glam::{Mat4, Vec2, Vec3};
use engine_core::{CMenuManager, Camera, MenuAction, MenuPage};
use engine_render::{EngineRenderer, SwapchainTarget, VulkanContext};
use vulkano::swapchain::Surface;

pub struct EngineApp {
    pub renderer: Option<EngineRenderer>,
    pub camera: Camera,
    pub menu: CMenuManager,
    pub start_time: Instant,
    pub frame_count: u64,
    pub last_fps_time: Instant,
    pub current_fps_text: String,
    pub window_extent: [u32; 2],
    pub cursor_pos: Option<[f32; 2]>,
}

impl Default for EngineApp {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineApp {
    pub fn new() -> Self {
        Self {
            renderer: None,
            camera: Camera::new(16.0 / 10.0),
            menu: CMenuManager::new(),
            start_time: Instant::now(),
            frame_count: 0,
            last_fps_time: Instant::now(),
            current_fps_text: String::from("FPS: 0000"),
            window_extent: [1280, 720],
            cursor_pos: None,
        }
    }

    pub fn init(
        &mut self,
        context: VulkanContext,
        surface: Arc<Surface>,
        extent: [u32; 2],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let target = SwapchainTarget::new(&context, surface, Some(extent))?;
        let renderer = EngineRenderer::new(context, target)?;
        self.renderer = Some(renderer);
        self.resize(extent);
        Ok(())
    }

    pub fn resize(&mut self, extent: [u32; 2]) {
        if extent[0] == 0 || extent[1] == 0 {
            return;
        }
        self.window_extent = extent;
        self.camera.aspect = extent[0] as f32 / extent[1] as f32;
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(extent);
        }
    }

    pub fn on_key(&mut self, key_code: u32, pressed: bool) -> MenuAction {
        if !pressed {
            return MenuAction::None;
        }

        // Key mapping: Up (38), Down (40), Enter (13), Escape (27)
        match key_code {
            38 => { // Up Arrow
                self.menu.nav_up();
            }
            40 => { // Down Arrow
                self.menu.nav_down();
            }
            13 => { // Enter
                return self.menu.activate();
            }
            27 => { // Escape
                if self.menu.current_page != MenuPage::StartMenu {
                    self.menu.set_page(MenuPage::StartMenu);
                } else {
                    self.menu.is_active = !self.menu.is_active;
                }
            }
            _ => {}
        }
        MenuAction::None
    }

    pub fn on_touch(&mut self, pos: Vec2, pressed: bool) -> MenuAction {
        if !pressed || !self.menu.is_active {
            return MenuAction::None;
        }
        let win_size = Vec2::new(self.window_extent[0] as f32, self.window_extent[1] as f32);
        self.menu.handle_touch(pos, win_size)
    }

    pub fn render_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let Some(renderer) = &mut self.renderer else { return Ok(()) };

        let elapsed = self.start_time.elapsed().as_secs_f32();
        let model = Mat4::from_rotation_y(elapsed * 1.5) * Mat4::from_rotation_x(elapsed * 1.0);
        let view = Mat4::look_at_rh(Vec3::new(0.0, 1.8, 3.8), Vec3::ZERO, Vec3::Y);
        let mut proj = self.camera.projection_matrix();
        // Standard Vulkan Y-flip
        proj.y_axis.y = -proj.y_axis.y;

        let mvp = proj * view * model;

        if self.menu.is_active {
            renderer.draw_frame_with_overlay(
                mvp,
                true,
                self.menu.selected_option,
                self.cursor_pos,
                &[],
                &[],
            )?;
        } else {
            renderer.draw_frame(mvp, Some(&self.current_fps_text))?;
        }

        self.frame_count += 1;
        let now = Instant::now();
        let duration = now.duration_since(self.last_fps_time);
        if duration.as_secs_f32() >= 0.5 {
            let fps = self.frame_count as f32 / duration.as_secs_f32();
            self.current_fps_text = format!("FPS: {:.0}", fps);
            self.frame_count = 0;
            self.last_fps_time = now;
        }

        Ok(())
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("NeonEngine"),
    );

    log::info!("========================================");
    log::info!("  Neon Engine Starting (Android Vulkan) ");
    log::info!("========================================");

    let instance_extensions = vulkano::instance::InstanceExtensions {
        khr_surface: true,
        khr_android_surface: true,
        ..vulkano::instance::InstanceExtensions::empty()
    };

    let context = match VulkanContext::new(instance_extensions) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to create VulkanContext: {:?}", e);
            return;
        }
    };

    let mut engine_app = EngineApp::new();
    let mut running = true;
    let mut surface_created = false;

    while running {
        app.poll_events(Some(std::time::Duration::from_millis(0)), |event| {
            use android_activity::{MainEvent, PollEvent};
            match event {
                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    if let Some(native_window) = app.native_window() {
                        let win_ptr = native_window.ptr().as_ptr().cast();
                        match unsafe {
                            Surface::from_android(context.instance.clone(), win_ptr, None)
                        } {
                            Ok(surface) => {
                                let width = native_window.width() as u32;
                                let height = native_window.height() as u32;
                                if let Err(e) = engine_app.init(context.clone(), surface, [width, height]) {
                                    log::error!("Init error: {:?}", e);
                                } else {
                                    surface_created = true;
                                }
                            }
                            Err(e) => log::error!("Failed to create Android surface: {:?}", e),
                        }
                    }
                }
                PollEvent::Main(MainEvent::WindowResized { .. }) => {
                    if let Some(native_window) = app.native_window() {
                        let width = native_window.width() as u32;
                        let height = native_window.height() as u32;
                        engine_app.resize([width, height]);
                    }
                }
                PollEvent::Main(MainEvent::TerminateWindow { .. }) => {
                    surface_created = false;
                    engine_app.renderer = None;
                }
                PollEvent::Main(MainEvent::Destroy) => {
                    running = false;
                }
                _ => {}
            }
        });

        // Drain input events and handle touches for menu
        if let Ok(mut iter) = app.input_events_iter() {
            while iter.next(|event| {
                if let android_activity::input::InputEvent::MotionEvent(motion) = event {
                    let action = motion.action();
                    if action == android_activity::input::MotionAction::Down {
                        if let Some(pointer) = motion.pointers().next() {
                            let pos = Vec2::new(pointer.x(), pointer.y());
                            let menu_action = engine_app.on_touch(pos, true);
                            if menu_action == MenuAction::Quit {
                                running = false;
                            }
                        }
                    }
                }
                android_activity::InputStatus::Handled
            }) {}
        }

        if surface_created {
            if let Err(e) = engine_app.render_frame() {
                log::error!("Render frame error: {:?}", e);
            }
        }
    }
}
