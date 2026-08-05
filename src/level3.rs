use avian2d::prelude::*;
use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{
    Asteroid, GameState, Hostile, Mob, MobHealth, Pla, Shoot,
    enemmey::{Boss, ImageHandles, Mobhandle},
    modLevH::{
        level::{self, level1},
        levelState,
    },
    movey,
    physics::GameLayer,
};
const WALL_SPACING: f32 = 300.;
const Spawn_high: f32 = 250.;
pub(crate) struct MyLevel3Plugin;

impl Plugin for MyLevel3Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(level::level3), spawnSideWalls);
        app.add_systems(
            FixedUpdate,
            (
                y_mobs,
                (
                    (spawnSpaseShip, moveSpaseShip, updateSideWalls),
                    (update, bose, mobs).run_if(in_state(levelState::Inlevel)),
                )
                    .chain(),
                x.run_if(in_state(levelState::levelStart)),
            )
                .run_if(in_state(GameState::GamePlay))
                .run_if(in_state(level::level3)),
        );
    }
}
#[derive(Component)]
pub struct shipPart;

fn moveSpaseShip(commands: Commands, mut query: Query<&mut Transform, With<shipPart>>) {
    for mut query in query {
        query.translation.y -= 4.;
    }
}

fn get_spawn(mut q: Query<(&Transform, &ColliderAabb), With<shipPart>>) -> (f32, f32) {
    let mut low = -WALL_SPACING + Mob::SIZE.x;
    let mut high = WALL_SPACING - Mob::SIZE.x;
    println!("fornew");
    for s in q {
        if (Spawn_high).distance(s.0.translation.y) < 250. {
            println!("found");
            if s.0.translation.x < 0. {
                low = low.max(s.1.max.x)
            } else {
                high = high.min(s.1.min.x)
            };
            println!("{} {}", low, high)
        }
    }
    return (low, high);
}

#[derive(Component)]
struct SideWall;

fn spawnSideWalls(
    mut mat: ResMut<Assets<ColorMaterial>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut img: Res<AssetServer>,
) {
    // Left wall
    commands.spawn((
        SideWall,
        shipRect(
            &mut mat,
            &mut mesh,
            &mut img,
            -WALL_SPACING * 1.5,
            Spawn_high + 100.,
            400.,
            3000.,
        ),
    ));
    // Right wall
    commands.spawn((
        SideWall,
        shipRect(
            &mut mat,
            &mut mesh,
            &mut img,
            WALL_SPACING * 1.5,
            Spawn_high + 100.,
            400.,
            3000.,
        ),
    ));
}

fn updateSideWalls(mut query: Query<&mut Transform, With<SideWall>>) {
    for mut transform in &mut query {
        if transform.translation.y < Spawn_high {
            transform.translation.y += Spawn_high;
        }
    }
}

fn spawnSpaseShip(
    mut mat: ResMut<Assets<ColorMaterial>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut query: Query<&mut Transform, With<shipPart>>,
    mut img: Res<AssetServer>,
) {
    if query.iter().all(|a| a.translation.y < Spawn_high) {
        commands.spawn(
            (shipRect(
                &mut mat,
                &mut mesh,
                &mut img,
                rand::random_range(300.0..=500.0) * if rand::random_bool(0.5) { 1. } else { -1. },
                Spawn_high + 10. * Mob::SIZE.y,
                700.,
                100.,
            )),
        );
    };
}

fn shipRect(
    mut mat: &mut ResMut<Assets<ColorMaterial>>,
    mut mesh: &mut ResMut<Assets<Mesh>>,
    mut img: &mut Res<AssetServer>,
    x: f32,
    y: f32,
    width: f32,
    hight: f32,
) -> impl Bundle {
    (
        Mesh2d(mesh.add(Rectangle::new(width, hight))),
        MeshMaterial2d(mat.add(img.load("asteroid.png"))),
        shipPart,
        RigidBody::Kinematic,
        CollisionLayers::new(GameLayer::ShipPart, [GameLayer::Bullet, GameLayer::Player]),
        Collider::rectangle(width, hight),
        Transform::from_translation(Vec3 {
            x: x,
            y: y,
            z: Default::default(),
        }),
    )
}
fn bose(
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
    handle: Res<ImageHandles>,
    mut q: Query<(&Transform, &ColliderAabb), With<shipPart>>,
) {
    if (time.elapsed_secs() % 13. < time.delta_secs()) {
        let (low, high) = get_spawn(q);
        if low > high {
            return;
        }
        commands.spawn(Boss::bundle(
            handle,
            5.,
            Vec3 {
                x: rand::random_range(low..=high),
                y: Spawn_high,
                z: Default::default(),
            },
            Default::default(),
        ));
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
fn y_mobs(mut query: Query<(&mut Transform, &movey)>) {
    for (mut y, speed) in &mut query {
        y.translation.y -= speed.0;
    }
}
fn mobs(
    handle: Res<ImageHandles>,
    time: ResMut<Time>,
    mut commands: Commands,
    mut asset: ResMut<AssetServer>,
    mut q: Query<(&Transform, &ColliderAabb), With<shipPart>>,
) {
    if (time.elapsed_secs() % 2. < time.delta_secs()) {
        let (low, high) = get_spawn(q);
        if low > high {
            return;
        }
        commands.spawn(Mob::bundle(
            handle,
            4.,
            Vec3 {
                x: rand::random_range(low..=high),
                y: Spawn_high,
                z: Default::default(),
            },
            Default::default(),
        ));
    }
}

fn x(mut n: ResMut<NextState<levelState>>, mut t: Single<(Entity, &mut Transform), With<Pla>>) {
    let center = Pla::default_pos();

    if t.1.translation.xy() != (Pla::default_pos().xy()) {
        let d = center - t.1.translation;
        t.1.translation += d * 0.04;
    }

    if 1. >= Pla::default_pos().xy().distance(t.1.translation.xy()) {
        NextState::set_if_neq(&mut n, levelState::Inlevel);
        t.1.rotation = Quat::default();
    }
}
