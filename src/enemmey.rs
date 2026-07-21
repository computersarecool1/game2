use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::RigidBody,
};
use bevy::ecs::{component::Component, resource::Resource};

use crate::physics::GameLayer;

#[derive(Component)]

pub struct Boss;

#[derive(Component)]
#[require(
    Collider::rectangle(71., 71.),
    CollisionLayers::new(GameLayer::Mob, [GameLayer::Player,GameLayer::Bullet]),
    RigidBody::Kinematic,

)]

pub struct Mob;
#[derive(Resource, Component)]
pub struct P {
    x: i32,
    y: i32,
}

#[derive(Component)]

pub struct MobHealth(pub i32);

#[derive(Component)]

pub struct Hostile;
