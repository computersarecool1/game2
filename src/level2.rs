use std::f32::consts::PI;

use bevy::{log::Level, prelude::*};

use crate::{
    Boss, GameState, Hostile, Mob, MobHealth, Pla, Shoot,
    modLevH::{level, levelState},
    movey,
};

pub(crate) struct level2Plugin;

impl Plugin for level2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (((
                x.run_if(in_state(levelState::levelStart)),
                rotate,(
                bose,
                mobs,).run_if(in_state(levelState::Inlevel)),
                shoot,
                y_mobs,
            )
                .run_if(in_state(level::level2)),)),
        );
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
            Transform {
                translation: query.translation,
                rotation: query.rotation,
                ..Default::default()
            },
        ));
    }
}

fn bose(
    mut t: Single<(Entity, &mut Transform), With<Pla>>,
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    let pos =
        Quat::from_rotation_z(rand::random_range(PI..=PI * 2.)).mul_vec3(Vec3::new(0., 500., 0.));
    let rotate_to_pla = Quat::from_rotation_arc(Vec3::Y, (t.1.translation - pos).normalize());
    if (time.elapsed_secs() % 13. < time.delta_secs()) {
        commands.spawn((
            Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
            MeshMaterial2d(mat.add(Color::srgb(0.52_f32, 0.222_f32, 0.2_f32))),
            Boss,
            MobHealth(3),
            Mob,
            movey(-5.),
            Hostile,
            Transform {
                translation: pos,
                rotation: rotate_to_pla,
                ..Default::default()
            },
        ));
    }
}
fn mobs(
    time: ResMut<Time>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
    mut t: Single<(Entity, &mut Transform), With<Pla>>,
) {
    if (time.elapsed_secs() % 2. < time.delta_secs()) {
        let pos = Quat::from_rotation_z(rand::random_range(PI..=PI * 2.))
            .mul_vec3(Vec3::new(0., 500., 0.));
        let rotate_to_pla = Quat::from_rotation_arc(Vec3::Y, (t.1.translation - pos).normalize());
        commands.spawn((
            Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
            MeshMaterial2d(mat.add(Color::srgb(255_f32, 0_f32, 0_f32))),
            Mob,
            movey(-4.),
            Hostile,
            Transform {
                translation: pos,
                rotation: rotate_to_pla,
                ..Default::default()
            },
        ));
    }
}
fn y_mobs(mut query: Query<(&mut Transform, &movey)>) {
    for (mut y, speed) in &mut query {
        let m = y.rotation.mul_vec3(Vec3::new(0., speed.0, 0.));
        y.translation -= m;
    }
}

fn rotate(
    mut t: Single<(Entity, &mut Transform), With<Pla>>,
    mut mes_pos: MessageReader<CursorMoved>,
    main: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, cam_transform) = main.into_inner();

    for mes in mes_pos.read() {
        if let Ok(pos) = camera.viewport_to_world_2d(cam_transform, mes.position) {
            let to_mouse = (pos - t.1.translation.xy()).normalize();
            let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));
            t.1.rotation = rotate_to_mouse;
        }
    }
}
fn x(mut n: ResMut<NextState<levelState>>, mut t: Single<(Entity, &mut Transform), With<Pla>>) {
    let center = Vec3::new(0., 0., 0.);

    if t.1.translation.xy() != (0., 0.).into() {
        println!("dddddddd");
        let d = center - t.1.translation;
        t.1.translation += d * 0.04;
    }

    if t.1.translation.xy().as_ivec2() == IVec2::ZERO {
        NextState::set_if_neq(&mut n, levelState::Inlevel);
    }
}
