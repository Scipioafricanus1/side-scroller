use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;



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

#[derive(Default, Resource)]
pub struct LevelWalls {
    pub wall_locations: HashSet<GridCoords>,
    pub level_width: i32,
    pub level_height: i32,
}

impl LevelWalls {
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}