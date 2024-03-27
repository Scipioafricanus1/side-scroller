use crate::prelude::*;

pub struct AnimatePlugin;
impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate.in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct MovementTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

impl Default for MovementTimer {
    fn default() -> Self {
        MovementTimer(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct LastDirection(pub Vec2);

impl Default for LastDirection {
    fn default() -> Self {
        LastDirection(Vec2::ZERO)
    }
}



pub fn animate(
    mut sprites: Query<(&mut TextureAtlasSprite, &EnemyType, &LastDirection, &mut AnimationTimer), With<Enemy>>, 
    time: Res<Time>,
) {
    for (mut sprite, enemy_type, last_direction, mut animation_timer) in sprites.iter_mut() {
            animation_timer.tick(time.delta());

            if animation_timer.just_finished() {
                match enemy_type {
                    EnemyType::Knight(_ ) => {
                        sprite.index = if sprite.index == 30 {
                            println!("updating sprite index for Knight: {}", 0);
                            0
                        } else {
                            println!("updating sprite index for Knight: {}", sprite.index+1);
                            sprite.index +1
                        }
                    }
                    EnemyType::Shooter(_) => {
                        println!("updating sprite index for shooter: {}", 0);
                        sprite.index = if sprite.index == 30 {
                            0
                        } else {
                            println!("updating sprite index for shooter: {}", sprite.index+1);
                            sprite.index +1
                        }
                    },
                }
            } 
    }
}