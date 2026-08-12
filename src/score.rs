use bevy::prelude::*;

pub(crate) struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                entered: crate::GameState::GamePlay,
                exited: crate::GameState::GameOver,
            },
            reset_score,
        )
        .init_resource::<Score>()
        .add_observer(on_score_event);
    }
}

#[derive(Resource, Default)]

pub struct Score(pub(crate) i32);
#[derive(Event)]

pub struct ScoreEvent(pub i32);

fn on_score_event(event: On<ScoreEvent>, mut score: ResMut<Score>) {
    score.0 += event.0;
}

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}
