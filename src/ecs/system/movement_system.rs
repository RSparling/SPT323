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
        // Calculate the movement direction based on rotation
        let rotation = transform.position.rotation();
        let move_x = transform.velocity.delta_x * rotation.cos();
        let move_y = transform.velocity.delta_x * rotation.sin();

        // Update position based on movement direction
        transform.position.set_x(transform.position.x() + move_x);
        transform.position.set_y(transform.position.y() + move_y);
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
