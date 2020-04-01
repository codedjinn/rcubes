
use web_sys::{WebGlRenderingContext};

use super::camera::Camera;

//
// Basic scene-tree that can have mutliple roots.  The scene itself is 
// anchored at origin 0,0,0
//
// work in progress, probably gonna change a lot
//

// not gonna code things too modular, but at least have the flexibility
// of specifying different cameras like Free,FPS,RTS,Debug,etc....
pub struct Scene<C> where
    C : Camera
{
    nodes: Vec<i32>,
    context: WebGlRenderingContext,
    camera: C
}

impl<C> Scene<C> {

    pub fn new(context: WebGlRenderingContext) -> Self {

        let camera: C = C::new();
        
        Scene {
            nodes: Vec::new(),
            context: context,
            camera: camera
        }

    }

}


