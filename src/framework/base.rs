
use nalgebra::{Point3, Vector3, Isometry3, Perspective3};

// credits to 
pub trait Camera {

    fn initialize() -> Self;

    fn update(&self, time: f32);

    fn get_pos(&self) -> Point3<f32>;

    fn get_frustum(&self) -> Perspective3<f32>;
    
}

