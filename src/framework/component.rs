
//
// for now keeping it simple, will have a list of all defined components
// so that they can be easily linked

pub trait Component {
    fn get_type() -> u32;
}