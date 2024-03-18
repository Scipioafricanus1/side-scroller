use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CombatLoop {
    #[default]
    StartTurn,
    DuringTurn,
}

#[derive( Component)]
pub struct Combat {
    pub initiative: u32,
    pub health: u32,
    pub melee_attack: u32,
    pub ranged_attack: u32,
    pub intelligence: u32, //potency of spells both learned and in practice
    pub wisdom: u32, //spells available
    pub melee_defence: u32,
    pub ranged_defence: u32,
    pub morale: u32,
}

impl Default for Combat {
    fn default() -> Self {
        Self { 
            initiative: 100,
            health: 50, 
            melee_attack: 10, 
            ranged_attack: 10, 
            intelligence: 10, 
            wisdom: 10, 
            melee_defence: 5, 
            ranged_defence: 5, 
            morale: 10 
        }
    }
}

#[derive(Default, Resource, Clone)]
pub struct InitiativeRolls {
    pub initiatives: Vec<Entity>,
}


pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<CombatLoop>()
        .init_resource::<InitiativeRolls>()
        .add_systems(
            OnEnter(CombatLoop::StartTurn),
            roll_initiative.in_set(InGameSet::EntityUpdates)
        )
        .add_systems(Update, 
            combat_turn.run_if(in_state(CombatLoop::DuringTurn))
            .in_set(InGameSet::EntityUpdates));
    }
}

pub fn combat_turn(
    mut next_state: ResMut<NextState<CombatLoop>>,
    state: Res<State<CombatLoop>>,
    combat_entities: Query<(&Combat, Option<&Player>)>, //Add Option Enemy too later.
    mut initiative_rolls: ResMut<InitiativeRolls>,
) { // queries initiative on change. 
    if let Some(entity) = initiative_rolls.initiatives.get(0) {
        if let Ok((combat, player_opt)) = combat_entities.get(*entity){
            if let Some(player) = player_opt {
                println!("This is a player-controlled character!");
                initiative_rolls.initiatives.remove(0);
            } else { //should be an enemy then. don't have them implemented yet.
                println!("They should be an enemy then.")
            }
        } 
    } 
    if initiative_rolls.initiatives.is_empty() {
        println!("Setting to startTurn again since we finished list.");
        next_state.set(CombatLoop::StartTurn)
    }
}

///Note: querying for all entities with combatbundles now. Might be the case that
/// 1. Not all entities are in combat. Might always be the case. Not sure yet.
pub fn roll_initiative( 
    mut next_state: ResMut<NextState<CombatLoop>>,
    state: Res<State<CombatLoop>>,
    combat_entities: Query<(Entity, &Combat)>, 
    mut initiative_rolls: ResMut<InitiativeRolls>
) { // on CombatEvent, roll initiative. 

        let mut initiatives: Vec<(u32, Entity)> = Vec::new();
        for (entity, combat_bundle) in combat_entities.iter() {
            let init_roll = do_roll(combat_bundle.initiative);
            initiatives.push((init_roll, entity));
            println!("Init roll {}, entity {:?}", init_roll, entity);
        }
        initiatives.sort_by(|a, b| a.0.cmp(&b.0));
        
        for (value, entity) in initiatives.iter() {
            println!("initiative value: {}", value);
        }
        initiative_rolls.initiatives = initiatives.iter().map(|val| val.1).collect();

        match state.get() {
            CombatLoop::StartTurn => {
                println!("Setting state to duringTurn");
                next_state.set(CombatLoop::DuringTurn)
            },
            _ => println!("Should always be startTurn when this runs")
        }
}


