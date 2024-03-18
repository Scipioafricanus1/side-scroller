mod systems;
mod components;
mod pathfinding;
mod animation;
mod grid;
mod movement;
mod combat;
mod schedule;
mod state;
mod randomizer;

pub mod prelude {
    pub use crate::schedule::*;
    pub use crate::state::*;
    pub use crate::systems::*;
    pub use crate::randomizer::*;
    pub use crate::components::*;
    pub use crate::pathfinding::*;
    pub use crate::animation::*;
    pub use crate::grid::*;
    pub use crate::combat::*;
    pub use crate::movement::*;
    pub use bevy::{ prelude::*, window::{PrimaryWindow, WindowResized, WindowMode}};
    pub use bevy_ecs_ldtk::prelude::*;
    pub use futures_lite::future;
    pub use pathfinding::prelude::*;   

    pub const RAND_VARIANCE: u32 = 3;
    
}