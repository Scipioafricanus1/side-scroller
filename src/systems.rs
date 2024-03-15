use crate::{components::{self, Clickable, MainCamera}, grid::{BlockedAreas, MyWorldCoords}, pathfinding::create_path};

use components::Goal;
use crate::prelude::*;



pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.set_maximized(true);

    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn((camera, MainCamera));
    
    commands.spawn( LdtkWorldBundle {
        ldtk_handle: asset_server.load( "tile_based_game.ldtk"),
        ..Default::default()
    });
}

pub fn window_resize_system(mut resize_events: EventReader<WindowResized>) {
    for event in resize_events.read() {
        println!("width = {} height = {}", event.width, event.height);
    }
}


pub fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<components::Player>>,
    input: Res<Input<KeyCode>>,
    level_walls: Res<BlockedAreas>,
) {
    let movement_direction = if input.just_pressed(KeyCode::W) {
        GridCoords::new(0, 1)
    } else if input.just_pressed(KeyCode::A) {
        GridCoords::new(-1, 0)
    } else if input.just_pressed(KeyCode::S) {
        GridCoords::new(0, -1)
    } else if input.just_pressed(KeyCode::D) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_blocked_coords(&destination) {
            *player_grid_coords = destination;
            eprintln!("playerGridCoords: x: {}, y: {}", player_grid_coords.x, player_grid_coords.y)
        }
    }
}

pub fn click_drag_pathing(
    mut commands: Commands,
    mut my_coords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
    mut players: Query<(Entity, &GridCoords, &mut Clickable), With<components::Player>>,
    blocked_areas: Res<BlockedAreas>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    //get world position using window and camera.
    if let Some(world_position) = window.cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate())
    {
        my_coords.0 = world_position;
        
        for (target, player_coords, mut clickable) in players.iter_mut() {
            let destination = GridCoords::from(my_coords.as_ref());

            if buttons.just_pressed(MouseButton::Left) {
                eprintln!("World coords: {}/{}", world_position.x, world_position.y);
                eprintln!("Dest coords: x: {} y: {}", destination.x, destination.y);
                //check if player entity clicked
                if player_coords == &destination {
                    clickable.is_clicked = true ;
                }
                    
            }
            if buttons.just_released(MouseButton::Left) {
                //get path to location. 
                if clickable.is_clicked && !blocked_areas.in_blocked_coords(&destination) {
                    // *player_coords = destination; // sets player pos to destionation.
                    // creates path for player to move towards slowly
                    create_path(
                        &mut commands,
                        target,
                        &blocked_areas,
                        player_coords.clone(),
                        destination,
                    );
                }
                clickable.is_clicked = false;
            }
        }
    }
}

pub fn check_goal(
    level_selection: ResMut<LevelSelection>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    goals: Query<&GridCoords, With<Goal>>,
) {
    if players
        .iter()
        .zip(goals.iter())
        .any(|(player_grid_coords, goal_grid_coords)| player_grid_coords == goal_grid_coords) 
    {
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };

        indices.level += 1;
    }
}
