
//TOOD: clean-up once I have something that is running

extern crate console_error_panic_hook;

extern crate nalgebra_glm as glm;

mod core;
mod framework;
mod render;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader };

use glm::{vec3};

use framework::Scene;
use framework::FreeCamera;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// thanks to rust wasm examples on https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Client {

    running: bool,
    time_stamp: u128,
    gl: WebGl2RenderingContext,
    scene: Scene<FreeCamera>,
    renderer: render::Renderer,
    divTest: web_sys::Element

}

#[wasm_bindgen]
impl Client {

    pub fn new() -> Client {
        console_error_panic_hook::set_once();

        console_log!("Creating new client instance...");

        console_log!("calling get method");
        core::get(String::from("http://localhost:8080/assets/file.txt"));//(String::from("http://localhost:8080/assets/file.txt"));

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let divElement = document.get_element_by_id("test").unwrap();

        let context = canvas
            .get_context("webgl2").unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>().unwrap();

        let scene: Scene<FreeCamera> = framework::Scene::new(context.clone());
        let mut renderer = render::Renderer::new(context.clone());
        renderer.intiialize();

        return Client {
            running: true,
            time_stamp: 0,
            gl: context,
            scene,
            renderer,
            divTest: divElement
        };

    }

    pub fn main_loop(&mut self, time: i32) -> Result<(), JsValue> {

        // personally more predictable to work with seconds doing calculations
        let time_as_seconds = (time as f32) / 1000f32;

        // TODO: when things get more solid, see if we can't push
        // update into a web worker
        // test output on browser
        // self.divTest.set_inner_html(&time_as_second.to_string());
        self.update(time_as_seconds);

        self.renderer.render(time_as_seconds);

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