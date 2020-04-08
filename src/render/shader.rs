
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};

use super::super::utils;

pub struct Shader {

    program: Option<WebGlProgram>,

    vertex_source: String,
    fragment_source: String,

    context: WebGl2RenderingContext
    
}

impl Shader {

    pub fn new(context: WebGl2RenderingContext,
               vertex_source: String, 
               fragment_source: String) -> Shader {

        Shader {
            vertex_source,
            fragment_source,
            context,

            program: None
        }

    }

    pub fn compile(&mut self) -> Result<(), String> {

        let vs: &str = &self.vertex_source;
        let vertex_shader 
                = compile_with_type(&self.context, WebGl2RenderingContext::VERTEX_SHADER, vs);
                
        match vertex_shader {
            Ok(_) => {},
            Err(msg) => return Err(msg)
        }

        let fs: &str = &self.fragment_source;
        let fragment_shader
                = compile_with_type(&self.context, WebGl2RenderingContext::FRAGMENT_SHADER, fs);
        match fragment_shader {
            Ok(_) => {},
            Err(msg) => return Err(msg)
        }

        let program 
            = link_program(&self.context, &vertex_shader.unwrap(), &fragment_shader.unwrap())?;

        self.program = Some(program);

        return Ok(());
    }

    pub fn get_program(&self) -> Option<&WebGlProgram> {
        self.program.as_ref()
    }
}

fn compile_with_type(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {

    let shader = context
                     .create_shader(shader_type)
                     .ok_or_else(|| String::from("Unable to create shader!"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
       .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
       .as_bool()
       .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader!"))
        )
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader
) -> Result<WebGlProgram, String> {

    let program = context
                    .create_program()
                    .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        
        Ok(program)   

    } else {

        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unkown error creating program shader")))

    }

}