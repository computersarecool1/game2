use bevy::{log::Level, prelude::*};

use crate::{GameState, Pla, modLevH::{level, levelState}};

pub(crate) struct level2Plugin;

impl Plugin for level2Plugin {
fn build(&self, app: &mut App) {
app.add_systems(FixedUpdate, (x.run_if(in_state(levelState::levelStart)),rotate));

}
}
fn rotate(mut t: Single<(Entity,&mut Transform), With<Pla>>,    mut mes_pos: MessageReader<CursorMoved>,) {
    for mes_pos in mes_pos.read() {
        mes_pos
        todo!("mes_pos to rotate");
    }
}
fn x(  mut n: ResMut<NextState<levelState>>,mut t: Single<(Entity,&mut Transform), With<Pla>>) {
    let center = Vec3::new(0., 0., 0.);
    
    if t.1.translation.xy() != (0.,0.).into() {println!("dddddddd");let d = center - t.1.translation; t.1.translation += d *0.04;}

    if t.1.translation.xy() == (0.,0.).into() {NextState::set_if_neq(&mut n, levelState::Inlevel);}
}