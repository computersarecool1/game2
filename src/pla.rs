use crate::{
    MoveY,
    health::{DespawnEvent, Health, on_health_event},
    physics::GameLayer,
};
use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::RigidBody,
};
use bevy::{
    asset::Assets,
    camera::Camera2d,
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::{Vec3, primitives::Rectangle},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};
use bevy::{prelude::*, ui_widgets::observe};

impl Pla {
    pub fn default_pos() -> Vec3 {
        Vec3 {
            x: Default::default(),
            y: -333.,
            z: Default::default(),
        }
    }
}

#[derive(Component)]
#[require(Collider::rectangle(71., 71.))]
pub struct Pla;

#[derive(Component)]
#[require(
    Collider::rectangle(71., 71.),
    CollisionLayers::new(GameLayer::Bullet, [GameLayer::ShipPart,GameLayer::Mob,GameLayer::Asteroid]),
    RigidBody::Kinematic,

)]
pub struct Shoot;

pub fn run_pla_die(mut commands: Commands, pla: Single<(Entity), With<Pla>>) {
    commands.entity(*pla).despawn();
}
pub fn start(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
        MeshMaterial2d(mat.add(Color::srgb(0_f32, 0_f32, 255_f32))),
        Pla,
        RigidBody::Kinematic,
        CollisionLayers::new(
            GameLayer::Player,
            [GameLayer::Mob, GameLayer::Asteroid, GameLayer::ShipPart],
        ),
        Transform::from_translation(Pla::default_pos()),
        Health(3),
        observe(on_unhealth_event),
        observe(on_health_event),
    ));
}
fn on_unhealth_event(event: On<DespawnEvent>, mut s: ResMut<NextState<crate::GameState>>) {
    NextState::set_if_neq(&mut s, crate::GameState::GameOver);
}
pub fn shoot(
    mut commands: Commands,
    query: Single<&Transform, With<Pla>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<ColorMaterial>>,
    click: Res<ButtonInput<MouseButton>>,
) {
    if click.just_pressed(MouseButton::Left) {
        commands.spawn((
            Mesh2d(mesh.add(Rectangle::new(61_f32, 62_f32))),
            MeshMaterial2d(mat.add(Color::srgb(0_f32, 0_f32, 255_f32))),
            MoveY(-7.),
            Shoot,
            Transform {
                translation: query.translation,
                rotation: query.rotation,
                ..Default::default()
            },
        ));
    }
}
