use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Component)]
pub struct Mothership;

#[derive(Component)]
pub struct Childship {
    pub mother: Entity,
    pub state: State,
    pub action_queue: VecDeque<State>,
}

#[derive(Component)]
pub struct Target {
    pub count: u32,
}

#[derive(Default)]
pub enum State {
    #[default]
    Idle,
    Gathering {
        target: Entity,
        progress: f32,
    },
}
