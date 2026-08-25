use std::f32::consts::PI;
use std::time::Duration;

use crate::enemy::ImageHandles;
use crate::level3::{self, ROCK, SHIP_SPEED, updatelevel3};
use crate::level4;
use crate::{
    MoveY,
    enemy::Mob,
    level3::{SPAWN_HIGH, ShipPart, WALL_SPACING, move_space_ship, ship_rect},
    mod_level_h::{Level, LevelState},
    pla::{Pla, Shoot},
};
use bevy::asset::*;
use bevy::color::palettes::css::RED;
use bevy::mesh::*;
use bevy::prelude::*;
use bevy::{ecs::system::command, prelude::TimerMode};
use noisy_bevy::fbm_simplex_2d;

pub(crate) struct Level4plugin;

const SPAWN_OFF: f32 = 1400.;
impl Plugin for Level4plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Level::Level4), trans);
        app.add_systems(
            FixedUpdate,
            (updatelevel3, move_space_ship, y_mobs, transToStart).run_if(in_state(Level::Level4)),
        )
        .add_systems(
            OnEnter(LevelState::Inlevel),
            orbit_pla.after(spawn_core).run_if(in_state(Level::Level4)),
        );
        app.add_systems(
            FixedUpdate,
            (x, level3::y_mobs)
                .run_if(in_state(Level::Level4))
                .run_if(in_state(LevelState::LevelStart)),
        );
        app.add_systems(
            FixedUpdate,
            (move_pla_to_orbit, orbit_b, orbit_system, move_pla, boss_bar)
                .run_if(in_state(LevelState::Inlevel))
                .run_if(in_state(Level::Level4)),
        );
        app.add_systems(
            OnEnter(Level::Level4),
            (spawn_core).run_if(in_state(Level::Level4)),
        );
    }
}
pub fn not_ob_spawn_space_ship(
    mut mat: ResMut<Assets<ColorMaterial>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    query: Query<&mut Transform, With<ShipPart>>,
    mut img: Res<AssetServer>,
) {
    if query.iter().all(|a| a.translation.y < SPAWN_HIGH) {
        commands.spawn(ship_rect(
            &mut mat,
            &mut mesh,
            &mut img,
            Vec3 {
                x: -WALL_SPACING,
                y: SPAWN_HIGH + 200.,
                z: 1.,
            },
            61.,
            10. * Mob::SIZE.x,
        ));

        commands.spawn(ship_rect(
            &mut mat,
            &mut mesh,
            &mut img,
            Vec3 {
                x: WALL_SPACING,
                y: SPAWN_HIGH + 200.,
                z: 1.,
            },
            61.,
            10. * Mob::SIZE.x,
        ));
    }
}
#[derive(Component)]
struct ShipCore;

fn move_pla_to_orbit(mut t: Single<(Entity, &mut Orbit), With<Pla>>) {
    let finealspeed = 3.;

    if t.1.angular_speed <= finealspeed {
        t.1.angular_speed += 0.001;
    }
}
#[derive(Component)]
struct ShipChamber;
fn x(time: Res<Time>, mut t: Query<(Entity, &mut Transform), With<ShipChamber>>) {
    for mut t in t {
        let center = Vec3::new(0., 0., 0.);

        if t.1.translation.xy() != (0., 0.).into() {
            let d = center - t.1.translation;
            t.1.translation += d.normalize() * time.delta_secs() * d.length().clamp(0., SHIP_SPEED);
        }
        println!("{}", t.1.translation);
    }
}
#[derive(Component)]
struct Orbit {
    target: Entity,
    radius: f32,
    angle: f32,
    angular_speed: f32,
    min_radius: f32,
    max_radius: f32,
}

fn y_mobs(mut query: Query<(&mut Orbit, &MoveY)>) {
    for (mut y, speed) in &mut query {
        let m = speed.0;
        y.radius += m;
    }
}

