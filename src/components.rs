use std::default;

use crate::prelude::*;

#[derive(Default, Component)]
pub struct Player;

#[derive(Component)]
pub struct Clickable {
    pub clickable: bool,
    pub is_clicked: bool,
}
impl Default for Clickable {
    fn default() -> Self {
        Clickable {
            clickable: false,
            is_clicked: false
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    clickable: Clickable,
    combat: Combat,
    actor_bundle: LdtkActorBundle,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    enemy: Enemy,
    combat: Combat,
    combat_class: CombatClass,
    ai_type: AIType,
    actor_bundle: LdtkActorBundle,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct LdtkActorBundle {
    
    ai_path: AiPath,
    last_direction: LastDirection,
    animation_timer: AnimationTimer,
}



#[derive(Default, Component)]
pub enum AIType {
    #[default]
    Aggressive,
    Squadmate,
    Flanker,
    Defender,
}

#[derive(Default, Component)]
pub enum CombatClass {
    #[default]
    Melee,
    Ranged,
}

#[derive(Default, Component)]
pub struct Enemy;

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
