
use na::{ Perspective3 };

pub struct Camera {


}

impl Camera {
    
    pub fn new() -> Camera {

        let persp = Perspective3::new(1.3, 45.0, 0.1, 1000.0);

        Camera { }
    }

}