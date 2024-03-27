use std::str::FromStr;

use crate::prelude::*;

use thiserror::Error;

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
    actor_bundle: LdtkActorBundle,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[with(EnemyType::from_field)]
    enemy_type: EnemyType,
}
#[derive(Default, Component)]
pub struct TypeBundle {
    pub combat_class: CombatClass,
    pub ai_type: AIType,
}
#[derive(Default, Bundle, LdtkEntity)]
pub struct LdtkActorBundle {
    ai_path: AiPath,
    last_direction: LastDirection,
    movement_timer: MovementTimer,
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


#[derive(Debug, Error)]
#[error("the given enemy value doesn't exist")]
pub struct NoSuchEnemyType;
#[derive(Component)]
pub enum EnemyType {
    Shooter(TypeBundle),
    Knight(TypeBundle),
}

impl Default for EnemyType {
    fn default() -> Self {
        EnemyType::Knight(
            TypeBundle {
                combat_class: CombatClass::Melee,
                ai_type: AIType::Aggressive,
            })
    }
}

impl EnemyType {
    pub fn from_field(entity_instance: &EntityInstance) -> EnemyType {
        let enemy_type = entity_instance.get_enum_field("EnemyType").expect("expected enemyType Enum here");
        EnemyType::from_str(&enemy_type).expect("expected enemy to be knight or Shooter")
    }
}
impl FromStr for EnemyType {
    type Err = NoSuchEnemyType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EnemyType::*;

        match s {
            "Knight" => {
                Ok(Knight(
                    TypeBundle {
                        combat_class: CombatClass::Melee,
                        ai_type: AIType::Aggressive,
                    }
                ))
            },
            "Shooter" => {
                Ok(Shooter(
                    TypeBundle {
                        combat_class: CombatClass::Ranged,
                        ai_type: AIType::Squadmate,
                    }
                ))
            },
            _ => Err(NoSuchEnemyType),
        }
    }
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
