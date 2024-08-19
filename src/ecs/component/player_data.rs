use crate::ecs::component::Component;

#[derive(Clone)]
pub struct PlayerData{
    fov: i32,
}

impl Component for PlayerData{}

impl PlayerData{
    pub fn new() -> PlayerData{
        PlayerData{
            fov: 60,
        }
    }
    pub fn get_fov(&self) -> i32{
        self.fov
    }
}
