use std::thread::spawn;

use bevy::prelude::*;

use crate::{
    level3::{moveSpaseShip, updatelevel3},
    modLevH::{level, levelState},
    pla::Pla,
};

pub(crate) struct level4Plugin;

impl Plugin for level4Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (orbit_pla, orbit_system, moveSpaseShip, trans)
                .run_if(in_state(levelState::levelStart)),
        );
        app.add_systems(
            FixedUpdate,
            (move_pla, boss_bar).run_if(in_state(levelState::Inlevel)),
        );
        app.add_systems(OnEnter(level::level4), spawn_core);
    }
}
#[derive(Component)]
struct ship_core;

fn spawn_core(mut commands: Commands, img: Res<AssetServer>) {
    commands.spawn((
        ship_core,
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

        orbit.angle += orbit.angular_speed * time.delta_secs();

        let delta = Vec2::new(orbit.angle.cos(), orbit.angle.sin()) * orbit.radius;

        transform.translation = target_transform.translation + delta.extend(0.0);
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
fn move_pla() {}
fn orbit_pla(
    mut commands: Commands,
    pla: Single<Entity, With<Pla>>,
    core: Single<Entity, With<ship_core>>,
) {
    commands.entity(*pla).insert(Orbit {
        target: *core,
        radius: 100.,
        angle: 0.,
        angular_speed: 500.1,
        min_radius: 50.,
        max_radius: 200.,
    });
}
fn boss_bar() {}

// fn spawn_at_pipe() {}

// fn win_scene // Observer

// fn win_end_sceen // Observer
