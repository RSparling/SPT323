use crate::ecs::entity_manager::EntityManager;
use crate::ecs::component::world_data::WorldData;
use crate::sdl_window_manager::SDLWindowManager;

use std::cell::RefCell;
use std::rc::Rc;

pub struct TestWorld;

impl TestWorld{
    pub fn spawn(entity_manager: &mut EntityManager, window_manager: Rc<RefCell<SDLWindowManager>>) {
        let world_entity = entity_manager.create_entity();
        let window_size_x;
        let window_size_y;
        {
            let sdl_window_manager = window_manager.borrow_mut();
            let (window_width, window_height) = sdl_window_manager.get_window_size();
            window_size_x = window_width;
            window_size_y = window_height;
        }
        entity_manager.add_component(
            &world_entity,
            WorldData::new(20, window_size_x, window_size_y),
        );
    }
}