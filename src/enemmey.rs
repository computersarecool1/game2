use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::RigidBody,
};
use bevy::prelude::*;
use bevy::{
    asset::Assets,
    ecs::{
        bundle::{self, Bundle},
        component::Component,
        resource::Resource,
        system::ResMut,
    },
    mesh::Mesh,
    sprite_render::ColorMaterial,
};
#[derive(Resource)]
pub struct ImageHandles {
    pub mob: Handle<Image>,
    pub boss: Handle<Image>,
}

impl FromWorld for ImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            mob: asset_server.load("mob.png"),
            boss: asset_server.load("boss.png"),
        }
    }
}
use crate::{movey, physics::GameLayer};
#[derive(Resource, Deref)]
pub struct Mobhandle(Handle<Image>);
#[derive(Component)]

pub struct Boss;

impl Boss {
    pub const SIZE: Vec2 = Vec2 { x: 91., y: 91. };

    pub fn bundle(handle: Res<ImageHandles>, y: f32, pos: Vec3, rot: Quat) -> impl Bundle {
        ((
            Sprite {
                image: handle.boss.clone(),
                custom_size: Some(Vec2 { x: 141., y: 142. }),
                ..Default::default()
            },
            Boss,
            MobHealth(3),
            Mob,
            movey(y),
            Hostile,
            Transform::from_translation(pos).with_rotation(rot),
        ))
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
            Mob,
            movey(y),
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
#[derive(Resource, Component)]
pub struct P {
    x: i32,
    y: i32,
}

#[derive(Component)]

pub struct MobHealth(pub i32);

#[derive(Component)]

pub struct Hostile;
