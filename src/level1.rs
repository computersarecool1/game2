use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::RigidBody,
    math::PI,
};
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::GREY,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{GameState, enemmey::*, modLevH::*, movey, physics::GameLayer, pla::*};

pub(crate) struct MyLevel1Plugin;

impl Plugin for MyLevel1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update,
                (asteroid, bose, mobs).run_if(in_state(levelState::Inlevel)),
                y_mobs,
            )
                .run_if(in_state(GameState::GamePlay))
                .run_if(in_state(level::level1)),
        );
    }
}

fn bose(time: ResMut<Time>, mut commands: Commands, handle: Res<ImageHandles>) {
    if (time.elapsed_secs() % 13. < time.delta_secs()) {
        commands.spawn(Boss::bundle(
            handle,
            5.,
            Vec3 {
                x: rand::random_range(-600.0..=600.0),
                y: 600.,
                z: Default::default(),
            },
            Default::default(),
        ));
    }
}

fn asteroid(
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    handle: Res<ImageHandles>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    let mut asteroid_shaper = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    let mut points = vec![[0., 0., 0.]];
    let mut vecr = vec![[0.5, 0.5]];
    let mut mutVec = vec![];
    let n_points = 12;
    for i in 0..n_points {
        let angle = 2. * PI / (n_points as f32) * i as f32;
        let distance = rand::random_range(20.0..=35.0);
        points.push([distance * angle.cos(), distance * angle.sin(), 0.]);
        vecr.push([0.5 * angle.cos() + 0.5, 0.5 * angle.sin() + 0.5]);
    }
    vecr = vecr
        .into_iter()
        .map(|f| {
            [
                f[0] + rand::random_range(-0.2..=0.2),
                f[1] + rand::random_range(-0.2..=0.2),
            ]
        })
        .collect();
    for i in 2..(n_points + 1) {
        mutVec.push(0);
        mutVec.push(i);
        mutVec.push(i - 1);
    }
    mutVec.push(0);
    mutVec.push(n_points);
    mutVec.push(1);
    asteroid_shaper.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    asteroid_shaper.insert_attribute(Mesh::ATTRIBUTE_UV_0, vecr);

    asteroid_shaper.insert_indices(Indices::U32(mutVec));

    if (time.elapsed_secs() % 1. < time.delta_secs()) {
        commands.spawn((
            Mesh2d(mesh.add(asteroid_shaper)),
            MeshMaterial2d(mat.add(handle.asteroid.clone())),
            Asteroid,
            movey(5.),
            Hostile,
            Transform::from_translation(Vec3 {
                x: rand::random_range(-600.0..=600.0),
                y: 600.,
                z: Default::default(),
            }),
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
#[derive(Component)]
#[require(Collider::rectangle(71., 71.), CollisionLayers::new(GameLayer::Asteroid, [GameLayer::ShipPart,GameLayer::Player,GameLayer::Bullet]),RigidBody::Kinematic)]
pub struct Asteroid;

fn y_mobs(mut query: Query<(&mut Transform, &movey)>) {
    for (mut y, speed) in &mut query {
        y.translation.y -= speed.0;
    }
}
fn mobs(
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
    handle: Res<ImageHandles>,
) {
    if (time.elapsed_secs() % 2. < time.delta_secs()) {
        commands.spawn(Mob::bundle(
            handle,
            4.,
            Vec3 {
                x: rand::random_range(-600.0..=600.0),
                y: 600.,
                z: Default::default(),
            },
            Default::default(),
        ));
    }
}
