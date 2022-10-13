use bevy::{prelude::*, core::FixedTimestep};
use bevy_rapier2d::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct SpellOrb;

const SPELLORB_COLOR: Color = Color::rgb(0., 0., 0.7);
const UPDATE_TIME_ORB_ROTATE_RADIUS_STEP: f32 = 2.0;
pub const UPDATE_TIME_ORB_ROTATE: f64 = 0.01;

pub fn rotate_spawnorb(
    time: Res<Time>,
    mut player_position: Query<&Transform, With<Player>>,
    mut spell : Query<&mut Transform, (With<SpellOrb>, Without<Player>)>,
) {
    let angle = std::f32::consts::PI / 2.0;
    for player_transform in player_position.iter() {
        let player_pos = player_transform.translation;
        for mut spell_transform in spell.iter_mut(){
                spell_transform.rotate_around(
                    //Vec3::new(0.,0.,0.),
                    player_pos,
                    Quat::from_rotation_z(-angle * UPDATE_TIME_ORB_ROTATE_RADIUS_STEP * time.delta_seconds()));
        }
    }
}

pub fn spawn_spellorb(mut commands: Commands){
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: SPELLORB_COLOR,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.,100.,0.),
            scale: Vec3::new(40.0, 40.0, 0.0),
            ..default()
        },
        ..default()
    })
        .insert(Collider::ball(0.5))
        .insert(SpellOrb)
        .insert(RigidBody::Dynamic)
        .insert(Sensor(true))
        .insert(ActiveEvents::COLLISION_EVENTS);
}
