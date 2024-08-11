//ecs::system::PlayerController.rs
//Description:
//This module contains the player controller system which is responsible for updating the player's velocity based on input.
use crate::InputHandler; //use input handler
use crate::ecs::entity_manager::{EntityManager, Entity}; //use entity manager and entity

use crate::ecs::component::{player_data, velocity_data}; //use PlayerData and Velocity components

pub struct PlayerController {}

impl PlayerController  {//to be placed on player entity, sets the player's velocity based on input. If no input, sets velocity to 0.
    pub fn update(&mut self, entity_manager: &mut EntityManager, input_handler: &InputHandler) {
        //get entities with player data
        let player_entities: Vec<_> = entity_manager
            .query_entities::<player_data::PlayerData>()
            .iter()
            .map(|entity| entity.id)
            .collect();

            //if player_entities is empty then print "no player entities"
        if player_entities.is_empty() {
            println!("No player entities");
        }
        //for each player entity, set velocity based on input
        //if no input, set velocity to 0
        for entity_id in player_entities {
            if let Some(velocity) = entity_manager.get_component_mut::<velocity_data::Velocity>(&Entity { id: entity_id }) {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
            else{
                println!("No velocity component for player entity");
            }
            //check input
            if let Some(velocity) = entity_manager.get_component_mut::<velocity_data::Velocity>(&Entity { id: entity_id }) {
                if input_handler.is_w_pressed() {
                    velocity.y -= 1.0;
                }
                if input_handler.is_s_pressed() {
                    velocity.y += 1.0;
                }
                if input_handler.is_a_pressed() {
                    velocity.x -= 1.0;
                }
                if input_handler.is_d_pressed() {
                    velocity.x += 1.0;
                }
            }
            else{
                println!("No velocity component for player entity");
            }
        }
    }
}