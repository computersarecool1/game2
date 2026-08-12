use bevy::prelude::*;

use crate::{
    MoveY,
    level3::move_space_ship,
    mod_level_h::{Level, LevelState},
    pla::{Pla, Shoot},
};

pub(crate) struct Level4plugin;

impl Plugin for Level4plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (orbit_system, move_space_ship, trans, y_mobs, orbit_b)
                .run_if(in_state(LevelState::LevelStart)),
        );
        app.add_systems(
            FixedUpdate,
            (move_pla, boss_bar).run_if(in_state(LevelState::Inlevel)),
        );
        app.add_systems(
            OnEnter(Level::Level4),
            (orbit_pla.after(spawn_core), spawn_core),
        );
    }
}

#[derive(Component)]
struct ShipCore;

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

fn spawn_core(mut commands: Commands, img: Res<AssetServer>) {
    commands.spawn((
        ShipCore,
        Transform {
            ..Default::default()
        },
        Sprite::from_image(img.load("core.png")),
    ));
}

fn trans() {}

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
        let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));
        transform.rotation = rotate_to_mouse;
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
            angular_speed: 2.1,
            min_radius: 50.,
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
        angular_speed: 2.1,
        min_radius: 50.,
        max_radius: 200.,
    });
}

fn boss_bar() {}

// fn spawn_at_pipe() {}

// fn win_scene // Observer

// fn win_end_sceen // Observer
