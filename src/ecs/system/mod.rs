//ecs::systems::mod.rs
//Description:
//This module contains systems which are responsible for updating entities and acting on their components. These modules should contain no data unless that data is
//shared between all entities.

use crate::ecs::entity_manager::EntityManager; //use entity manager

pub mod movement_system; //module for movement system
pub mod render_system; //module for render system
pub mod player_controller; //module for player system
pub mod collision_system; //module for collision system


pub trait System {
    fn update(&mut self, entity_manager: &mut EntityManager); // function to update
}
