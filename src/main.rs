use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod enemies;
mod player;
mod spells;

#[derive(Component)]
struct ColorText;

#[derive(Default)]
struct Game {
    score: i32,
    max_hp: i32,
    curr_hp: i32,
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(ColorText);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn setup_game(mut game: ResMut<Game>) {
    game.score = 0;
    game.max_hp = 5;
    game.curr_hp = game.max_hp;
}

fn scoreboard_system(game: Res<Game>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!(
        r"Score: {}
HP: {}/{}",
        game.score, game.curr_hp, game.max_hp
    );
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_startup_system(setup_game)
        .add_startup_system(setup_gravity)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_text)
        .add_startup_system(player::spawn_player)
        .add_system(player::player_movement)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(spells::UPDATE_TIME_ORB_ROTATE))
                .with_system(spells::rotate_spawnorb),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemies::spawn_enemy),
        )
        .add_plugins(DefaultPlugins)
        //.alugin(WorldInspectorPl ugin::new())
        .insert_resource(RapierConfiguration { ..default() }) // Optionally define gravity
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_system(display_events)
        .add_system(enemies::enemies_movement)
        .add_system(scoreboard_system)
        .run();
}

fn display_events(
    mut commands: Commands,
    mut spell_query: Query<Entity, (With<spells::SpellOrb>, Without<player::Player>)>,
    mut player_query: Query<Entity, With<player::Player>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut game: ResMut<Game>,
) {
    if let Ok(spell) = spell_query.get_single_mut() {
        if let Ok(player) = player_query.get_single_mut() {
            for collision_event in collision_events.iter() {
                if let CollisionEvent::Started(collider_entity1, collider_entity2, _) =
                    collision_event
                {
                    if collider_entity1 == &player || collider_entity2 == &player {
                        // Colidiu com um player
                        if game.curr_hp > 0 {
                            game.curr_hp -= 1;
                        }
                        if collider_entity1 == &player {
                            commands.entity(*collider_entity2).despawn();
                        } else {
                            commands.entity(*collider_entity1).despawn();
                        }
                    } else if collider_entity1 == &spell || collider_entity2 == &spell {
                        // Colidiu com spell
                        game.score += 1;
                        if collider_entity1 == &spell {
                            commands.entity(*collider_entity2).despawn();
                        } else {
                            commands.entity(*collider_entity1).despawn();
                        }
                    }
                }
            }
        }
    }
}
