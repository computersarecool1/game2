use std::f32::consts::PI;
use std::time::Duration;

use bevy::asset::*;
use bevy::color::palettes::css::RED;
use bevy::mesh::*;
use bevy::prelude::*;
use bevy::{ecs::system::command, prelude::TimerMode};

use crate::level3::{self, updatelevel3};
use crate::level4;
use crate::{
    MoveY,
    enemy::Mob,
    level3::{SPAWN_HIGH, ShipPart, WALL_SPACING, move_space_ship, ship_rect},
    mod_level_h::{Level, LevelState},
    pla::{Pla, Shoot},
};

pub(crate) struct Level4plugin;

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

fn x(mut t: Single<(Entity, &mut Transform), With<ShipCore>>) {
    let center = Vec3::new(0., 0., 0.);

    if t.1.translation.xy() != (0., 0.).into() {
        let d = center - t.1.translation;
        t.1.translation += d * 0.04;
    }
    println!("{}", t.1.translation);
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
    img: Res<AssetServer>,
) {
    commands.spawn((
        ShipCore,
        Transform {
            translation: Vec3 {
                x: 0.,
                y: 600.,
                z: 0.,
            },
            ..Default::default()
        },
        Sprite::from_image(img.load("core.png")),
    ));

    let mut asteroid_shaper = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    let mut points = vec![[0., 0., 0.]];
    let mut vecr = vec![[0.5, 0.5]];
    let mut mut_vec = vec![];
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
        mut_vec.push(0);
        mut_vec.push(i);
        mut_vec.push(i - 1);
    }
    mut_vec.push(0);
    mut_vec.push(n_points);
    mut_vec.push(1);
    asteroid_shaper.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    asteroid_shaper.insert_attribute(Mesh::ATTRIBUTE_UV_0, vecr);

    asteroid_shaper.insert_indices(Indices::U32(mut_vec));

    commands.spawn((
        Mesh2d(mesh.add(asteroid_shaper)),
        MeshMaterial2d(mat.add(ColorMaterial::from_color(RED))),
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
