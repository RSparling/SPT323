// src/ecs/systems/collision_system.rs
// Description:
// This module contains the collision system which is responsible for checking if entities are colliding with each other or the boundaries of the screen.

use crate::ecs::component::{
    collision_data::CollisionData, position_data::Position, render_data::RenderData,
    velocity_data::Velocity,
};
use crate::ecs::entity_manager::{Entity, EntityManager};
use crate::ecs::system::System;
use std::any::Any;

pub struct CollisionSystem;

// Define screen boundaries
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

impl System for CollisionSystem {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        // Collect the necessary data immutably first
        let mut collision_entities: Vec<(u32, f32, f32, f32)> = Vec::new();

        for entity in entity_manager.query_entities::<CollisionData>() {
            if let Some(position) = entity_manager.get_component::<Position>(entity) {
                if let Some(render_data) = entity_manager.get_component::<RenderData>(entity) {
                    collision_entities.push((entity.id, position.x, position.y, render_data.size));
                }
            }
        }

        // Perform the mutable updates in a separate loop
        for (entity_id, pos_x, pos_y, size) in collision_entities {
            if let Some(velocity) = entity_manager.get_component_mut::<Velocity>(&Entity { id: entity_id }) {
                // Check collision with screen boundaries and adjust velocity accordingly
                if pos_x < 0.0 {
                    velocity.x = velocity.x.abs(); // Move right
                }
                if pos_x + size > SCREEN_WIDTH {
                    velocity.x = -velocity.x.abs(); // Move left
                }
                if pos_y < 0.0 {
                    velocity.y = velocity.y.abs(); // Move down
                }
                if pos_y + size > SCREEN_HEIGHT {
                    velocity.y = -velocity.y.abs(); // Move up
                }
            }
        }
    }

    fn priority(&self) -> u32 {
        2
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
