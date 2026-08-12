use crate::{
    MoveY,
    health::{Health, on_despawn_event, on_health_event},
    physics::GameLayer,
};
use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::RigidBody,
};
use bevy::prelude::*;
use bevy::{
    ecs::{bundle::Bundle, component::Component, resource::Resource},
    ui_widgets::observe,
};

#[derive(Resource)]
pub struct ImageHandles {
    pub mob: Handle<Image>,
    pub boss: Handle<Image>,
    pub asteroid: Handle<Image>,
}

impl FromWorld for ImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            mob: asset_server.load("mob.png"),
            boss: asset_server.load("boss.png"),
            asteroid: asset_server.load("asteroid.png"),
        }
    }
}

#[derive(Component)]
pub struct Boss;

impl Boss {
    pub const SIZE: Vec2 = Vec2 { x: 91., y: 91. };

    pub fn bundle(handle: Res<ImageHandles>, y: f32, pos: Vec3, rot: Quat) -> impl Bundle {
        (
            Sprite {
                image: handle.boss.clone(),
                custom_size: Some(Vec2 { x: 141., y: 142. }),
                ..Default::default()
            },
            Boss,
            Mob,
            MoveY(y),
            Hostile,
            Transform::from_translation(pos).with_rotation(rot),
            Health(3),
            observe(on_health_event),
            observe(on_despawn_event),
        )
    }
}

impl Mob {
    pub const SIZE: Vec2 = Vec2 { x: 91., y: 91. };

    pub fn bundle(handle: Res<ImageHandles>, y: f32, pos: Vec3, rot: Quat) -> impl Bundle {
        (
            Sprite {
                image: handle.mob.clone(),
                custom_size: Some(Vec2 { x: 141., y: 142. }),
                ..Default::default()
            },
            Health(1),
            observe(on_health_event),
            observe(on_despawn_event),
            Mob,
            MoveY(y),
            Hostile,
            Transform::from_translation(pos).with_rotation(rot),
        )
    }
}

#[derive(Component)]
#[require(
    Collider::rectangle(Self::SIZE.x,Self::SIZE.y),
    CollisionLayers::new(GameLayer::Mob, [GameLayer::Player,GameLayer::Bullet]),
    RigidBody::Kinematic,

)]
pub struct Mob;

#[derive(Component)]
pub struct MobHealth(pub i32);

#[derive(Component)]
pub struct Hostile;
