
use nalgebra;

pub enum Position {
    None,
    Value(nalgebra::Vec3)
}

pub struct Node {
    pos: Position
}

impl Node {

    pub fn new () -> Node {
        let pos:Position = Value(vec3(0,0,0));
        
    }

}

