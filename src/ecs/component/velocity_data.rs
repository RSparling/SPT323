use crate::ecs::component::Component;

#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {} //implementation of component for velocity