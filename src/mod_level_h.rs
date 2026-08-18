use bevy::state::state::States;

#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum Level {
    #[default]
    Level1,
    Level2,
    Level3,
    Level4,
}
#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum LevelState {
    #[default]
    Inlevel,
    LevelStart,
    LevelEnd,
}
use bevy::prelude::*;
impl Level {
    pub fn next(&self) -> Level {
        match self {
            Level::Level1 => Level::Level2,
            Level::Level2 => Level::Level3,
            Level::Level3 => Level::Level4,
            Level::Level4 => todo!(),
        }
    }
}
use crate::{Asteroid, Mob, enemy::Boss};

pub(crate) struct MyLevelH;

impl Plugin for MyLevelH {
    fn build(&self, app: &mut App) {
        app.init_state::<Level>();
        app.init_state::<LevelState>();
        app.add_systems(
            FixedUpdate,
            (
                start_new.run_if(in_state(LevelState::LevelEnd)),
                change_level.run_if(in_state(LevelState::Inlevel)),
            ),
        );
    }
}

fn change_level(
    mut n: ResMut<NextState<LevelState>>,
    s: Res<crate::score::Score>,
    c: Res<State<Level>>,
) {
    let end_score = match **c {
        Level::Level1 => 2,
        Level::Level2 => 5,
        Level::Level3 => 8,
        Level::Level4 => 13,
    };

    if s.0 >= end_score {
        NextState::set_if_neq(&mut n, LevelState::LevelEnd);
    }
}

fn start_new(
    level: Res<State<Level>>,
    mut n2: ResMut<NextState<Level>>,
    mut n: ResMut<NextState<LevelState>>,
    query: Query<Entity, Or<(With<Mob>, With<Boss>, With<Asteroid>)>>,
) {
    if query.is_empty() {
        NextState::set_if_neq(&mut n, LevelState::LevelStart);
        NextState::set_if_neq(&mut n2, level.next());
    };
}
