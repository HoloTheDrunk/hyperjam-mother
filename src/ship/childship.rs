use bevy::prelude::*;

#[derive(Component)]
pub struct Childship {
    pub mother: Entity,
    pub state: State,
}

pub enum State {
    Idle,
    Gathering { target: Entity },
}

mod boids {
    use super::*;
    use crate::ship::ship::Ship;

    const RANGE: f32 = 20.;
    const FORCE: f32 = 2.;

    fn separation(mut query: Query<(&Transform, &mut Ship), With<Childship>>) {
        let range_squared = RANGE * RANGE;

        let mut combinations = query.iter_combinations_mut();
        while let Some([(a_transform, mut a_childship), (b_transform, mut b_childship)]) =
            combinations.fetch_next()
        {
            if a_transform
                .translation
                .distance_squared(b_transform.translation)
                < range_squared
            {
                println!("oh no");
            }
        }
    }
}
