// src/ecs/system/movement_system.rs
// Description:
// This module contains the movement system, responsible for updating the position of entities based on their velocity.

use crate::ecs::entity_manager::{EntityManager, Entity};
use crate::ecs::system::System;
use crate::ecs::component::transform_data::Transform;
use std::any::Any;

pub struct MovementSystem;

impl MovementSystem {
    /// Updates the position of a single entity based on its velocity
    fn update_entity_position(transform: &mut Transform) {
        // Calculate the forward vector
        let forward = transform.position.forward_vector();
        //calulate the right vector
        let right = transform.position.right_vector();
        //now calculate the direction it should move relative to looking position and how far it should move using velocity.
        let delta_x = forward.0 * transform.velocity.delta_x + right.0 * transform.velocity.delta_y * (1.0/60.0);
        let delta_y = forward.1 * transform.velocity.delta_x + right.1 * transform.velocity.delta_y * (1.0/60.0);
        // Update the position
        transform.position.modify_position(delta_x, delta_y);
    }
}

impl System for MovementSystem {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        // Collect entity IDs with Transform components first
        let entities_to_update: Vec<Entity> = entity_manager
            .query_entities::<Transform>()
            .iter()
            .map(|entity| Entity { id: entity.id })
            .collect();

        // Update positions based on velocities
        for entity in entities_to_update {
            if let Some(transform) = entity_manager.get_component_mut::<Transform>(&entity) {
                MovementSystem::update_entity_position(transform);
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
