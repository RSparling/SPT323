// src/ecs/system/movement_system.rs
// Description:
// This module contains the movement system which is responsible for updating the position of entities based on their velocity.
use crate::ecs::entity_manager::{EntityManager, Entity};
use crate::ecs::system::System;
use crate::ecs::component::transform_data::{self, Transform};
use std::any::Any;

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        // Collect all entities with both Position and Velocity components
        let entities_to_update: Vec<(u32, transform_data::Transform)> = entity_manager
            .query_entities::<transform_data::Transform>()
            .iter()
            .filter_map(|entity| {
                let entity_id = entity.id;
                if let Some(velocity) = entity_manager.get_component::<transform_data::Transform>(entity) {
                    if entity_manager.get_component::<transform_data::Transform>(entity).is_some() {
                        return Some((entity_id, velocity.clone()));
                    }
                }
                None
            })
            .collect();

        // Update positions based on velocities
        for (entity_id, Transform) in entities_to_update {
            if let Some(transform) = entity_manager.get_component_mut::<transform_data::Transform>(&Entity { id: entity_id }) {
                transform.position.pos_x += transform.velocity.delta_x * (1.0/8.0);
                transform.position.pos_y += transform.velocity.delta_y * (1.0/8.0);
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
