// src/ecs/system/player_look.rs
// Description:
// This module contains the player look system, responsible for casting rays to detect walls in the world.

use crate::ecs::component::player_data::PlayerData;
use crate::ecs::component::transform_data::Transform;
use crate::ecs::component::world_data::WorldData;
use crate::ecs::entity_manager::EntityManager;
use crate::sdl_window_manager::SDLWindowManager;
use std::any::Any;
use std::cell::RefCell;
use std::f32::INFINITY;
use std::rc::Rc;

use super::System;

pub struct PlayerLook {
    pub window_manager: Rc<RefCell<SDLWindowManager>>,
}

impl PlayerLook {
    /// Casts a ray from the player's position and returns the hit position if a wall is detected.
    fn cast_rays(
        &mut self,
        player: &Transform,
        world_data: &WorldData,
    ) {
        let mut dist_horz: f32 = INFINITY;
        let mut dist_vert: f32 = INFINITY;

        let mut hit_horizontal: (f32, f32) = (0.0, 0.0);
        let mut hit_vertical: (f32, f32) = (0.0, 0.0);
        // Variables
        let mut ray_angle: f32 = player.position.rotation();  // Now in degrees
        let mut ray_x: f32;
        let mut ray_y: f32;
        let mut x_offset: f32;
        let mut y_offset: f32;
        let mut map_x: i32;
        let mut map_y: i32;
        let mut dof: i32 = 0; // Depth of field, the maximum distance the ray can travel measured in number of tiles
        let max_dof: i32 = world_data.world_size as i32; // Max depth of field is set to the world size

        // The loop for each ray
        for _ in 0..1 {
            // Horizontal
            dof = 0;
            let aTan: f32 = -1.0 / ray_angle.to_radians().tan(); // Convert angle to radians for trigonometric functions

            if ray_angle > 180.0 { // Ray is pointing up
                ray_y = (player.position.y() / world_data.cell_size as f32).floor() * world_data.cell_size as f32 - 0.0001;
                ray_x = (player.position.y() - ray_y) * aTan + player.position.x();
                y_offset = -(world_data.cell_size as f32);
                x_offset = -y_offset * aTan;
            } else if ray_angle < 180.0 { // Ray is pointing down
                ray_y = (player.position.y() / world_data.cell_size as f32).floor() * world_data.cell_size as f32 + world_data.cell_size as f32;
                ray_x = (player.position.y() - ray_y) * aTan + player.position.x();
                y_offset = world_data.cell_size as f32;
                x_offset = -y_offset * aTan;
            } else { // Ray is pointing left or right
                ray_x = player.position.x();
                ray_y = player.position.y();
                dof = world_data.world_size as i32;; // Set depth of field to max as the ray will never intersect
                x_offset = world_data.cell_size as f32;
                y_offset = 0.0;
                println!("Warning: Ray never intersects with horizontal grid lines");
            }
            while dof <world_data.world_size as i32 {
                map_x = (ray_x / world_data.cell_size as f32) as i32;
                map_y = (ray_y / world_data.cell_size as f32) as i32;

                if map_x < 0 || map_y < 0 || map_x >= world_data.world_size as i32 || map_y >= world_data.world_size as i32 {
                    break; // Out of bounds
                }

                if world_data.is_wall(map_x, map_y) {
                    dof = max_dof; // Stop when a wall is hit
                    hit_horizontal = (ray_x, ray_y);
                } else { // If it's not a wall, keep going
                    ray_x += x_offset;
                    ray_y += y_offset;
                    dof += 1;
                }
            }
            // vertical line check
            dof = 0;
            let n_tan: f32 = -ray_angle.to_radians().tan(); // Convert angle to radians for trigonometric functions
            //looking left
            if ray_angle > 90.0 && ray_angle < 270.0 {
                ray_x = (player.position.x() / world_data.cell_size as f32).floor() * world_data.cell_size as f32 - 0.0001;
                ray_y = (player.position.x() - ray_x) * n_tan + player.position.y();
                x_offset = -(world_data.cell_size as f32);
                y_offset = -x_offset * n_tan;
            } else if ray_angle < 90.0 || ray_angle > 270.0 { //looking right
                ray_x = (player.position.x() / world_data.cell_size as f32).floor() * world_data.cell_size as f32 + world_data.cell_size as f32;
                ray_y = (player.position.x() - ray_x) * n_tan + player.position.y();
                x_offset = world_data.cell_size as f32;
                y_offset = -x_offset * n_tan;
            } else { // Ray is pointing up or down
                ray_x = player.position.x();
                ray_y = player.position.y();
                dof = world_data.world_size as i32; // Set depth of field to max as the ray will never intersect
                x_offset = world_data.cell_size as f32;
                y_offset = 0.0;
                println!("Warning: Ray never intersects with horizontal grid lines");
            }
            while dof < world_data.world_size as i32 {
                map_x = (ray_x / world_data.cell_size as f32) as i32;
                map_y = (ray_y / world_data.cell_size as f32) as i32;

                if map_x < 0 || map_y < 0 || map_x >= world_data.world_size as i32 || map_y >= world_data.world_size as i32 {
                    break; // Out of bounds
                }

                if world_data.is_wall(map_x, map_y) {
                    dof = max_dof; // Stop when a wall is hit
                    hit_vertical = (ray_x, ray_y);
                } else { // If it's not a wall, keep going
                    ray_x += x_offset;
                    ray_y += y_offset;
                    dof += 1;
                }
            }
            // Calculate the distance to the horizontal and vertical walls use the power of basic geometry
            dist_horz = ((player.position.x() - hit_horizontal.0).powi(2) + (player.position.y() - hit_horizontal.1).powi(2)).sqrt();
            dist_vert = ((player.position.x() - hit_vertical.0).powi(2) + (player.position.y() - hit_vertical.1).powi(2)).sqrt();

            //draw the shortest ray
            if dist_horz < dist_vert {
                self.draw_ray((player.position.x(), player.position.y()), hit_horizontal);
            } else {
                self.draw_ray((player.position.x(), player.position.y()), hit_vertical);
            }
        }
    }

    /// Draws the ray from the player's position to the hit position or maximum length.
    fn draw_ray(
        &mut self,
        player_pos: (f32, f32),
        hit_pos: (f32, f32),
    ) {
        let mut window_manager = self.window_manager.borrow_mut();
        window_manager.draw_line(
            player_pos.0 as i32,
            player_pos.1 as i32,
            hit_pos.0 as i32,
            hit_pos.1 as i32,
            255,
            255,
            255,
        );
    }
}

impl System for PlayerLook {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        let player_entities = entity_manager.query_entities::<PlayerData>();

        // Ensure there's exactly one player entity
        if player_entities.len() != 1 {
            panic!("There must be exactly one player entity with PlayerData.");
        }

        let player_entity_id = player_entities[0];

        // Get the player's transform
        let transform = entity_manager
            .get_component::<Transform>(&player_entity_id)
            .expect("Failed to get Transform component");

        // Get the world data
        let world_entities = entity_manager.query_entities::<WorldData>();
        if world_entities.len() != 1 {
            panic!("There must be exactly one world entity with WorldData.");
        }

        let world_data = entity_manager
            .get_component::<WorldData>(&world_entities[0])
            .expect("Failed to get WorldData component");

        // Cast a ray from the player's position
        self.cast_rays(transform, world_data);
    }

    fn priority(&self) -> u32 {
        130
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
