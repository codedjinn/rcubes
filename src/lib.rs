
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

#[wasm_bindgen]
pub struct Client {

    running: bool,
    time_stamp: u128,
    gl: WebGlRenderingContext,
    scene: Scene,
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

        let scene = framework::Scene::new(context.clone());
        
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


// #[wasm_bindgen(start)]
// pub fn start() -> Result<(), JsValue> {

//     console_error_panic_hook::set_once();

//     let document = web_sys::window().unwrap().document().unwrap();
//     let canvas = document.get_element_by_id("canvas").unwrap();
//     let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

//     let context = canvas
//         .get_context("webgl")?
//         .unwrap()
//         .dyn_into::<WebGlRenderingContext>()?;
    
//     let vert_shader = compile_shader(
//         &context,
//         WebGlRenderingContext::VERTEX_SHADER,
//         r#"
//         attribute vec4 position;
//         void main() {
//             gl_Position = position;
//         }
//     "#,
//     )?;
//     let frag_shader = compile_shader(
//         &context,
//         WebGlRenderingContext::FRAGMENT_SHADER,
//         r#"
//         void main() {
//             gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
//         }
//     "#,
//     )?;
//     let program = link_program(&context, &vert_shader, &frag_shader)?;
//     context.use_program(Some(&program));

//     let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

//     let buffer = context.create_buffer().ok_or("failed to create buffer")?;
//     context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

//     // Note that `Float32Array::view` is somewhat dangerous (hence the
//     // `unsafe`!). This is creating a raw view into our module's
//     // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
//     // (aka do a memory allocation in Rust) it'll cause the buffer to change,
//     // causing the `Float32Array` to be invalid.
//     //
//     // As a result, after `Float32Array::view` we have to be very careful not to
//     // do any memory allocations before it's dropped.
//     unsafe {
//         let vert_array = js_sys::Float32Array::view(&vertices);

//         context.buffer_data_with_array_buffer_view(
//             WebGlRenderingContext::ARRAY_BUFFER,
//             &vert_array,
//             WebGlRenderingContext::STATIC_DRAW,
//         );
//     }

//     context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
//     context.enable_vertex_attrib_array(0);

//     context.clear_color(0.0, 0.0, 0.0, 1.0);
//     context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

//     context.draw_arrays(
//         WebGlRenderingContext::TRIANGLES,
//         0,
//         (vertices.len() / 3) as i32,
//     );

//     Ok(())
// }

