// ecs::systems::mod.rs
// Description:
// This module contains systems responsible for updating entities and acting on their components. These modules should contain no data unless that data is shared between all entities.

pub mod system_manager;       // Module for system manager
pub mod movement_system;       // Module for movement system
pub mod render_system;         // Module for render system
pub mod player_controller;     // Module for player controller
pub mod collision_system;      // Module for collision system
pub mod world_system;          // Module for world system
pub mod player_look;           // Module for player look system
use std::any::Any;
use crate::ecs::entity_manager::EntityManager;

#[allow(dead_code)]
pub trait SystemBase {
    fn update(&mut self, entity_manager: &mut EntityManager, entity_id: u32);
    fn priority(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Implementation of SystemBase for any type that implements the System trait.
impl<T: System + 'static> SystemBase for T {
    fn update(&mut self, entity_manager: &mut EntityManager, entity_id: u32) {
        T::update(self, entity_manager, entity_id);
    }

    fn priority(&self) -> u32 {
        T::priority(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// The original System trait.
pub trait System: Any {
    fn update(&mut self, entity_manager: &mut EntityManager, entity_id: u32);
    fn priority(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
