
use web_sys::{WebGl2RenderingContext};

use super::base::Camera;

//
// Basic scene-tree that can have mutliple roots.  The scene itself is 
// anchored at origin 0,0,0
//
// work in progress, probably gonna change a lot
//

// not gonna code things too modular, but at least have the flexibility
// of specifying different cameras like Free,FPS,RTS,Debug,etc....
pub struct Scene<Cam>
{
    nodes: Vec<i32>,
    context: WebGl2RenderingContext,
    camera: Cam
}

impl<Cam> Scene<Cam> where Cam: Camera {

    pub fn new(context: WebGl2RenderingContext) -> Self {

        let camera: Cam = Cam::initialize();
        
        Scene {
            nodes: Vec::new(),
            context: context,
            camera: camera
        }

    }

    pub fn update(&self, time: f32) {
    }

}

