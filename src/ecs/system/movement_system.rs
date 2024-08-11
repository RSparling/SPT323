// src/ecs/system/movement_system.rs
// Description:
// This module contains the movement system which is responsible for updating the position of entities based on their velocity.
use crate::ecs::entity_manager::{EntityManager, Entity};
use crate::ecs::system::System;
use crate::ecs::component::{position_data, velocity_data};
use std::any::Any;

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        // Collect all entities with both Position and Velocity components
        let entities_to_update: Vec<(u32, velocity_data::Velocity)> = entity_manager
            .query_entities::<velocity_data::Velocity>()
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
                position.x += velocity.x * (1.0/8.0);
                position.y += velocity.y * (1.0/8.0);
            }
        }
    }

    fn priority(&self) -> u32 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
