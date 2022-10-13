
use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;

use crate::spells;

#[derive(Component)]
pub struct Player;

const PLAYER_COLOR: Color = Color::rgb(0.7, 0., 0.);

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(50.0, 50.0, 50.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Sensor(true))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Collider::ball(0.5))
        .insert(Player);
    spells::spawn_spellorb(commands);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_position: Query<&mut Transform, With<Player>>,
    mut pets: Query<&mut Transform, (With<spells::SpellOrb>,Without<Player>)>,
) {
    for mut transform in player_position.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
    for mut transform in pets.iter_mut(){
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }

    }
}
