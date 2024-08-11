use crate::ecs::component::Component;

#[derive(Clone)]
pub struct RenderData {
    pub size: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Component for RenderData {} //implementation of component for render data
