// src/prefabs/player.rs
// Description: This module demonstrates how a prefab can be created using the player entity.
// It creates an entity with the the necessary components and then subscribes them to the desired systems.
use crate::ecs::entity_manager::EntityManager;
use crate::ecs::entity_manager::Entity;
//components
use crate::ecs::component::transform_data::Position;
use crate::ecs::component::{
    player_data::PlayerData, render_data::RenderData, transform_data, camera_data,
};
use crate::ecs::system::{
    movement_system::MovementSystem, player_controller::PlayerController, camera_system::Camera_System,
};

pub struct Player;

impl Player {
    pub fn spawn(entity_manager: &mut EntityManager) -> Entity {
        let player_entity: Entity = entity_manager.create_entity();
        entity_manager.add_component(
            &player_entity,
            RenderData {
                r: 0.99,
                g: 0.99,
                b: 0.5,
                size: 10.0,
            },
        );
        entity_manager.add_component(
            &player_entity,
            transform_data::Transform {
                velocity: transform_data::Velocity {
                    delta_x: 0.0,
                    delta_y: 0.0,
                },
                position: Position::new(100.0, 100.0, 0.0),
            },
        );
        entity_manager.add_component(&player_entity, PlayerData::new());
        entity_manager.add_component(&player_entity, camera_data::CameraData::new());

        entity_manager.register_entity_to_system::<MovementSystem>(&player_entity);
        entity_manager.register_entity_to_system::<PlayerController>(&player_entity);
        entity_manager.register_entity_to_system::<Camera_System>(&player_entity);

        return player_entity;
    }
}
