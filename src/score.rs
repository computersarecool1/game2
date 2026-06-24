use bevy::prelude::*;

pub(crate) struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                entered: crate::GameState::GamePlay,
                exited: crate::GameState::GameOver,
            },
            reSetScore,
        )
        .add_systems(FixedUpdate, score)
        .init_resource::<Score>()
        .add_observer(on_score_event);
    }
}

#[derive(Resource, Default)]

pub struct Score(pub(crate) i32);
#[derive(Event)]

pub struct ScoreEvent(pub i32);
fn score(mut score: ResMut<Score>, mut time: Res<Time>) {
    println!("{:?}", score.0)
}
fn on_score_event(event: On<ScoreEvent>, mut score: ResMut<Score>) {
    score.0 += event.0;
}

fn reSetScore(mut score: ResMut<Score>) {
    score.0 = 0;
}
