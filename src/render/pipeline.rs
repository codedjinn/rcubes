
use super::Shader;

struct ForwardRenderer {
    shader: Option<Shader>
}

impl ForwardRenderer {

    pub fn new() -> Self {
        ForwardRenderer {
            shader: None
        }
    }

    pub fn init(&self) {

        let vs = String::from("");

    }

    pub fn render(&self, time: f32) {

    }

}