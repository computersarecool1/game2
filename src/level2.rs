use bevy::{log::Level, prelude::*};

use crate::modLevH::level;

pub(crate) struct level2Plugin;

impl Plugin for level2Plugin {
fn build(&self, app: &mut App) {
app.add_systems(FixedUpdate, (a,).run_if(in_state(level::level2)));
}
}

fn a() {println!("dddd")}