
use glm::{Vec3,Mat4};

pub struct Projection {
    pub znear: f32,
    pub zfar: f32,
    pub aspect_ratio: f32,
    pub some: f32
}

impl Projection {

    pub fn default() -> Self {
        Projection {
            znear: 1.0,
            zfar: 1000.0,
            aspect_ratio: 1.3333,
            some: 1000.0
        }
    }

}

// credits to 
pub trait Camera {

    fn initialize() -> Self;

    fn update(&mut self, time: f32);

    // looks like modern terminology for position
    fn get_eye(&self) -> &Vec3;
    fn get_look_at(&self) -> &Vec3;

    fn get_view(&self) -> &Mat4;
    fn get_proj(&self) -> &Mat4;

    fn get_frustum(&self) -> &Projection;
}

