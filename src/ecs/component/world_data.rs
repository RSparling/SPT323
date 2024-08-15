// src/ecs/component/world_data.rs
// Description: This module contains the WorldData component, which stores information about the world.

use super::Component;

#[derive(Clone)]
pub struct WorldData {
    pub world_size: u32,    // Width and height of the world (in tiles)
    pub cell_size: u32,     // Size of each square tile
    pub walls: Vec<Vec<bool>>, // 2D vector to represent walls
}

pub struct Wall {
    pub x: u32,
    pub y: u32,
}

impl Component for WorldData {}

impl WorldData {
    pub fn new(tiles: u32, window_width: u32, window_height: u32) -> WorldData {
        let min_dimension = window_width.min(window_height);
        let cell_size = min_dimension / tiles;
        let mut walls = vec![vec![false; tiles as usize]; tiles as usize]; // Creates a square grid
        
        // Draw wall along the top and bottom
        for x in 0..tiles {
            walls[0][x as usize] = true;
            walls[tiles as usize - 1][x as usize] = true;
        }
    
        // Draw wall along the left and right
        for y in 0..tiles {
            walls[y as usize][0] = true;
            walls[y as usize][tiles as usize - 1] = true;
        }
    
        // Draw one in the middle
        walls[tiles as usize / 2][tiles as usize / 2] = true;
    
        WorldData {
            world_size: tiles,
            cell_size,
            walls,
        }
    }

    // Returns a reference to walls
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        // Return true if x or y is out of bounds
        if x < 0 || y < 0 || x >= self.world_size as i32 || y >= self.world_size as i32 {
            return true;
        }

        self.walls[y as usize][x as usize]
    }

    // Returns the size of each tile
    pub fn get_tile_size(&self) -> u32 {
        self.cell_size
    }

    // Returns all the walls as a vector of Wall structs
    pub fn get_all_walls(&self) -> Vec<Wall> {
        let mut walls = Vec::new();
        for y in 0..self.world_size {
            for x in 0..self.world_size {
                if self.walls[y as usize][x as usize] {
                    walls.push(Wall { x, y });
                }
            }
        }
        walls
    }
}
