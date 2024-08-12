use crate::ecs::component::Component;

#[derive(Clone)]
pub struct Transform{//holds structs for position and velocity
    pub position: Position,
    pub velocity: Velocity,
}

#[derive(Clone)]
pub struct Position {
    pub pos_x: f32,
    pub pos_y: f32,
    pub rotation: f32,
}

#[derive(Clone)]
pub struct Velocity {
    pub delta_x: f32,
    pub delta_y: f32,
}

impl Component for Transform {} //implementation of component for transform