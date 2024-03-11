use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use systems::MyWorldCoords;

mod systems;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, systems::setup)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::GoalBundle>("Goal")
        .add_systems(
            Update, 
            (
                systems::move_player_from_input,
                systems::translate_grid_coords_entities,
                systems::cache_wall_locations,
                systems::check_goal,
                systems::cursor_system,
            )
        )
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .init_resource::<components::LevelWalls>()         
        .init_resource::<MyWorldCoords>() 
        .run();
}