fn spawn_core(
    mut mat: ResMut<Assets<ColorMaterial>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    handle: Res<ImageHandles>,

    img: Res<AssetServer>,
) {
    commands.spawn((
        ShipCore,
        ShipChamber,
        Transform {
            translation: Vec3 {
                x: 0.,
                y: SPAWN_OFF,
                z: 0.,
            },
            ..Default::default()
        },
        Sprite::from_image(img.load("core.png")),
    ));

    let mut asteroid_shaper = Mesh::new(
        PrimitiveTopology::TriangleStrip,
        RenderAssetUsages::RENDER_WORLD,
    );
    let r = 500.;
    let mut points = vec![[-WALL_SPACING / 2., r, 0.]];
    let mut vecr = vec![[0.5, 1.]];
    let mut mut_vec = vec![];
    let sve = -WALL_SPACING / 2.;
    let sve2 = WALL_SPACING / 2.;

    let angle_gap = ((WALL_SPACING) / r);
    let n_points = 200;
    let scale = 0.7;
    for i in 0..n_points {
        let angle =
            (2. * PI - angle_gap) / (n_points as f32) * i as f32 - (PI / 2.) + angle_gap / 2.;
        let offset = fbm_simplex_2d(
            Vec2::new(angle.cos() * scale, angle.sin() * scale),
            3,
            3.,
            0.5,
        );
        let distance = r + offset * 50.;

        points.push([
            (distance + 1500.) * angle.cos(),
            (distance + 1500.) * angle.sin(),
            0.,
        ]);
        points.push([(distance) * angle.cos(), (distance) * angle.sin(), 0.]);
        vecr.push([
            (ROCK + 1.5) * angle.cos() + 0.5,
            (ROCK + 1.5) * angle.sin() + 0.5,
        ]);
        vecr.push([
            (ROCK + 0.2 + offset * 0.05 * ROCK) * angle.cos() + 0.5,
            (ROCK + 0.2 + offset * 0.05 * ROCK) * angle.sin() + 0.5,
        ]);
    }
    vecr.push([0.5, 1.]);

    vecr.push([0.5, 1.]);

    points.push([WALL_SPACING / 2., -r, 0.]);
    points.push([WALL_SPACING / 2., -r - 27., 0.]);

    for i in 0..(2 * n_points + 1) {
        mut_vec.push(i);
    }

    asteroid_shaper.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    asteroid_shaper.insert_attribute(Mesh::ATTRIBUTE_UV_0, vecr);

    asteroid_shaper.insert_indices(Indices::U32(mut_vec));

    commands.spawn((
        Mesh2d(mesh.add(asteroid_shaper)),
        ShipChamber,
        Transform {
            translation: Vec3 {
                x: 0.,
                y: SPAWN_OFF,
                z: 1.,
            },
            ..Default::default()
        },
        MeshMaterial2d(mat.add(handle.asteroid.clone())),
    ));
}
#[derive(Resource)]
struct eTime(Timer);

fn trans(mut commands: Commands) {
    commands.insert_resource(eTime(Timer::new(Duration::from_secs(9), TimerMode::Once)));
}
fn transToStart(
    time: ResMut<Time>,
    mut nextstate: ResMut<NextState<LevelState>>,
    mut commands: Commands,
    mut etime: ResMut<eTime>,
) {
    etime.0.tick(time.delta());
    if etime.0.is_finished() {
        NextState::set_if_neq(&mut nextstate, LevelState::Inlevel);
    }
}
fn orbit_system(
    time: Res<Time>,
    mut orbiters: Query<(&mut Transform, &mut Orbit)>,
    transforms: Query<&Transform, Without<Orbit>>,
) {
    for (mut transform, mut orbit) in &mut orbiters {
        let Ok(target_transform) = transforms.get(orbit.target) else {
            continue;
        };
        println!("{:?}", orbit.angle);

        orbit.angle += orbit.angular_speed * time.delta_secs();
        println!("{:?}", orbit.angle);

        let delta = Vec2::new(orbit.angle.cos(), orbit.angle.sin()) * orbit.radius;
        let to_mouse = (target_transform.translation.xy() - transform.translation.xy()).normalize();
        let mut playerforword = (transform.rotation * Vec3::Y).xy();
        let mut dot = playerforword.dot(to_mouse);
        let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));

        let enemy_right = (transform.rotation * Vec3::X).xy();

        let right_dot_player = enemy_right.dot(to_mouse);

        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        let max_angle = ops::acos(dot.clamp(-1.0, 1.0));

        let rotation_angle = rotation_sign
            * (f32::to_radians((4. * orbit.angular_speed).powf(2.) * 25.) * time.delta_secs())
                .min(max_angle);

        transform.rotate_z(rotation_angle);

        // transform.rotation = rotate_to_mouse;
        transform.translation = target_transform.translation + delta.extend(0.0);
    }
}

fn move_pla() {}

fn orbit_b(
    core: Single<(&Transform, Entity), With<ShipCore>>,
    mut commands: Commands,
    a: Query<(&Transform, Entity), (With<Shoot>, Without<Orbit>)>,
) {
    for b in a {
        let direction = b.0.translation - core.0.translation;
        let angle = direction.y.atan2(direction.x);
        let radius = direction.length();
        commands.entity(b.1).insert(Orbit {
            target: core.1,
            radius,
            angle,
            angular_speed: 0.1,
            min_radius: 0.,
            max_radius: 200.,
        });
    }
}
fn orbit_pla(
    mut commands: Commands,
    pla: Single<Entity, With<Pla>>,
    core: Single<Entity, With<ShipCore>>,
    query: Query<&Transform>,
) {
    let player_pos = query.get(*pla).unwrap().translation;
    let core_pos = query.get(*core).unwrap().translation;

    let direction = player_pos - core_pos;
    let angle = direction.y.atan2(direction.x);
    let radius = direction.length();

    commands.entity(*pla).insert(Orbit {
        target: *core,
        radius,
        angle,
        angular_speed: 0.0,
        min_radius: 0.,
        max_radius: 200.,
    });
}

fn boss_bar() {}

// fn spawn_at_pipe() {}

// fn win_scene // Observer

// fn win_end_sceen // Observer
