
use crate::{log, console_log};

use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

// imports
mod shader;
pub use shader::*;

trait SubRenderer {
    fn render(&self, time: f32);
}

pub struct Renderer {
    shader: Shader,
    context: WebGlRenderingContext
}

impl Renderer {

    pub fn new(context: WebGlRenderingContext) -> Self {

        let vs = String::from("#version 300 es
                in vec3 a_position;
                
                uniform float uPointSize;

                void main(void){
                    gl_PointSize = uPointSize;
                    gl_Position = vec4(a_position, 1.0);
                }
        ");

        let fs = String::from("#version 300 es
            precision mediump float;

            out vec4 finalColor;
            
            void main(void) {
                finalColor = vec4(0.0, 0.0, 0.0, 1.0);
            }
        ");

        let mut shader = Shader::new(context.clone(), vs, fs);
        match shader.compile() {
            Err(msg) => console_log!("Shader Compilation: {}", msg),
            Ok(()) => {}
        }

        Renderer {
            context,
            shader
        }
    }

    pub fn render(&self, time: f32) {

        // yeah!
        let gl = &self.context;

        let vertices: [f32: 9] {
            -0.5, 0.5, 0.0, 
            -0.5, -0.5, 0.0, 
            0.5, -0.5, 0.0, 
        };

        let vertex_buffer = gl.createBuffer().ok_or("BOOM!")?;

        gl.bindBuffer(gl.ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices,
            WebGlRenderingContext::STATIC_DRAW
        );        
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

        self.gl.use_program(Some(self.shader.get_program()));
        gl.uniform1()
    }

}