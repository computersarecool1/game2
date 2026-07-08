use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameOver, GameState, Pla};

pub(crate) struct physicsPlugin;

impl Plugin for physicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(61.));
        #[cfg(debug_assertions)]
        app.add_plugins(avian2d::prelude::PhysicsDebugPlugin);
        app.add_systems(
            FixedUpdate,
            check_player_collisions.run_if(in_state(GameState::GamePlay)),
        );
    }
}
fn check_player_collisions(
    mut mes: MessageWriter<GameOver>,
    player: Single<Entity, With<Pla>>,
    collisions: Collisions,
) {
    if collisions.collisions_with(*player).count() != 0 {
        mes.write_default();
    }
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Bullet,
    Mob,
    Asteroid,
    ShipPart,
}
