use std::collections::HashSet;
use crate::prelude::*;

pub const GRID_SIZE: i32 = 16;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Default, Resource, Clone)]
pub struct BlockedAreas {
    pub blocked_locations: HashSet<GridCoords>,
    pub level_width: i32,
    pub level_height: i32,
}

impl BlockedAreas {
    pub fn in_blocked_coords(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.blocked_locations.contains(grid_coords)
    }
}

#[derive(Resource, Default)]
pub struct  MyWorldCoords(pub Vec2);

impl From<&MyWorldCoords> for GridCoords{
    fn from(world_coords: &MyWorldCoords) -> Self {
        GridCoords::new(world_coords.0.x.round() as i32 / GRID_SIZE, world_coords.0.y.round() as i32 / GRID_SIZE)
    }
}
pub trait Distance {
    fn distance(&self, grid_coords: &GridCoords) -> usize;
}
impl Distance for GridCoords {
    fn distance(&self, other: &GridCoords) -> usize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as usize
    }
}

pub trait CoordAround {
    fn closest_around(&self, grid_coords: &GridCoords, blocked_coords: &BlockedAreas) -> GridCoords;
}

impl CoordAround for GridCoords {
    fn closest_around(&self, grid_coords: &GridCoords, blocked_coords: &BlockedAreas) -> GridCoords {
        
        let (x, y) = (self.x, self.y);
        let mut successors: Vec<(usize, GridCoords)> = Vec::new();
        
        if let Some(left) = x.checked_sub(1) {
            let location = GridCoords::new(left, y);
            if !blocked_coords.in_blocked_coords(&location) {
                successors.push((location.distance(grid_coords), location));
            }
        }
        if let Some(down) = y.checked_sub(1) {
            let location = GridCoords::new(x, down);
            if !blocked_coords.in_blocked_coords(&location) {
                successors.push((location.distance(grid_coords), location));
            }
        }
        
        let right = x + 1;
        let location = GridCoords::new(right, y);
        if !blocked_coords.in_blocked_coords(&location) {
            successors.push((location.distance(grid_coords), location));
        }

        let up = y + 1;
        let location = GridCoords::new(x, up);
        if !blocked_coords.in_blocked_coords(&location) {
            successors.push((location.distance(grid_coords), location));
        }
        successors.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some((_, grid_coords)) = successors.get(0) {
            GridCoords::new(grid_coords.x, grid_coords.y)
        } else {
            grid_coords.clone()
        }
    }
}

