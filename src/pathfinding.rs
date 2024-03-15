use std::collections::VecDeque;

use crate::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use pathfinding::prelude::*;

pub struct PathfindingPlugin;
impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
             (
                apply_pathfinding,
                follow_path,
             )
        );
    }
}

#[derive(Component, Default)]
pub struct AiPath {
    pub locations: VecDeque<Vec2>,
}

pub struct Path {
    pub steps: Vec<GridCoords>,
}

#[derive(Debug)]
pub struct PathfindingError;



#[derive(Component)]
pub struct PathfindingTask(Task<Result<Path, PathfindingError>>);

pub fn neumann_neighbors(blocked_coords: &BlockedAreas, location: &GridCoords) -> Vec<GridCoords> {
    let (x, y) = (location.x, location.y);
    let mut successors = Vec::new();
    
    if let Some(left) = x.checked_sub(1) {
        let location = GridCoords::new(left, y);
        if !blocked_coords.in_blocked_coords(&location) {
            successors.push(location);
        }
    }
    if let Some(down) = y.checked_sub(1) {
        let location = GridCoords::new(x, down);
        if !blocked_coords.in_blocked_coords(&location) {
            successors.push(location);
        }
    }
    
    let right = x + 1;
    let location = GridCoords::new(right, y);
    if !blocked_coords.in_blocked_coords(&location) {
        successors.push(location);
    }

    let up = y + 1;
    let location = GridCoords::new(x, up);
    if !blocked_coords.in_blocked_coords(&location) {
        successors.push(location);
    }
    
    for coords in successors.iter() {
        println!("successors: x: {}, y: {}", coords.x, coords.y);
    }
    successors
}


pub fn   path_to(
    blocked_coords: &BlockedAreas,
    start: &GridCoords,
    goal: &GridCoords,
) -> Result<Path, PathfindingError> {
    let result = astar(
        start,
         |p| {
            neumann_neighbors(blocked_coords, p)
            .iter()
            .map(|neighbor| (neighbor.clone(), 1))
            .collect::<Vec<_>>()
         },
          |p| p.distance(goal) / 3,
            |p| {
                println!("start: x {} y {} goal: x {} y {}", p.x, p.y, goal.x, goal.y);
                p == goal
            },
    );

    if let Some((steps, _length)) = result {
        Ok(Path { steps })
    } else {
        Err(PathfindingError)
    }
}

pub fn create_path (
    commands: &mut Commands,
    target: Entity,
    blocked_coords: &BlockedAreas,
    start: GridCoords,
    end: GridCoords,
) {
    if blocked_coords.in_blocked_coords(&end) {
        println!("stopped at blocked coords: x: {}, y: {}", end.x, end.y);
        return;
    }
    
    let thread_pool = AsyncComputeTaskPool::get();

    let clone_blocked_coords = Box::new(blocked_coords.clone());

    let task = thread_pool.spawn(async move {
        let path: Result<Path, PathfindingError> = path_to(&clone_blocked_coords, &start, &end);
        let cloned_path = path_to(&clone_blocked_coords, &start, &end);
        match cloned_path {
            Ok(actual_path) => {
                for coords in actual_path.steps.iter() {
                    println!("step coords.x: {}, coords.y: {}", coords.x, coords.y);
                }
            },
            Err(_) => {
                println!("path should be found here. start.x: {}, start.y: {}, end.x: {}, end.y: {}", start.x, start.y, end.x, end.y);
            }
        };
        
        path
    });

    commands.entity(target).insert(PathfindingTask(task));
}

pub fn apply_pathfinding(
    mut commands: Commands,
    mut paths: Query<&mut AiPath>,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(task_entity).remove::<PathfindingTask>();

            if let Ok(mut ai_path) = paths.get_mut(task_entity) {
                if let Ok(path) = result {
                    ai_path.locations.clear();
                    for location in path.steps.iter() {
                        ai_path
                            .locations
                            .push_back(Vec2::new(location.x as f32, location.y as f32));
                    }
                }
            }
        }
    }
}

pub fn follow_path(
    mut players: Query<(&mut AiPath, &mut GridCoords, &mut LastDirection, &mut AnimationTimer), With<Player>>,
    time: Res<Time>,
) {
    for (mut path, mut grid_coords, mut last_direction, mut animation_timer) in players.iter_mut() {
        if let Some(next_target) = path.locations.front() {
            let (diff_x, diff_y) = (next_target.x - grid_coords.x as f32, next_target.y - grid_coords.y as f32);
            let delta = Vec2::new(diff_x, diff_y);
            let travel_amount = time.delta_seconds();
            animation_timer.tick(time.delta());

            if !animation_timer.just_finished() {
                let direction = delta.normalize().extend(0.0) * travel_amount;
                last_direction.0 = direction.truncate();
            } else {
                // println!("delta x rounded: {} \n delta y rounded: {}", delta.x.round(), delta.y.round() );
                grid_coords.x += delta.x.round() as i32;
                grid_coords.y += delta.y.round() as i32;
                path.locations.pop_front();
            }
        }
    }
}