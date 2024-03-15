use bevy::{prelude::*, window::WindowMode};
use bevy::input::common_conditions::input_toggle_active;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use side_scroller::prelude::*;



pub const WIDTH: f32 = 2560.0;
pub const HEIGHT: f32 = 1440.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "turn-based game".into(),
                    mode: WindowMode::Windowed,
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
        .add_plugins(LdtkPlugin)
        .add_plugins(PathfindingPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .add_systems(
            Update, 
            (
                move_player_from_input,
                translate_grid_coords_entities,
                cache_wall_locations,
                check_goal,
                click_drag_pathing,
                window_resize_system,
            )
        )
        .register_ldtk_int_cell::<WallBundle>(1)
        .init_resource::<BlockedAreas>()         
        .init_resource::<MyWorldCoords>() 
        .run();
}
