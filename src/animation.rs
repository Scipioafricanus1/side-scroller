use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct LastDirection(pub Vec2);

impl Default for LastDirection {
    fn default() -> Self {
        LastDirection(Vec2::ZERO)
    }
}