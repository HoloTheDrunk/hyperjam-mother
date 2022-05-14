use bevy::prelude::*;

#[derive(Component)]
pub struct Mothership;

#[derive(Component)]
pub struct Childship {
    pub mother: Entity,
    pub state: State,
}

pub enum State {
    Idle,
    Gathering { target: Entity },
}
