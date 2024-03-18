use crate::prelude::*;

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
    combat: Combat,
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

#[derive(Component)]
pub struct MainCamera;
