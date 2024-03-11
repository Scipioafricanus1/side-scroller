use crate::components::{self, Clickable};
use bevy::{ prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct  MyWorldCoords(Vec2);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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


pub fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<components::Player>>,
    input: Res<Input<KeyCode>>,
    level_walls: Res<components::LevelWalls>,
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
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
            eprintln!("playerGridCoords: x: {}, y: {}", player_grid_coords.x, player_grid_coords.y)
        }
    }
}

pub fn cursor_system(
    mut my_coords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
    mut players: Query<(&mut GridCoords, &mut Clickable), (With<components::Player>, With<Clickable>)>,
    level_walls: Res<components::LevelWalls>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
   
    if let Some(world_position) = window.cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate())
    {
        my_coords.0 = world_position;
        
        for (mut player_coords, mut clickable) in players.iter_mut() {
            let destination = GridCoords::new(my_coords.0.x.round() as i32/GRID_SIZE, my_coords.0.y.round() as i32 /GRID_SIZE);

            if buttons.just_pressed(MouseButton::Left) {
                eprintln!("World coords: {}/{}", my_coords.0.x, my_coords.0.y);
                eprintln!("Dest coords: x: {} y: {}", destination.x, destination.y);
                //check if player entity clicked
                
                if player_coords.as_ref() == &destination {
                    clickable.is_clicked = true ;
                }
                    
            }
            if buttons.just_released(MouseButton::Left) {
                if clickable.is_clicked && !level_walls.in_wall(&destination) {
                    *player_coords = destination;
                }
                clickable.is_clicked = false;
            }
        }
        
    }

    

    

    // for mut player_grid_coords in players.iter_mut() {
    //     let destination = *player_grid_coords + movement_direction;
    //     if !level_walls.in_wall(&destination) {
    //         *player_grid_coords = destination;
    //     }
    // }


}

const GRID_SIZE: i32 = 16;

pub fn translate_grid_coords_entities(
    mut grid_coord_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coord_entities.iter_mut() {
        transform.translation = 
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}

pub fn cache_wall_locations(
    mut level_walls: ResMut<components::LevelWalls>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<components::Wall>>,
    ldtk_project_entites: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entites.single())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");
            
            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = components::LevelWalls {
                wall_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
}

pub fn check_goal(
    level_selection: ResMut<LevelSelection>,
    players: Query<&GridCoords, (With<components::Player>, Changed<GridCoords>)>,
    goals: Query<&GridCoords, With<components::Goal>>,
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