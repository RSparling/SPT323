use crate::ecs::component::Component;

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {} //implementation of component for position
