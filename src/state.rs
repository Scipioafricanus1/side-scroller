pub use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Overworld,
    #[default]
    Combat,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
        .add_systems(Update, (
            game_state_input_events,
            transition_to_in_game.run_if(in_state(GameState::GameOver)),
        ));
    }
}


fn game_state_input_events(                         /**** NOTE: Combat shouldn't be a toggle with Paused. I'll fix that later. For now this is a learning experience. */
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Combat => {
                println!("changing gamestate to paused");
                next_state.set(GameState::Paused)
            }, 
            GameState::Paused => next_state.set(GameState::Combat),
            _ => (),
        }
    }
}

fn transition_to_in_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Combat);
}
