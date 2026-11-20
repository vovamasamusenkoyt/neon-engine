pub mod menu;

pub use glam::*;
pub use menu::*;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, -5.0),
            yaw: 0.0,
            pitch: 0.0,
            fov_y: 45.0f32.to_radians(),
            aspect,
            z_near: 0.1,
            z_far: 1000.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let rot = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        let forward = rot * Vec3::Z;
        Mat4::look_to_rh(self.position, forward, Vec3::Y)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov_y, self.aspect, self.z_near, self.z_far)
    }
}
