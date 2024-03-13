use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;
use noisy_float::prelude::*;

use crate::{animation::{AnimationTimer, LastDirection}, pathfinding::AiPath, systems::GRID_SIZE};



#[derive(Default, Component)]
pub struct Player;

#[derive(Component)]
pub struct Clickable {
    pub is_clicked: bool,
}
impl Default for Clickable {
    fn default() -> Self {
        Clickable {
            is_clicked: false
        }
    }
}
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    clickable: Clickable,
    ai_path: AiPath,
    last_direction: LastDirection,
    animation_timer: AnimationTimer,
}

#[derive(Default, Component)]
pub struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
pub struct GoalBundle {
    goal: Goal,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

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

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct  MyWorldCoords(pub Vec2);

impl From<&MyWorldCoords> for GridCoords{
    fn from(world_coords: &MyWorldCoords) -> Self {
        GridCoords::new(world_coords.0.x.round() as i32 / GRID_SIZE, world_coords.0.y.round() as i32 / GRID_SIZE)
    }
}
pub trait Distance {
    fn distance(&self, grid_coords: &GridCoords) -> i32;
}
impl Distance for GridCoords {
    fn distance(&self, grid_coords: &GridCoords) -> i32 {
        let float: f32 = r32((self.x as f32 - grid_coords.x as f32).hypot(self.y as f32 - grid_coords.y as f32)).raw();
        float.round() as i32
    }
}

