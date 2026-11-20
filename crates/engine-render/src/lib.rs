pub struct RenderConfig {
    pub enable_validation: bool,
    pub width: u32,
    pub height: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            enable_validation: cfg!(debug_assertions),
            width: 1280,
            height: 720,
        }
    }
}
