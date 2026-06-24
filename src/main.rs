use bevy::{
    ecs::query, log::tracing_subscriber::fmt::format, math::NormedVectorSpace, prelude::*,
    transform::commands,
};
mod health;
mod score;

use crate::GameState::GamePlay;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((score::ScorePlugin, health::HealthPlugin))
        .add_systems(Startup, (start).chain())
        .add_systems(OnEnter(GameState::GameOver), game_over_ui)
        .add_systems(
            OnTransition {
                entered: GameState::GamePlay,
                exited: GameState::GameOver,
            },
            start_deSpawnMobs,
        )
        .add_systems(
            FixedUpdate,
            (
                (
                    update,
                    mobs,
                    shootHit,
                    y_mobs,
                    despanw,
                    shoot,
                    hit,
                    asteroid,
                    game_over.run_if(on_message::<Hit>),
                )
                    .run_if(in_state(GameState::GamePlay)),
                (un_game_over_ui).run_if(in_state(GameState::GameOver)),
            ),
        )
        .add_message::<Hit>()
        .init_state::<GameState>()
        .run();
}

#[derive(Component)]

struct Asteroid;

fn asteroid(
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    if (time.elapsed_secs() % 1. < time.delta_secs()) {
        commands.spawn((
            Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
            MeshMaterial2d(mat.add(Color::srgb(22_f32, 22_f32, 22_f32))),
            Asteroid,
            movey(5.),
            Hostile,
            Transform::from_translation(Vec3 {
                x: rand::random_range(-600.0..=600.0),
                y: 23.,
                z: Default::default(),
            }),
        ));
    }
}

fn shoot(
    mut commands: Commands,
    query: Single<(&Transform), With<Pla>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
    mut click: Res<ButtonInput<MouseButton>>,
) {
    if click.just_pressed(MouseButton::Left) {
        commands.spawn((
            Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
            MeshMaterial2d(mat.add(Color::srgb(0_f32, 0_f32, 255_f32))),
            movey(-7.),
            Shoot,
            Transform::from_translation(Vec3 {
                x: query.translation.x,
                y: -333.,
                z: Default::default(),
            }),
        ));
    }
}
#[derive(Component)]

struct Mob;
#[derive(Resource, Component)]
struct P {
    x: i32,
    y: i32,
}
#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
enum GameState {
    GameOver,
    #[default]
    GamePlay,
}
#[derive(Event, Message)]
struct Hit {
    hit: Entity,
}
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
    mut color: Single<&mut TextColor, With<retrybutttonText>>,
    mut interaction_query: Query<(Entity, &Interaction), (With<ReButton>, Changed<Interaction>)>,
) {
    for (e, interaction) in interaction_query {
        match interaction {
            Interaction::Pressed => NextState::set_if_neq(&mut s, GameState::GamePlay),
            Interaction::Hovered => color.0 = Color::srgb(0.5, 0.5, 0.5),
            Interaction::None => color.0 = Color::default(),

            _ => {}
        }
    }
}

#[derive(Component)]
struct movey(f32);

#[derive(Component)]

struct Pla;
fn start(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
        MeshMaterial2d(mat.add(Color::srgb(0_f32, 0_f32, 255_f32))),
        Pla,
        Transform::from_translation(Vec3 {
            x: Default::default(),
            y: -333.,
            z: Default::default(),
        }),
    ));
}

#[derive(Component)]

struct Shoot;
fn shootHit(
    mut commands: Commands,
    shoot: Query<(Entity, &Transform), With<Shoot>>,
    hos: Query<(Entity, &Transform, Has<Asteroid>, Has<Mob>), With<Hostile>>,
) {
    for (mobEntity, hos, asteroid, mob) in hos {
        for shoot in shoot {
            if hos.translation.xy().distance(shoot.1.translation.xy()) <= 61. {
                if asteroid {
                    commands.entity(shoot.0).despawn();
                };
                if mob {
                    commands.trigger(score::ScoreEvent(1));
                    commands.entity(mobEntity).despawn();
                    commands.entity(shoot.0).despawn();
                }
            }
        }
    }
}

#[derive(Component)]

struct Hostile;
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
fn update(
    main: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut mes_pos: MessageReader<CursorMoved>,
    mut pla_transform: Query<(&mut Transform), With<Pla>>,
) {
    let (camera, cam_transform) = main.into_inner();
    for mes in mes_pos.read() {
        for mut pla_transform in &mut pla_transform {
            if let Ok(pos) = camera.viewport_to_world_2d(cam_transform, mes.position) {
                pla_transform.translation.x = pos.x;
            }
        }
    }
}

fn mobs(
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    if (time.elapsed_secs() % 2. < time.delta_secs()) {
        commands.spawn((
            Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
            MeshMaterial2d(mat.add(Color::srgb(255_f32, 0_f32, 0_f32))),
            Mob,
            movey(4.),
            Hostile,
            Transform::from_translation(Vec3 {
                x: rand::random_range(-600.0..=600.0),
                y: 23.,
                z: Default::default(),
            }),
        ));
    }
}

fn y_mobs(mut query: Query<(&mut Transform, &movey)>) {
    for (mut y, speed) in &mut query {
        y.translation.y -= speed.0;
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
