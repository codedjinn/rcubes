
use glm::{Vec3,Mat4};

use super::base::{Camera, Projection};

// make this central global const later
pub struct FreeCamera {

    pos: Vec3,
    look_at: Vec3,
    dir: Vec3,
    cam_right: Vec3,
    cam_up: Vec3,

    view: Mat4,
    proj: Mat4,

    frustrum: Projection
}

impl Camera for FreeCamera {
    
    fn initialize() -> Self { 
        
        FreeCamera {
            pos: Vec3::new(0.0,0.0,0.0),
            look_at: Vec3::new(0.0,0.0,0.0),
            dir: Vec3::new(0.0,0.0,0.0),
            cam_right: Vec3::new(0.0,0.0,0.0),
            cam_up: Vec3::new(0.0,0.0,0.0),

            view: glm::identity(),
            proj: glm::identity(),

            frustrum: Projection::default()
        }
    }

    fn update(&mut self, time: f32) {

        let new_dir: Vec3 = self.pos - self.look_at;
        self.dir = new_dir.normalize();
        
        let up = Vec3::new(0.0,1.0,0.0);
        let cross = Vec3::cross(&up, &self.dir);
        
        self.cam_right = Vec3::normalize(&cross);
        self.cam_up = Vec3::cross(&self.dir, &self.cam_right);

        //self.view = glm::look_at()

    }
    
    fn get_eye(&self) -> &Vec3 { &self.pos }
    
    fn get_look_at(&self) -> &Vec3 { &self.look_at }
    
    fn get_view(&self) -> &Mat4 { &self.view }
    
    fn get_proj(&self) -> &Mat4 { &self.proj }
    
    fn get_frustum(&self) -> &Projection { &self.frustrum }
    
}