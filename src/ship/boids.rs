use crate::ship::prelude::*;
use bevy::prelude::*;

use std::collections::HashMap;

pub struct BoidSystem {
    separation_settings: BoidSystemSetting,
    alignment_settings: BoidSystemSetting,
    cohesion_settings: BoidSystemSetting,
}

pub struct BoidSystemSetting {
    range: f32,
    force: f32,
}

pub fn separation(
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

pub fn alignment(
    system: Res<BoidSystem>,
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut Ship), With<Childship>>,
) {
    let range_squared = system.alignment_settings.range * system.alignment_settings.range;

    let mut adjustments = HashMap::<Entity, (usize, Vec2)>::new();

    let mut combinations = query.iter_combinations_mut();
    while let Some([(a_entity, a_transform, _), (_, b_transform, b_ship)]) =
        combinations.fetch_next()
    {
        if a_transform
            .translation
            .distance_squared(b_transform.translation)
            < range_squared
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
