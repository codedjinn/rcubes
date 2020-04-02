
use nalgebra::{Point3, Vector3, Isometry3, Perspective3};

// credits to 
pub trait Camera {

    fn initialize() -> Self;

    fn update(&self, time: f32);

    fn get_pos(&self) -> Vector3<f32>;
    fn set_pos(&self, value: Vector3<f32>);
    fn get_frustum(&self) -> Perspective3<f32>;
    fn get_view(&self) -> Isometry3<f32>;
    fn get_transform(&self) -> Matrix4<f32>;

}

