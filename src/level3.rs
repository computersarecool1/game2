use avian2d::prelude::*;
use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{
    GameState, Mob, MoveY, Pla,
    enemy::{Boss, ImageHandles},
    mod_level_h::{
        Level::{self},
        LevelState,
    },
    physics::GameLayer,
};
pub const WALL_SPACING: f32 = 300.;
pub const SPAWN_HIGH: f32 = 550.;
pub(crate) struct MyLevel3Plugin;

impl Plugin for MyLevel3Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                (bose, mobs).run_if(in_state(LevelState::Inlevel)),
                updatelevel3,
                y_mobs,
                spawn_space_ship,
                move_space_ship,
                x.run_if(in_state(LevelState::LevelStart)),
            )
                .run_if(in_state(GameState::GamePlay))
                .run_if(in_state(Level::Level3)),
        );
    }
}
#[derive(Component)]
pub struct ShipPart;

pub fn move_space_ship(_commands: Commands, query: Query<&mut Transform, With<ShipPart>>) {
    for mut query in query {
        query.translation.y -= 4.;
    }
}

fn get_spawn(q: Query<(&Transform, &ColliderAabb), With<ShipPart>>) -> (f32, f32) {
    let mut low = -WALL_SPACING + Mob::SIZE.x;
    let mut high = WALL_SPACING - Mob::SIZE.x;
    println!("fornew");
    for s in q {
        if (SPAWN_HIGH).distance(s.0.translation.y) < 250. {
            println!("found");
            if s.0.translation.x < 0. {
                low = low.max(s.1.max.x)
            } else {
                high = high.min(s.1.min.x)
            };
            println!("{} {}", low, high)
        }
    }
    (low, high)
}

pub fn spawn_space_ship(
    mut mat: ResMut<Assets<ColorMaterial>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    query: Query<&mut Transform, With<ShipPart>>,
    mut img: Res<AssetServer>,
) {
    if query.iter().all(|a| a.translation.y < SPAWN_HIGH - 600.) {
        let mut wall_with = 680.;
        commands.spawn(ship_rect(
            &mut mat,
            &mut mesh,
            &mut img,
            Vec3 {
                x: -WALL_SPACING - 0.5 * wall_with,
                y: SPAWN_HIGH + 200.,
                z: 1.,
            },
            wall_with,
            800.,
        ));

        commands.spawn(ship_rect(
            &mut mat,
            &mut mesh,
            &mut img,
            Vec3 {
                x: WALL_SPACING + 0.5 * wall_with,
                y: SPAWN_HIGH + 200.,
                z: 1.,
            },
            wall_with,
            800.,
        ));

        commands.spawn(ship_rect(
            &mut mat,
            &mut mesh,
            &mut img,
            Vec3 {
                x: rand::random_range(300.0..=500.0)
                    * if rand::random_bool(0.5) { 1. } else { -1. },
                y: SPAWN_HIGH + 200.,
                z: 0.,
            },
            700.,
            100.,
        ));
    };
}

pub fn ship_rect(
    mat: &mut ResMut<Assets<ColorMaterial>>,
    mesh: &mut ResMut<Assets<Mesh>>,
    img: &mut Res<AssetServer>,
    vec3: Vec3,
    width: f32,
    hight: f32,
) -> impl Bundle {
    (
        Mesh2d(mesh.add(Rectangle::new(width, hight))),
        MeshMaterial2d(mat.add(img.load("asteroid.png"))),
        ShipPart,
        RigidBody::Kinematic,
        CollisionLayers::new(GameLayer::ShipPart, [GameLayer::Bullet, GameLayer::Player]),
        Collider::rectangle(width, hight),
        Transform::from_translation(vec3),
    )
}

fn bose(
    time: ResMut<Time>,
    mut commands: Commands,
    handle: Res<ImageHandles>,
    q: Query<(&Transform, &ColliderAabb), With<ShipPart>>,
) {
    if time.elapsed_secs() % 13. < time.delta_secs() {
        let (low, high) = get_spawn(q);
        if low > high {
            return;
        }
        commands.spawn(Boss::bundle(
            handle,
            5.,
            Vec3 {
                x: rand::random_range(low..=high),
                y: SPAWN_HIGH,
                z: Default::default(),
            },
            Default::default(),
        ));
    }
}

pub fn updatelevel3(
    main: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut mes_pos: MessageReader<CursorMoved>,
    mut pla_transform: Query<&mut Transform, With<Pla>>,
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

pub(crate) fn y_mobs(mut query: Query<(&mut Transform, &MoveY)>) {
    for (mut y, speed) in &mut query {
        y.translation.y -= speed.0;
    }
}

fn mobs(
    handle: Res<ImageHandles>,
    time: ResMut<Time>,
    mut commands: Commands,
    q: Query<(&Transform, &ColliderAabb), With<ShipPart>>,
) {
    if time.elapsed_secs() % 2. < time.delta_secs() {
        let (low, high) = get_spawn(q);
        if low > high {
            return;
        }
        commands.spawn(Mob::bundle(
            handle,
            4.,
            Vec3 {
                x: rand::random_range(low..=high),
                y: SPAWN_HIGH,
                z: Default::default(),
            },
            Default::default(),
        ));
    }
}

fn x(mut n: ResMut<NextState<LevelState>>, mut t: Single<(Entity, &mut Transform), With<Pla>>) {
    let center = Pla::default_pos();

    if t.1.translation.y != (Pla::default_pos().y) {
        let d = center - t.1.translation;
        t.1.translation.y += d.y * 0.04;
    }

    if 1. >= Pla::default_pos().y.distance(t.1.translation.y) {
        NextState::set_if_neq(&mut n, LevelState::Inlevel);
        t.1.rotation = Quat::default();
    }
}
