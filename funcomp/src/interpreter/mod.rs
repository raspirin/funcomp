use crate::interpreter::environment::Environment;

pub mod environment;
pub mod visit;

pub struct State {
    rot: f32,
    origin: f32,
}

pub struct Interpreter{
    pub environment: Environment,
}
