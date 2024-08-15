use crate::ecs::component::Component;

#[derive(Clone, Default)]
pub struct Transform {
    pub position: Position,
    pub velocity: Velocity,
}

#[derive(Clone, Default)]
pub struct Position {
    x: f32,
    y: f32,
    rotation: f32, // rotation in degrees, should be between 0 and 360
}

#[derive(Clone, Default)]
pub struct Velocity {
    pub delta_x: f32,
    pub delta_y: f32,
}

impl Component for Transform {}

impl Position {
    pub fn new(x: f32, y: f32, rotation: f32) -> Self {
        Position {
            x,
            y,
            rotation,
        }
    }
    /// Normalize the rotation to ensure it remains between 0 and 360 degrees
    pub fn normalize_rotation(&mut self) {
        if self.rotation < 0.0{
            self.rotation += 360.0;
        }
        if self.rotation >= 360.0{
            self.rotation -= 360.0;
        }
    }

    /// Update rotation by a given delta, normalizing afterwards
    pub fn update_rotation(&mut self, delta: f32) {
        self.rotation += delta;
        self.normalize_rotation();
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.normalize_rotation();
    }

    pub fn set_coords(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl Velocity {

    /// Set velocity directly
    pub fn set_direct(&mut self, delta_x: f32, delta_y: f32) {
        self.delta_x = delta_x;
        self.delta_y = delta_y;
    }
}
