//ecs::system::MovementSystem.rs
//Description:
//This module contains the movement system which is responsible for updating the position of entities based on their velocity.
use crate::ecs::entity_manager::{EntityManager, Entity}; //use entity manager and entity

use crate::ecs::system::System; //use system trait
use crate::ecs::component::{position_data, velocity_data}; //use Position and Velocity components

pub struct MovementSystem; //struct for movement system

impl System for MovementSystem {
    fn update(&mut self, entity_manager: &mut EntityManager) {
        // Collect all entities with both Position and Velocity components
        let entities_to_update: Vec<(u32, velocity_data::Velocity)> = entity_manager
            .query_entities::<velocity_data::Velocity>() // query entities with Velocity
            .iter()
            .filter_map(|entity| {
                let entity_id = entity.id;
                if let Some(velocity) = entity_manager.get_component::<velocity_data::Velocity>(entity) {
                    if entity_manager.get_component::<position_data::Position>(entity).is_some() {
                        return Some((entity_id, velocity.clone()));
                    }
                }
                None
            })
            .collect();

        // Update positions based on velocities
        for (entity_id, velocity) in entities_to_update {
            if let Some(position) = entity_manager.get_component_mut::<position_data::Position>(&Entity { id: entity_id }) {
                position.x += velocity.x;
                position.y += velocity.y;
            }
        }
    }
}
