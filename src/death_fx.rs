use core::f32;

use avian2d::{
    dynamics::rigid_body::{LinearDamping, LinearVelocity, RigidBody, sleeping::SleepingDisabled},
    parry::math::ComplexField,
    schedule::PhysicsStepSystems::Sleeping,
};
use bevy::{ecs::system::command, prelude::*, sprite_render::ColorMaterialFlags};
use rand::seq::IteratorRandom;

use crate::{GameOver, pla::Pla};

pub(crate) struct Death_fx;

impl Plugin for Death_fx {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (spawn_enem_death_fx, spawn_wall_death_fx).run_if(on_message::<GameOver>),
        );
    }
}
fn spawn_enem_death_fx(
    mut pla: Single<&Transform, With<Pla>>,
    mut mes: MessageReader<GameOver>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for mes in mes.read() {
        let shapes: Vec<_> = (3..10)
            .map(|x| {
                mesh.add(RegularPolygon::new(
                    rand::random_range(0.3..=1.5).powf(3.0),
                    (x),
                ))
            })
            .collect();
            todo!(not now)
        let mut rng = &mut rand::rng();
        let matt = mat.add(Color::srgba(255., 255., 255., 100.));
        for _ in 0..22225 {
            let speed = rand::random_range(0.1..=15.0) * 30.;
            let angle = rand::random_range(0.0..=std::f64::consts::TAU);
            commands.spawn((
                Mesh2d(shapes.iter().choose(rng).unwrap().clone()),
                MeshMaterial2d(matt.clone()),
                (*pla).clone(),
                RigidBody::Kinematic,
                LinearVelocity(Vec2 {
                    x: (angle.cos() * speed * rand::random_range(0.3..=1.5).powf(3.0)) as f32,
                    y: (angle.sin() * speed * rand::random_range(0.5..=1.5).powf(7.0)) as f32,
                }),
                LinearDamping(0.0),
                SleepingDisabled,
            ));
        }
    }
}

fn spawn_wall_death_fx() {}
