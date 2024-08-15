// src/ecs/system/player_controller.rs

use std::rc::Rc;
use std::cell::RefCell;

use crate::{
    ecs::entity_manager::{EntityManager, Entity},
    input_handler::InputHandler,
    ecs::system::System,
    ecs::component::{transform_data::Transform, player_data::PlayerData},
};

use std::any::Any;

pub struct PlayerController {
    pub input_handler: Rc<RefCell<InputHandler>>, // Expecting Rc<RefCell<InputHandler>>
}

impl PlayerController {
    /// Performs the actual task of updating the player's transform based on input
    fn update_player_transform(&self, transform: &mut Transform) {
        transform.velocity.set_direct(0.0, 0.0); // Reset velocity

        let input_handler = self.input_handler.borrow();

        if input_handler.is_w_pressed() {
            transform.velocity.delta_y -= 10.0;
        }
        if input_handler.is_s_pressed() {
            transform.velocity.delta_y += 10.0;
        }
        if input_handler.is_a_pressed() {
            transform.velocity.delta_x -= 10.0;
        }
        if input_handler.is_d_pressed() {
            transform.velocity.delta_x += 10.0;
        }
        if input_handler.is_q_pressed() { // Rotate counterclockwise
            transform.position.update_rotation(-5.0);
        }
        if input_handler.is_e_pressed() { // Rotate clockwise
            transform.position.update_rotation(5.0);
        }
    }
}

impl System for PlayerController {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        let player_entities: Vec<_> = entity_manager
            .query_entities::<PlayerData>()
            .into_iter()
            .map(|entity| entity.id)
            .collect();

        if player_entities.is_empty() {
            println!("No player entities");
            return;
        }

        for entity_id in player_entities {
            if let Some(transform) = entity_manager.get_component_mut::<Transform>(&Entity { id: entity_id }) {
                // Delegate the update task to the struct implementation
                self.update_player_transform(transform);
            } else {
                println!("No transform component for player entity");
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
