use bevy::prelude::*;

use crate::{
    mod_level_h::{Level, LevelState},
    start_de_spawn_mobs,
};

pub(crate) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (change_level_next_button, start_de_spawn_mobs)
                .chain()
                .run_if(system_on_key(KeyCode::ArrowRight)),
        );
    }
}

pub fn system_on_key(keycode: KeyCode) -> impl Fn(Res<ButtonInput<KeyCode>>) -> bool {
    move |keys: Res<ButtonInput<KeyCode>>| keys.just_pressed(keycode)
}

fn change_level_next_button(
    level: Res<State<Level>>,

    mut n2: ResMut<NextState<Level>>,
    mut n: ResMut<NextState<LevelState>>,
) {
    NextState::set_if_neq(&mut n2, level.next());
    NextState::set_if_neq(&mut n, LevelState::LevelStart);
}
