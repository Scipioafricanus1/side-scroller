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

