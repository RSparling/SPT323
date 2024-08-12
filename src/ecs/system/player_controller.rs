// src/ecs/system/player_controller.rs
// Description:
// This module contains the player controller system which is responsible for updating the player's velocity based on input.

use std::rc::Rc;
use std::cell::RefCell;

use crate::{ecs::entity_manager::{EntityManager, Entity}, input_handler::InputHandler};
use crate::ecs::system::System;
use crate::ecs::component::{transform_data::Transform, player_data::PlayerData};

use std::any::Any;

pub struct PlayerController {
    pub input_handler: Rc<RefCell<InputHandler>>, // Expecting Rc<RefCell<InputHandler>>
}

impl System for PlayerController {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        let player_entities: Vec<_> = entity_manager
            .query_entities::<PlayerData>()
            .iter()
            .map(|entity| entity.id)
            .collect();

        if player_entities.is_empty() {
            println!("No player entities");
        }

        for entity_id in player_entities {
            if let Some(transform) = entity_manager.get_component_mut::<Transform>(&Entity { id: entity_id }) {
                transform.velocity.delta_x = 0.0;
                transform.velocity.delta_y = 0.0;

                let input_handler = self.input_handler.borrow();

                if input_handler.is_w_pressed() {
                    transform.velocity.delta_y -= 3.0;
                }
                if input_handler.is_s_pressed() {
                    transform.velocity.delta_y += 3.0;
                }
                if input_handler.is_a_pressed() {
                    transform.velocity.delta_x -= 3.0;
                }
                if input_handler.is_d_pressed() {
                    transform.velocity.delta_x += 3.0;
                }
            } else {
                println!("No velocity component for player entity");
            }
        }
    }

    fn priority(&self) -> u32 {
        3
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
