use crate::prelude::*;

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, combat_turn.chain().in_set(InGameSet::EntityUpdates));
    }
}

pub fn combat_turn() { // queries initiative on change. 

}

pub fn roll_initiative() { // on CombatEvent, roll initiative. 

}