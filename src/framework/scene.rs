
use super::node::Node;


use web_sys::{WebGlRenderingContext};

//
// Basic scene-tree that can have mutliple roots.  The scene itself is 
// anchored at origin 0,0,0
//


pub struct Scene {

    nodes: Vec<Node>,

    context: WebGlRenderingContext

}

impl Scene {

    pub fn new(context: WebGlRenderingContext) -> Scene {
        Scene {

            context,
            nodes: Vec::new(),

        }
    }

    pub fn update(&self, time: f32) {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.is_enabled() {
                node.update(time);
            }
        }
    }
}

