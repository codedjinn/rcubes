
//TOOD: clean-up once I have something that is running

extern crate console_error_panic_hook;

extern crate nalgebra as na;

mod core;
mod framework;
mod render;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use na::{Vector3, Point3};

use framework::Scene;
use framework::FreeCamera;

// struct FreeCamera {
// }
// impl Camera for FreeCamera {

//     fn initialize() {
//     }

//     fn update(time: f32) {

//     }
// }

#[wasm_bindgen]
pub struct Client {

    running: bool,
    time_stamp: u128,
    gl: WebGlRenderingContext,
    scene: Scene<FreeCamera>,
    divTest: web_sys::Element

}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// thanks to rust wasm examples on https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
#[macro_use]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
impl Client {

    pub fn new() -> Client {
        console_error_panic_hook::set_once();

        console_log!("Creating new client instance...");

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let divElement = document.get_element_by_id("test").unwrap();

        let context = canvas
            .get_context("webgl").unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>().unwrap();

        let scene: Scene<FreeCamera> = framework::Scene::new(context.clone());
        
        return Client {
            running: true,
            time_stamp: 0,
            gl: context,
            scene, 
            divTest: divElement
        };

    }

    pub fn main_loop(&mut self, time: i32) -> Result<(), JsValue> {

        // personally more predictable to work with seconds doing calculations
        let time_as_second = (time as f32) / 1000f32;

        // test output on browser
        // self.divTest.set_inner_html(&time_as_second.to_string());

        self.update(time_as_second);

        Ok(())
    }

    fn update(&self, deltaTime: f32) {
    }

    pub fn is_running(&self) -> bool { return self.running; }
}


// how to mock WebGlRenderingContext ?
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn main_loop() {

//         let client = new Client(){};

//     }
// }