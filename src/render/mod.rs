
use crate::{log, console_log};

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

// imports
mod shader;
pub use shader::*;

mod pipeline;
pub use pipeline::*;

// trait SubRenderer {
    
//     fn restore_state();
//     fn render(&self, time: f32);
// }

const QuadPositions: [f32; 12] = [
    -1.0, -1.0,
     1.0, -1.0,
     1.0,  1.0,
     1.0,  1.0,
    -1.0,  1.0,
    -1.0, -1.0
];

const QuadTexCoords: [f32; 12] = [
    0.0, 0.0,
    1.0, 0.0,
    1.0, 1.0, 
    1.0, 1.0, 
    0.0, 1.0, 
    0.0, 0.0
];

const Triangles: [f32; 18] = [
     0.5, -0.5, 0.0,  1.0, 0.0, 0.0,  // bottom right
    -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  // bottom left
     0.0,  0.5, 0.0,  0.0, 1.0, 1.0   // top 
];

pub struct Renderer {
    context: WebGl2RenderingContext,

    shader: Shader,

    light_buffer: Option<WebGlBuffer>,
    visible_light_indices_buffer: Option<WebGlBuffer>,
}

impl Renderer {

    pub fn new(context: WebGl2RenderingContext) -> Self {

        let vs = String::from("#version 300 es
                    layout (location = 0) in vec3 aPos;
                    out vec4 vertexColor;

                    void main() 
                    {
                        gl_Position = vec4(aPos, 1.0);
                        vertexColor = vec4(0.0, 0.5, 0.3, 1.0);
                    }");

        let fs = String::from("#version 300 es
                    precision mediump float;
                    out vec4 FragColor;
                    in vec4 vertexColor;

                    void main() 
                    {
                        FragColor = vertexColor;
                    }");

        let mut shader = Shader::new(context.clone(), vs, fs);
        match shader.compile() {
            Err(msg) => console_log!("Shader Compilation: {}", msg),
            Ok(()) => {}
        }

        // set states for our renderer
        
       // context.enable(WebGlRenderingContext::GL_CULL_FACE);
       // context.enable(WebGlRenderingContext::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE);

        Renderer {
            context,
            shader,
            light_buffer: None,
            visible_light_indices_buffer: None
        }
    }

    pub fn intiialize(&mut self) {
        let gl = &self.context;


        let triangle_buffer = gl.create_buffer().ok_or("failed").unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&triangle_buffer));

        unsafe {
            let js_array = js_sys::Float32Array::view(&Triangles);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &js_array,
                WebGl2RenderingContext::STATIC_DRAW
            );
        }

        gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 6 * 4, 0);
        gl.enable_vertex_attrib_array(0);

        gl.vertex_attrib_pointer_with_i32(1, 3, WebGl2RenderingContext::FLOAT, false, 6 * 4, 3 * 4);
        gl.enable_vertex_attrib_array(1);

        // gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        // gl.depth_mask(true);
        //gl.enable(GL_CULL_FACE);

        // unsafe {
        //     let jsPositionArray = js_sys::Float32Array::view(&QuadPositions);
        //     gl.buffer_data_with_array_buffer_view(
        //         WebGl2RenderingContext::ARRAY_BUFFER,
        //         &jsPositionArray,
        //         WebGl2RenderingContext::STATIC_DRAW
        //     );

        //     let jsTextArray = js_sys::Float32Array::view(&QuadTexCoords);
        //     gl.buffer_data_with_array_buffer_view(
        //         WebGl2RenderingContext::ARRAY_BUFFER,
        //         &jsTextArray,
        //         WebGl2RenderingContext::STATIC_DRAW
        //     );
        // }
    }

    pub fn render(&self, time: f32) {

        // yeah!
        let gl: &WebGl2RenderingContext = &self.context;

        gl.clear_color(0.38, 0.58, 0.93, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        gl.use_program(self.shader.get_program());

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            3
        )

    }
}