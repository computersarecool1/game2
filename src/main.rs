use crate::{
    debug::DebugPlugin,
    enemy::{ImageHandles, Mob, MobHealth},
    health::{HealthEvent, update_health},
    level1::Asteroid,
    level3::ShipPart,
    mod_level_h::{
        Level,
        LevelState::{self},
    },
    pla::{Pla, Shoot, shoot, start},
    score::Score,
};
use avian2d::prelude::*;
use bevy::prelude::*;

#[cfg(debug_assertions)]
mod debug;
mod enemy;
mod health;
mod level1;
mod level2;
mod level3;
mod level4;
mod mod_level_h;
mod physics;
mod pla;
mod score;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            level2::Level2plugin,
            level3::MyLevel3Plugin,
            mod_level_h::MyLevelH,
            score::ScorePlugin,
            health::HealthPlugin,
        ))
        .add_plugins(physics::physicsPlugin)
        .add_plugins(
            #[cfg(debug_assertions)]
            DebugPlugin,
        )
        .add_plugins(level4::Level4plugin)
        .add_plugins(level1::MyLevel1Plugin)
        .add_systems(Startup, (start).chain())
        .add_systems(OnEnter(GameState::GameOver), game_over_ui)
        .add_systems(Update, shoot)
        .add_systems(
            FixedUpdate,
            (
                shoot_hit,
                despawn,
                speed_uptime,
                (game_over, start_de_spawn_mobs).run_if(on_message::<GameOver>),
                (reset_speed_uptime, restart, start)
                    .chain()
                    .run_if(on_message::<Restart>),
                (un_game_over_ui).run_if(in_state(GameState::GameOver)),
            ),
        )
        .add_message::<Restart>()
        .init_state::<GameState>()
        .add_message::<GameOver>()
        .init_resource::<ImageHandles>()
        .run();
}

#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
enum GameState {
    GameOver,
    #[default]
    GamePlay,
}

#[derive(Default, Message)]
pub struct GameOver;

#[derive(Event, Message)]
struct Restart;

#[derive(Component)]
struct ReButton;

#[derive(Component)]
struct RetrybutttonText;

fn speed_uptime(real_time: Res<Time<Real>>, mut name: ResMut<Time<Virtual>>) {
    let a = name.relative_speed() + 0.03 * real_time.delta_secs();
    name.set_relative_speed(a);
}

fn reset_speed_uptime(_real_time: Res<Time<Real>>, mut name: ResMut<Time<Virtual>>) {
    name.set_relative_speed(1.);
}

fn restart(
    mut gs: ResMut<NextState<GameState>>,
    mut ls: ResMut<NextState<LevelState>>,
    mut l: ResMut<NextState<Level>>,
    mut score: ResMut<Score>,
    query: Query<Entity, Or<(With<Transform>, With<Text>)>>,
    mut commands: Commands,
) {
    score.0 = 0;
    NextState::set_if_neq(&mut gs, GameState::GamePlay);
    NextState::set_if_neq(&mut ls, LevelState::Inlevel);
    NextState::set_if_neq(&mut l, Level::Level1);
    for query in query {
        commands.entity(query).try_despawn();
    }
}

fn game_over(mut s: ResMut<NextState<GameState>>) {
    NextState::set_if_neq(&mut s, GameState::GameOver);
}

fn game_over_ui(score: ResMut<score::Score>, mut commands: Commands) {
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
                children![(Text::new("Retry"), RetrybutttonText,)]
            ),
        ],
    ));
}

fn start_de_spawn_mobs(mut commands: Commands, query: Query<(Entity), With<MoveY>>) {
    for query in query {
        commands.entity(query).despawn();
    }
}

fn un_game_over_ui(
    mut mes: MessageWriter<Restart>,
    mut color: Single<&mut TextColor, With<RetrybutttonText>>,
    interaction_query: Query<&Interaction, (With<ReButton>, Changed<Interaction>)>,
) {
    for interaction in interaction_query {
        match interaction {
            Interaction::Pressed => {
                mes.write(Restart);
            }
            Interaction::Hovered => color.0 = Color::srgb(0.5, 0.5, 0.5),
            Interaction::None => color.0 = Color::default(),
        }
    }
}

#[derive(Component)]
struct MoveY(f32);

fn shoot_hit(
    collisions: Collisions,

    mut commands: Commands,
    shoot: Query<(Entity, &Transform), With<Shoot>>,
    mut hos: Query<(
        Entity,
        Has<Asteroid>,
        Has<Mob>,
        Has<ShipPart>,
        Option<&mut MobHealth>,
    )>,
) {
    for shoot in shoot {
        for inpac in collisions.entities_colliding_with(shoot.0) {
            if let Ok((e, asteroid, mob, ship, mut health)) = hos.get_mut(inpac) {
                commands.entity(shoot.0).despawn();
                commands.trigger(HealthEvent {
                    entity: e,
                    value: 1,
                });
                if mob {
                    commands.trigger(score::ScoreEvent(1));
                    // match health {
                    //     Some(ref mut a) => {
                    //         a.0 -= 1;
                    //         if a.0 == 0 {
                    //                     commands.entity(e).despawn();
                    //         };
                    //     }
                    //     None => {
                    //         commands.entity(e).despawn();
                    //     }
                    // }
                }
            }
        }
    }
}

fn despawn(
    query: Query<(&Transform, Entity, Has<Mob>), With<MoveY>>,
    mut commands: Commands,
    p: Single<Entity, With<Pla>>,
) {
    for (y, e, mob) in query {
        if y.translation.y < -540. {
            commands.entity(e).despawn();
            if mob {
                commands.trigger(HealthEvent {
                    entity: *p,
                    value: 1,
                });
            }
        }
        if y.translation.y > 1000. {
            commands.entity(e).despawn();
        }
    }
}
