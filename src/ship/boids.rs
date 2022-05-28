use crate::ship::prelude::*;
use bevy::prelude::*;

use std::collections::HashMap;

pub struct BoidSystem {
    separation_settings: BoidSystemSetting,
    alignment_settings: BoidSystemSetting,
    cohesion_settings: BoidSystemSetting,
    field_of_view: f32,
}

impl Default for BoidSystem {
    fn default() -> Self {
        BoidSystem {
            separation_settings: BoidSystemSetting::default(),
            alignment_settings: BoidSystemSetting::default(),
            cohesion_settings: BoidSystemSetting::default(),
            field_of_view: 0.75,
        }
    }
}

pub struct BoidSystemSetting {
    range: f32,
    force: f32,
}

impl Default for BoidSystemSetting {
    fn default() -> Self {
        BoidSystemSetting {
            range: 100.,
            force: 1.,
        }
    }
}

fn separation(
    system: Res<BoidSystem>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Ship), With<Childship>>,
) {
    let range_squared = system.separation_settings.range * system.separation_settings.range;

    let mut combinations = query.iter_combinations_mut();
    while let Some([(a_transform, mut a_ship), (b_transform, mut b_ship)]) =
        combinations.fetch_next()
    {
        if a_transform
            .translation
            .distance_squared(b_transform.translation)
            < range_squared
            && a_ship.speed.dot(b_ship.speed) > -system.field_of_view
        {
            let a_to_b = (b_transform.translation - a_transform.translation)
                .truncate()
                .normalize();
            let factor = time.delta_seconds() * system.separation_settings.force;
            a_ship.speed -= a_to_b * factor;
            b_ship.speed += a_to_b * factor;
        }
    }
}

fn alignment(
    system: Res<BoidSystem>,
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut Ship), With<Childship>>,
) {
    let range_squared = system.alignment_settings.range * system.alignment_settings.range;

    let mut adjustments = HashMap::<Entity, (usize, Vec2)>::new();

    let mut combinations = query.iter_combinations_mut();
    while let Some([(a_entity, a_transform, a_ship), (_, b_transform, b_ship)]) =
        combinations.fetch_next()
    {
        if a_transform
            .translation
            .distance_squared(b_transform.translation)
            < range_squared
            && a_ship.speed.dot(b_ship.speed) > -system.field_of_view
        {
            let (ref mut count, ref mut total) =
                adjustments.entry(a_entity).or_insert((0, Vec2::ZERO));

            *count += 1;
            *total += b_ship.speed;
        }
    }

    for (entity, _, mut ship) in query.iter_mut() {
        if let Some((count, speed)) = adjustments.get(&entity) {
            ship.speed +=
                (*speed / *count as f32) * time.delta_seconds() * system.alignment_settings.force;
        }
    }
}

fn cohesion(
    system: Res<BoidSystem>,
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut Ship), With<Childship>>,
) {
    let range_squared = system.cohesion_settings.range * system.cohesion_settings.range;

    // Center of mass of the nearby boids of every boid.
    let mut com = HashMap::<Entity, (usize, Vec2)>::new();

    let mut combinations = query.iter_combinations_mut();
    while let Some([(a_entity, a_transform, a_ship), (_, b_transform, b_ship)]) =
        combinations.fetch_next()
    {
        if a_transform
            .translation
            .distance_squared(b_transform.translation)
            < range_squared
            && a_ship.speed.dot(b_ship.speed) > -system.field_of_view
        {
            let (ref mut count, ref mut total) = com.entry(a_entity).or_insert((0, Vec2::ZERO));

            *count += 1;
            *total += b_transform.translation.truncate();
        }
    }

    for (entity, transform, mut ship) in query.iter_mut() {
        if let Some((count, total_pos)) = com.get(&entity) {
            let com = *total_pos / *count as f32;
            let to_com = com - transform.translation.truncate();
            ship.speed += to_com / 10. * time.delta_seconds() * system.cohesion_settings.force;
        }
    }
}

pub struct BoidPlugin;
impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoidSystem::default())
            .add_system(separation)
            .add_system(alignment)
            .add_system(cohesion);
    }
}
