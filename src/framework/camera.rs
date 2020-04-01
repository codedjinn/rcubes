
use super::base::{Updateable,Initializer};

pub trait Camera : Initializer + Updateable {
    fn new() -> Self { Self{}}
} // : Initializer + Updateable {}
