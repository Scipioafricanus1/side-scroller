mod grid_structs;
pub use grid_structs::*;
use crate::prelude::*;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            cache_wall_locations,
            translate_grid_coords_entities,
        ).in_set(InGameSet::EntityUpdates));
    }
}

pub fn cache_wall_locations(
    mut level_walls: ResMut<BlockedAreas>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
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

            let new_level_walls = BlockedAreas {
                blocked_locations: wall_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
}

pub fn translate_grid_coords_entities(
    mut grid_coord_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coord_entities.iter_mut() {
        transform.translation = 
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}