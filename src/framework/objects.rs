
use na::{ Vector3, Isometry3 };

use web_sys::{WebGlRenderingContext};

pub struct Entity {
    pos: na::Vector3<f32>,
    rot: na::Isometry3<f32>,
    scale: na::Vector3<f32>,

    enabled: bool,

    context: Option<WebGlRenderingContext>
}

impl Entity {

    pub fn new() -> Node {
        let pos: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
        let scale: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
        
        Node {
            pos: pos,
            rot: na::Isometry3::identity(),
            scale: scale,
            enabled: true,
            nodes: Vec::new(),
            context: None
        }
    }

    pub fn update(&self, time: f32) {

    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

}


trait Component<T> {

}