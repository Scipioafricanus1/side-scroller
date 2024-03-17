use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
pub use std::collections::VecDeque;
pub use bevy::tasks::Task;

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
pub struct PathfindingTask(pub Task<Result<Path, PathfindingError>>);
