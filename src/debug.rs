use bevy::{app::ctrlc::Error::System, prelude::*, state::commands};

use crate::{
    modLevH::{level, levelState},
    start_deSpawnMobs,
};

pub(crate) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (changlevelNextButton, start_deSpawnMobs)
                .chain()
                .run_if(system_on_key(KeyCode::ArrowRight)),
        );
    }
}

pub fn system_on_key(keycode: KeyCode) -> impl Fn(Res<ButtonInput<KeyCode>>) -> bool {
    move |keys: Res<ButtonInput<KeyCode>>| keys.just_pressed(keycode)
}
fn changlevelNextButton(
    level: Res<State<level>>,

    mut n2: ResMut<NextState<level>>,
    mut n: ResMut<NextState<levelState>>,
) {
    NextState::set_if_neq(&mut n2, level.next());
    NextState::set_if_neq(&mut n, levelState::levelStart);
}
