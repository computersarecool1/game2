use bevy::state::state::States;

#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum level {
    #[default]
    level1,
    level2,
}
#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum levelState {
    #[default]
    Inlevel,
    levelEnd,
}
use bevy::prelude::*;

use crate::{Asteroid, Boss, Mob};

pub(crate) struct MyLevelH;

impl Plugin for MyLevelH {
    fn build(&self, app: &mut App) {
        app.init_state::<level>();
        app.init_state::<levelState>();
        app.add_systems(FixedUpdate, changlevel);
    }
}

fn changlevel(
    sl: Res<State<levelState>>,
    mut n: ResMut<NextState<levelState>>,
    s: Res<crate::score::Score>,
) {
    if s.0 >= 5 {
        NextState::set_if_neq(&mut n, levelState::levelEnd);
    }
}

fn startnew(
    mut n2: ResMut<NextState<level>>,
    mut n: ResMut<NextState<levelState>>,
    query: Query<Entity, Or<(With<Mob>, With<Boss>, With<Asteroid>)>>,
    mut commands: Commands,
) {
    if query.is_empty() {
        NextState::set_if_neq(&mut n, levelState::Inlevel);
        NextState::set_if_neq(&mut n2, level::level2);
    };
}
