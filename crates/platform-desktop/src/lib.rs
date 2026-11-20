use platform::PlatformBackend;

pub struct DesktopBackend {
    pub title: String,
}

impl DesktopBackend {
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into() }
    }
}
