/// src/ecs/system/world_system.rs
/// Description:
/// This module contains the world system which is responsible for rendering the walls in the world.
/// WARNING: This code is obselotete unless a top-down view is needed.
use crate::ecs::component::world_data::{Wall, WorldData};
use crate::ecs::entity_manager::EntityManager;
use crate::sdl_window_manager::SDLWindowManager;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::System;

pub struct WorldSystem {
    pub window_manager: Rc<RefCell<SDLWindowManager>>,
}

impl WorldSystem {
    /// Draws all walls for the world
    fn draw_walls(&self, walls: &[Wall], cell_size: u32) {
        print!("Drawing walls..."); // Debug print
        let sdl_window_manager = self.window_manager.borrow_mut();

        for wall in walls {

            sdl_window_manager.draw_filled_rect(
                (wall.x * cell_size) as i32,
                (wall.y * cell_size) as i32,
                cell_size,
                cell_size,
                0,
                0,
                255,
            );

            // Outline via non-filled rect
            sdl_window_manager.draw_rect(
                (wall.x * cell_size) as i32,
                (wall.y * cell_size) as i32,
                cell_size,
                cell_size,
                0,
                0,
                0,
            );
        }
    }
}

impl System for WorldSystem {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        let world_entities = entity_manager.query_entities::<WorldData>();

        // Ensure there's exactly one world entity
        match world_entities.len() {
            0 => panic!("There must be a world data component"),
            1 => (),
            _ => panic!("There can only be one world data component"),
        }

        // Get the world data
        let world_data = entity_manager
            .get_component::<WorldData>(&world_entities[0])
            .expect("Failed to get WorldData component");

        // Get all the walls to be rendered
        let walls = world_data.get_all_walls();
        let cell_size = world_data.get_cell_size();

        // Delegate the wall drawing to a separate method
        self.draw_walls(&walls, cell_size);
    }

    fn priority(&self) -> u32 {
        110
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
