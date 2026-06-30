use bevy::{log::Level, prelude::*};

use crate::{GameState, Pla, modLevH::{level, levelState}};

pub(crate) struct level2Plugin;

impl Plugin for level2Plugin {
fn build(&self, app: &mut App) {
app.add_systems(FixedUpdate, (x.run_if(in_state(levelState::levelStart)),rotate));

}
}


fn rotate(mut t: Single<(Entity,&mut Transform), With<Pla>>, 
   mut mes_pos: MessageReader<CursorMoved>,main: Single<(&Camera, &GlobalTransform), 
   With<Camera2d>>,) {
        let (camera, cam_transform) = main.into_inner();

    for mes in mes_pos.read() {
        
            if let Ok(pos) = camera.viewport_to_world_2d(cam_transform, mes.position) {
        let to_mouse = (pos - t.1.translation.xy()).normalize();
        let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));
        t.1.rotation = rotate_to_mouse;
        }

}}
fn x(  mut n: ResMut<NextState<levelState>>,mut t: Single<(Entity,&mut Transform), With<Pla>>) {
    let center = Vec3::new(0., 0., 0.);
    
    if t.1.translation.xy() != (0.,0.).into() {println!("dddddddd");let d = center - t.1.translation; t.1.translation += d *0.04;}

    if t.1.translation.xy() == (0.,0.).into() {NextState::set_if_neq(&mut n, levelState::Inlevel);}
}