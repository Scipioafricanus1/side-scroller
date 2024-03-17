use crate::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)] //TODO: Fix these up to make more sense later. What logical sets are there?
pub enum InGameSet {
    EntityUpdates,
    DespawnEntities,
    UserInput,
}

pub struct SchedulePlugin;
impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::EntityUpdates
            )
            .chain()
            .run_if( in_state(GameState::Combat)),
        )
        .add_systems(Update, apply_deferred
        .after(InGameSet::DespawnEntities)
        .before(InGameSet::UserInput));
    }
}