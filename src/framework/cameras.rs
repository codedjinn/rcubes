
use na::{Perspective3, Matrix4, Vector3};

use super::base::Camera;
use nalgebra::{Isometry3, Matrix};

pub struct FreeCamera {
    
    pos: Vector3<f32>,
    look_at: Vector3<f32>,

    transform: Matrix4<f32>,
    view: Matrix4<f32>,
    proj: Matrix4<f32>,

    frustrum: Perspective3<f32>
}

impl Camera for FreeCamera {
    
    fn initialize() -> Self { 
        
        FreeCamera {
            pos: na::zero(),
            look_at: na::zero(),

            transform: na::zero(),
            view: na::zero(),
            proj: na::zero(),

            frustrum: Perspective3::new(16.0 / 9.0, 3.14 / 4.0, 1.0, 1000.0)
        }
    }

    fn update(&self, time: f32) {
    }

    
    fn get_pos(&self) -> Vector3<f32> { self.pos }
    fn set_pos(&self, value: Vector3<f32>) { self.pos = value; }

    fn get_view(&self) -> Isometry3<f32> { self.view }

    fn get_transform(&self) -> Matrix4<f32> {self. transform }
    
    fn get_frustum(&self) -> Perspective3<f32> {
        na::zero()
    }
}