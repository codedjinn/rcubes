
pub trait Updateable {
    fn update(time: f32);
}

pub trait Initializer {
    fn initialize();
}