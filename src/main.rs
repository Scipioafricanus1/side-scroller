use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::MyWorldCoords;
use pathfinding::PathfindingPlugin;

mod systems;
mod components;
mod pathfinding;
mod animation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(PathfindingPlugin)
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
                systems::click_drag_pathing,
            )
        )
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .init_resource::<components::BlockedAreas>()         
        .init_resource::<MyWorldCoords>() 
        .run();
}

