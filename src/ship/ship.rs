use crate::ship::{ prelude::*, childship::State };

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use std::f32::consts::FRAC_PI_2;

pub struct ShipTexture(Handle<Image>);

#[derive(Component)]
pub struct Ship {
    speed: Vec2,
}

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(spawning)
            .add_system(accelerate)
            .add_system(follow_mothership)
            .add_system(apply_speed);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ShipTexture(asset_server.load::<Image, _>("ship.png")));

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("mothership.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(Ship { speed: Vec2::ZERO })
        .insert(Mothership);
}

fn accelerate(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Ship, &Transform), With<Mothership>>,
) {
    let (mut ox, mut oy) = (0., 0.);

    if input.pressed(KeyCode::W) {
        oy += 1.;
    }

    if input.pressed(KeyCode::A) {
        ox -= 1.;
    }

    if input.pressed(KeyCode::S) {
        oy -= 1.;
    }

    if input.pressed(KeyCode::D) {
        ox += 1.;
    }

    for (mut transform, _) in query.iter_mut() {
        transform.speed *= 1. - 0.33 * time.delta_seconds();
        transform.speed += Vec2::new(ox, oy);
    }
}

fn spawning(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    query: Query<(Entity, &Transform), With<Mothership>>,
    ship_texture: Res<ShipTexture>,
) {
    if input.just_pressed(KeyCode::H) {
        for (mother, transform) in query.iter() {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: ship_texture.0.clone(),
                    transform: *transform,
                    ..default()
                })
                .insert(Childship {
                    mother,
                    state: State::Idle,
                });
        }
    }
}

fn follow_mothership(
    time: Res<Time>,
    mut ships: Query<(&mut Transform, &mut Ship, &Childship), Without<Mothership>>,
    motherships: Query<&mut Transform, With<Mothership>>,
) {
    for (mut transform, mut ship, childship) in ships.iter_mut() {
        if let Ok(mother) = motherships.get(childship.mother) {
            let distance = mother.translation.distance(transform.translation);

            // TODO: Fix rotation to be towards the current speed.
            let (pos, target) = (
                transform.translation.truncate(),
                mother.translation.truncate(),
            );

            let angle = (pos - target).angle_between(pos);
            transform.rotation = Quat::from_rotation_z(-angle - FRAC_PI_2);

            // TODO: Remove static condition
            // TODO: Add movement towards mothership
            // transform.translation += forward * SHIP_SPEED * time.delta_seconds();
            let forward: Vec2 = transform.up().truncate();
            ship.speed *= 1. - 0.33 * time.delta_seconds();
            ship.speed += if distance > 50. { forward } else { -forward };
        }
    }
}

fn apply_speed(time: Res<Time>, mut ships: Query<(&mut Transform, &Ship)>) {
    for (mut transform, ship) in ships.iter_mut() {
        transform.translation += (ship.speed * time.delta_seconds()).extend(0.);
    }
}