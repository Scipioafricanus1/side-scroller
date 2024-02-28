use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod systems;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins((
        LdtkPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
    ))
    .insert_resource(RapierConfiguration {
        gravity: Vec2::new(0.0, -2000.0), 
        ..Default::default()
    })
    .insert_resource(LevelSelection::Uid(0))
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation { 
            load_level_neighbors: true, 
        },
        set_clear_color: SetClearColor::FromLevelBackground,
        ..Default::default()
    })
    .add_systems(Startup, systems::setup)
    .add_systems(Update, systems::spawn_wall_collision)
    .add_systems(Update, systems::movement)
    .add_systems(Update, systems::detect_climb_range)
    .add_systems(Update, systems::ignore_gravity_if_climbing)
    .register_ldtk_int_cell::<components::WallBundle>(1)
    .register_ldtk_int_cell::<components::LadderBundle>(2)
    .register_ldtk_int_cell::<components::WallBundle>(3)
    .register_ldtk_entity::<components::PlayerBundle>("Player")
    .register_ldtk_entity::<components::MobBundle>("Mob")
    .register_ldtk_entity::<components::ChestBundle>("Chest")
    .register_ldtk_entity::<components::PumpkinsBundle>("Pumpkins")
    .run();
    
}
