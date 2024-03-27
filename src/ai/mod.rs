use crate::prelude::*;

pub struct AIPlugin;
impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_ai.run_if(in_state(CombatLoop::DuringTurn)));
    }
}


pub fn enemy_ai( 
    mut commands: Commands,
    enemies: Query<(&Combat, &GridCoords, Entity), With<Enemy>>,
    players: Query<(&AiPath, &GridCoords), With<Player>>,
    mut initiative_rolls: ResMut<InitiativeRolls>,
    blocked_coords: Res<BlockedAreas>,
) {
    if !initiative_rolls.initiatives.is_empty() {
        if let Some(entity) = initiative_rolls.initiatives.front() {
            if let Ok((combat, enemy_coords, target)) = enemies.get(*entity) {
                println!("Got entity");
                let mut distances: Vec<(usize, GridCoords)> = Vec::new();
                for (ai_path, player_coords) in players.iter() {
                    if let Some(vec) = ai_path.locations.back() {
                        println!("Vec x {}, y {}", vec.x, vec.y);
                        let final_coords = GridCoords::new(vec.x as i32, vec.y as i32);
                        distances.push((enemy_coords.distance(&final_coords), final_coords ));
                    } else {
                        distances.push((enemy_coords.distance(player_coords), GridCoords::new(player_coords.x as i32, player_coords.y as i32)));
                    }
                }
                distances.sort_by(|a, b| a.0.cmp(&b.0));
                if let Some((_, grid_coords)) = distances.get(0) {
                    //get next closest spot
                    let end = grid_coords.closest_around(enemy_coords, &blocked_coords);
                    println!("End: x {}, y {}", end.x, end.y);
                    create_path(&mut commands, target, &blocked_coords, enemy_coords.clone(), end);
                    initiative_rolls.initiatives.pop_front();
                }
                
            }
        }
    }
}