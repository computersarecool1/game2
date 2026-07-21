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
        entity::Entity,
        event::Event,
        message::Message,
        system::{Commands, ResMut},
    },
    math::{Vec3, primitives::Rectangle},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::physics::GameLayer;

#[derive(Event, Message)]
pub struct Hit {
    pub(crate) hit: Entity,
}

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
    ));
}

#[derive(Component)]
#[require(
    Collider::rectangle(71., 71.),
    CollisionLayers::new(GameLayer::Bullet, [GameLayer::ShipPart,GameLayer::Mob,GameLayer::Asteroid]),
    RigidBody::Kinematic,

)]
pub struct Shoot;
