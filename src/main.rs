use avian2d::prelude::*;
use bevy::{
    ecs::query, log::tracing_subscriber::fmt::format, math::NormedVectorSpace, prelude::*,
    transform::commands,
};
#[cfg(debug_assertions)]
mod debug;
mod enemmey;
mod health;
mod level1;
mod level2;
mod level3;
mod modLevH;
mod physics;
mod pla;
mod score;
use crate::{
    GameState::GamePlay,
    debug::DebugPlugin,
    enemmey::{Hostile, ImageHandles, Mob, MobHealth, Mobhandle},
    level1::Asteroid,
    level3::shipPart,
    modLevH::{
        level,
        levelState::{self, levelEnd},
    },
    physics::GameLayer,
    pla::{Hit, Pla, Shoot, shoot, start},
    score::Score,
};
fn speedUPTime(real_time: Res<Time<Real>>, mut name: ResMut<Time<Virtual>>) {
    let a = name.relative_speed() + 0.03 * real_time.delta_secs();
    name.set_relative_speed(a);
}
#[derive(Event, Message)]
struct Restart;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            level2::level2Plugin,
            level3::MyLevel3Plugin,
            modLevH::MyLevelH,
            score::ScorePlugin,
            health::HealthPlugin,
        ))
        .add_plugins(physics::physicsPlugin)
        .add_plugins(
            #[cfg(debug_assertions)]
            DebugPlugin,
        )
        .add_plugins(level1::MyLevel1Plugin)
        .add_systems(Startup, (start).chain())
        .add_systems(OnEnter(GameState::GameOver), game_over_ui)
        .add_systems(Update, shoot)
        .add_systems(
            FixedUpdate,
            (
                shootHit,
                despanw,
                speedUPTime,
                (game_over, start_deSpawnMobs).run_if(on_message::<Hit>.or(on_message::<GameOver>)),
                (resetSpeedUPTime, restart, start)
                    .chain()
                    .run_if(on_message::<Restart>),
                (un_game_over_ui).run_if(in_state(GameState::GameOver)),
            ),
        )
        .add_message::<Hit>()
        .add_message::<Restart>()
        .init_state::<GameState>()
        .add_message::<GameOver>()
        .init_resource::<ImageHandles>()
        .run();
}
fn resetSpeedUPTime(real_time: Res<Time<Real>>, mut name: ResMut<Time<Virtual>>) {
    name.set_relative_speed(1.);
}

fn restart(
    mut gs: ResMut<NextState<GameState>>,
    mut ls: ResMut<NextState<levelState>>,
    mut l: ResMut<NextState<level>>,
    mut score: ResMut<Score>,
    query: Query<Entity, Or<(With<Transform>, With<Text>)>>,
    mut commands: Commands,
) {
    score.0 = 0;
    NextState::set_if_neq(&mut gs, GameState::GamePlay);
    NextState::set_if_neq(&mut ls, levelState::Inlevel);
    NextState::set_if_neq(&mut l, level::level1);
    for query in query {
        commands.entity(query).try_despawn();
    }
}

#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
enum GameState {
    GameOver,
    #[default]
    GamePlay,
}

#[derive(Default, Message)]
pub struct GameOver;

fn game_over(mut s: ResMut<NextState<GameState>>) {
    NextState::set_if_neq(&mut s, GameState::GameOver);
}
#[derive(Component)]
struct ReButton;
fn game_over_ui(mut score: ResMut<score::Score>, mut commands: Commands) {
    commands.spawn((
        Node {
            height: percent(100.),
            width: percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,

            ..Default::default()
        },
        DespawnOnExit(GameState::GameOver),
        children![
            (Text::new("game over")),
            (Text::new(format!("{}", score.0))),
            (
                Button,
                ReButton,
                children![(Text::new("Retry"), retrybutttonText,)]
            ),
        ],
    ));
}
fn start_deSpawnMobs(mut commands: Commands, query: Query<Entity, (With<movey>)>) {
    for query in query {
        commands.entity(query).despawn();
    }
}
#[derive(Component)]

struct retrybutttonText;
fn un_game_over_ui(
    mut s: ResMut<NextState<GameState>>,
    mut ss: ResMut<NextState<level>>,
    mut mes: MessageWriter<Restart>,

    mut color: Single<&mut TextColor, With<retrybutttonText>>,
    mut interaction_query: Query<(Entity, &Interaction), (With<ReButton>, Changed<Interaction>)>,
) {
    for (e, interaction) in interaction_query {
        match interaction {
            Interaction::Pressed => {
                mes.write(Restart);
            }
            Interaction::Hovered => color.0 = Color::srgb(0.5, 0.5, 0.5),
            Interaction::None => color.0 = Color::default(),

            _ => {}
        }
    }
}

#[derive(Component)]
struct movey(f32);

fn shootHit(
    collisions: Collisions,

    mut commands: Commands,
    shoot: Query<(Entity, &Transform), With<Shoot>>,
    mut hos: Query<(
        Entity,
        &Transform,
        Has<Asteroid>,
        Has<Mob>,
        Has<shipPart>,
        Option<&mut MobHealth>,
    )>,
) {
    //  println!("{}", shoot.iter().len());
    for shoot in shoot {
        let x = collisions.entities_colliding_with(shoot.0);
        // println!("{:?}", x.collect::<Vec<_>>());
        for inpac in collisions.entities_colliding_with(shoot.0) {
            if let Ok((e, p, asteroid, mob, ship, mut health)) = hos.get_mut(inpac) {
                if asteroid {
                    commands.entity(shoot.0).despawn();
                };

                // println!("{}, {}, {}, {}", e, asteroid, mob, ship);
                if ship {
                    commands.entity(shoot.0).despawn();
                };
                if mob {
                    commands.trigger(score::ScoreEvent(1));
                    match health {
                        Some(ref mut a) => {
                            a.0 = a.0 - 1;
                            if a.0 == 0 {
                                commands.entity(e).despawn();
                            };
                        }
                        None => {
                            commands.entity(e).despawn();
                        }
                    }

                    commands.entity(shoot.0).despawn();
                    //  println!("ddd");
                }
            }
        }
    }
}

fn hit(
    pla_transform: Query<(&Transform), With<Pla>>,
    mob_transform: Query<(&Transform, Entity), With<Hostile>>,
    mut commands: Commands,
    mut mes: MessageWriter<Hit>,
) {
    for pla_transform in pla_transform {
        for mob_transform in mob_transform {
            if pla_transform
                .translation
                .xy()
                .distance(mob_transform.0.translation.xy())
                <= 61.
            {
                mes.write(Hit {
                    hit: mob_transform.1,
                });
                commands.trigger(Hit {
                    hit: mob_transform.1,
                });
            }
        }
    }
}

fn despanw(query: Query<(&Transform, Entity, Has<Mob>), With<movey>>, mut commands: Commands) {
    for (y, e, mob) in query {
        if y.translation.y < -540. {
            commands.entity(e).despawn();
            if mob {
                commands.trigger(health::healthEvent(1));
            }
        }
        if y.translation.y > 1000. {
            commands.entity(e).despawn();
        }
    }
}
