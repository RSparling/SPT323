use sdl2::pixels::Color;
use sdl2::sys::{ldiv_t, XOC};

use crate::ecs::component::player_data::PlayerData;
use crate::ecs::component::transform_data::Transform;
use crate::ecs::component::world_data::WorldData;
use crate::ecs::entity_manager::EntityManager;
use crate::sdl_window_manager::SDLWindowManager;
use std::any::Any;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::f32::INFINITY;
use std::rc::Rc;

use super::System;

pub struct PlayerLook {
    pub window_manager: Rc<RefCell<SDLWindowManager>>,
}

impl PlayerLook {
    fn cast_rays(
        &mut self,
        player: &Transform,
        world_data: &WorldData,
        player_config: &PlayerData,
    ) {
        let map = &world_data.get_wall_array(); // Assuming world_data contains a 2D map array
        let map_width = map[0].len() as i32;
        let map_height = map.len() as i32;

        let pos_x = player.position.x();
        let pos_y = player.position.y();
        let dir_x = player.position.dir_x();
        let dir_y = player.position.dir_y();
        let fov = player_config.get_fov(); // Assuming player has a method to get FOV

        for angle in 0..fov {
            let ray_angle = (angle as f32 - (fov as f32 / 2.0)).to_radians();
            let ray_dir_x = dir_x * ray_angle.cos() - dir_y * ray_angle.sin();
            let ray_dir_y = dir_x * ray_angle.sin() + dir_y * ray_angle.cos();

            // Calculate the ray's position in the map
            let mut map_x = pos_x as i32;
            let mut map_y = pos_y as i32;

            // Length of ray from current position to next x or y-side
            let mut side_dist_x;
            let mut side_dist_y;

            // Length of ray from one x or y-side to next x or y-side
            let delta_dist_x = (1.0 / ray_dir_x).abs();
            let delta_dist_y = (1.0 / ray_dir_y).abs();
            let mut perp_wall_dist = 0.0;

            // Direction to step in x and y (+1 or -1)
            let step_x;
            let step_y;

            let mut hit = false; // Was there a wall hit?
            let mut side = 0; // Was a NS or a EW wall hit?

            // Calculate step and initial sideDist
            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (pos_x - map_x as f32) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f32 + 1.0 - pos_x) * delta_dist_x;
            }
            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (pos_y - map_y as f32) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f32 + 1.0 - pos_y) * delta_dist_y;
            }

            // Perform DDA
            while !hit {
                // Jump to next map square, OR in x-direction, OR in y-direction
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }
                // Check if ray has hit a wall
                let map_coords: (i32, i32) =
                    world_data.get_map_coord_from_world_pos(map_x as f32, map_y as f32);
                if world_data.is_wall(map_coords.0, map_coords.1) {
                    hit = true;
                    self.draw_ray(
                        player.position.x() as i32,
                        player.position.y() as i32,
                        map_x,
                        map_y,
                    );
                }
            }
        }
    }

    fn draw_ray(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let window_manager = self.window_manager.borrow();
        window_manager.draw_line(x1, y1, x2, y2, 255, 255, 255)
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

        let player_config = entity_manager
            .get_component::<PlayerData>(&player_entity_id)
            .expect("Failed to get PlayerData component");

        // Cast rays from the player's position
        self.cast_rays(transform, world_data, player_config);
    }

    fn priority(&self) -> u32 {
        150
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
