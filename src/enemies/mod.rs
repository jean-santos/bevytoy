use bevy::{core::FixedTimestep, prelude::*};
use bevy_rapier2d::prelude::*;

use rand::{prelude::random, thread_rng, Rng};

use crate::player::Player;

const ENEMY_COLOR: Color = Color::rgb(0.7, 0., 0.);

pub fn get_enemy_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(spawn_enemy)
}

#[derive(Component)]
pub struct Enemy;

pub fn enemies_movement(
    mut player_position: Query<&mut Transform, With<Player>>,
    mut enemies_position: Query<&mut Transform, (Without<Player>, With<Enemy>)>)
 {
    if let Ok(player) = player_position.get_single_mut() {
        for mut en_pos in enemies_position.iter_mut() {
            if en_pos.translation.x > player.translation.x {
                en_pos.translation.x -= 1.0;
            }else{
                en_pos.translation.x += 1.0;
            }
            if en_pos.translation.y > player.translation.y{
                en_pos.translation.y -= 1.0;
            }else{
                en_pos.translation.y += 1.0;
            }
        }
    }
}

pub fn spawn_enemy(
    mut player_position: Query<&mut Transform, With<Player>>,
    mut commands: Commands) {
    let mut rng = thread_rng();
    //p
    let mut c = Transform{
         scale: Vec3::new(20., 20., 0.),
          ..default()
    };


    if let Ok(player) = player_position.get_single_mut() {
        c.translation = player.translation;
        let distance = rng.gen_range(250.0..400.0);
        c.translation.y += distance;

        let angle = std::f32::consts::PI / 2.0;
        c.rotate_around(player.translation,
                    Quat::from_rotation_z(-angle * rng.gen_range(0.0..4.0) ));
    }
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: ENEMY_COLOR,
                ..default()
            },
            transform: c,
            ..default()
        })
        .insert(Enemy)
        .insert(RigidBody::Dynamic)
        .insert(Sensor(true))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Collider::ball(0.5));
}

