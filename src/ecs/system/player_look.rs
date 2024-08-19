use sdl2::pixels::Color;
use sdl2::sys::{ldiv_t, XOC};

use crate::ecs::component::camera_data::CameraData;
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
    fn cast_rays(&mut self, player: &Transform, world_data: &WorldData, camera: &CameraData) {
        let map = &world_data.get_wall_array(); // Assuming world_data contains a 2D map array
        let _map_width = map[0].len() as i32;
        let _map_height = map.len() as i32;
    
        let pos_x = player.position.x();
        let pos_y = player.position.y();
        let dir_x = player.position.dir_x();
        let dir_y = player.position.dir_y();
        let fov = camera.fov.clone(); // Assuming player has a method to get FOV
    
        let window_height;
        let window_width;
        {
            let window_manager = self.window_manager.borrow();
            window_height = window_manager.get_window_size().0 as i32;
            window_width = window_manager.get_window_size().1 as i32;
        }
    
        // Introduce a resolution variable to increase the number of rays fired
        let resolution = 2; // Increase this value to increase the resolution
        let rays_to_cast = window_width * resolution;
        let camera_plane = camera.calculate_camera_plane(dir_x, dir_y);
    
        // Calculate the center column index
        let center_column = rays_to_cast / 2;
    
        for ray_index in 0..rays_to_cast {
            // Adjust camera_x calculation to ensure the center column corresponds to the player's forward direction
            let camera_x = (ray_index as f32 - center_column as f32) / center_column as f32; // x-coordinate in camera space
            let ray_dir_x = dir_x + camera_plane.0 * camera_x;
            let ray_dir_y = dir_y + camera_plane.1 * camera_x;
    
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
                }
            }
    
            // Calculate distance projected on camera direction (oblique distance will give fisheye effect!)
            if side == 0 {
                perp_wall_dist =
                    ((map_x as f32 - pos_x + (1.0 - step_x as f32) / 2.0) / ray_dir_x).abs();
            } else {
                perp_wall_dist =
                    ((map_y as f32 - pos_y + (1.0 - step_y as f32) / 2.0) / ray_dir_y).abs();
            }
    
            // Calculate height of line to draw on screen
            let scale_factor = 1.5; // Adjust this factor to make walls higher or lower
            let line_height = (window_height as f32 / perp_wall_dist * scale_factor) as i32;
    
            // Calculate the lowest and highest pixel to fill in the current stripe
            let draw_start = (-line_height / 2 + window_height / 2).max(0);
            let draw_end = (line_height / 2 + window_height / 2).min(window_height as i32 - 1);
    
            // Calculate the width of each slice based on the number of rays and the window width
            let slice_width = (window_width as f32 / rays_to_cast as f32).ceil() as i32;
    
            // Shading
            let max_distance = 50.0; // Increase the maximum distance for a more gradual shading effect
            let distance_factor = (1.0 - (perp_wall_dist / max_distance)).max(0.0).min(1.0);
            let angle_factor = (ray_dir_x * dir_x + ray_dir_y * dir_y).max(0.0).min(1.0);
            let shading_factor = distance_factor * angle_factor;
    
            // Define the colors for shading
            let white = (255, 255, 255);
            let darkest = (150, 150, 150); // Dark gray
    
            // Interpolate between white and dark gray based on the shading factor
            let shaded_color = (
                (white.0 as f32 * shading_factor + darkest.0 as f32 * (1.0 - shading_factor)) as u8,
                (white.1 as f32 * shading_factor + darkest.1 as f32 * (1.0 - shading_factor)) as u8,
                (white.2 as f32 * shading_factor + darkest.2 as f32 * (1.0 - shading_factor)) as u8,
            );
            // Draw the wall slice for the current ray
            for x in (ray_index * slice_width)..((ray_index + 1) * slice_width) {
                if x >= 0 && x < window_width {
                    self.draw_wall_slice_with_shading(
                        x,
                        draw_start,
                        draw_end - draw_start,
                        shaded_color,
                    );
                }
            }
        }
    }
    
    // fn draw_wall_slice(&mut self, x: i32, y: i32, height: i32) {
    //     let window_manager = self.window_manager.borrow();
    //     window_manager.draw_rect(x, y, 1, height as u32, 255, 255, 255);
    // }
    // fn draw_ray(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
    //     let window_manager = self.window_manager.borrow();
    //     window_manager.draw_line(x1, y1, x2, y2, 255, 255, 255)
    // }
    fn draw_wall_slice_with_shading(&mut self, x: i32, y: i32, height: i32, color: (u8, u8, u8)) {
        let window_manager = self.window_manager.borrow();
        window_manager.draw_rect(x, y, 1, height as u32, color.0, color.1, color.2);
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

        let camera = entity_manager
            .get_component::<CameraData>(&player_entity_id)
            .expect("Failed to get CameraData component");

        // Cast rays from the player's position
        self.cast_rays(transform, world_data, camera);
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
