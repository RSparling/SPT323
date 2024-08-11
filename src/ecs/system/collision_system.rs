//ecs::systems::CollisionSystem.rs
//Description:
//This module contains the collision system which is responsible for checking if entities are colliding with each other or the boundaries of the screen.

use crate::ecs::system::System;
use crate::ecs::entity_manager::{EntityManager, Entity};
use crate::ecs::component::{collision_data::CollisionData, position_data::Position, render_data::RenderData, velocity_data::Velocity};
pub struct CollisionSystem;

impl System for CollisionSystem {
    fn update(&mut self, entity_manager: &mut EntityManager) {
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
                if pos_x < 0.0 {
                    velocity.x = 1.0;
                }
                if pos_x + size > 800.0 {
                    velocity.x = -1.0;
                }
                if pos_y < 0.0 {
                    velocity.y = 1.0;
                }
                if pos_y + size > 600.0 {
                    velocity.y = -1.0;
                }
            }
        }
    }
}
