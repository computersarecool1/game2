use bevy::prelude::*;

use crate::{
    Asteroid, Boss, GameState, Hostile, Mob, MobHealth, Pla, Shoot, modLevH::{level::{self, level1}, levelState}, movey,
};

pub(crate) struct MyLevel3Plugin;

impl Plugin for MyLevel3Plugin {
    fn build(&self, app: &mut App) {
                
        app.add_systems(
            FixedUpdate,
            (
                
                (update,asteroid, bose, mobs).run_if(in_state(levelState::Inlevel)),
                y_mobs,
                shoot,
                x.run_if(in_state(levelState::levelStart)),
            )
                .run_if(in_state(GameState::GamePlay))
                .run_if(in_state(level::level3)),
        );
    }
}

fn bose(
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    if (time.elapsed_secs() % 13. < time.delta_secs()) {
        commands.spawn((
            Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
            MeshMaterial2d(mat.add(Color::srgb(0.72_f32, 0.222_f32, 0.2_f32))),
            Boss,
            MobHealth(3),
            Mob,
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

fn asteroid(
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    if (time.elapsed_secs() % 1. < time.delta_secs()) {
        commands.spawn((
            Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
            MeshMaterial2d(mat.add(Color::srgb(22_f32, 92_f32, 22_f32))),
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
                y: 600.,
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
fn x(mut n: ResMut<NextState<levelState>>, mut t: Single<(Entity, &mut Transform), With<Pla>>) {
    let center = Pla::default_pos();

    if t.1.translation.xy() != (Pla::default_pos().xy()) {
        println!("dddddddd");
        let d = center - t.1.translation;
        t.1.translation += d * 0.04;
    }

    if 1. >= Pla::default_pos().xy().distance(t.1.translation.xy()) {
        NextState::set_if_neq(&mut n, levelState::Inlevel);
        t.1.rotation = Quat::default();
    }
}
