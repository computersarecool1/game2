use bevy::{prelude::*, state::commands};

pub(crate) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                entered: crate::GameState::GamePlay,
                exited: crate::GameState::GameOver,
            },
            rehealth,
        )
        .add_systems(FixedUpdate, health)
        .init_resource::<Health>()
        .add_observer(on_unhealth_event)
        .add_systems(Startup, startup)
        .add_systems(
            FixedUpdate,
            update_health.run_if(resource_exists_and_changed::<Health>),
        );
    }
}
impl Default for Health {
    fn default() -> Self {
        Self(3)
    }
}

fn startup(mut commands: Commands, score: ResMut<Health>) {
    commands.spawn((
        HealthText,
        Text::new(format!("{}", score.0)),
        TextFont {
            font_size: 60.0,
            ..Default::default()
        },
    ));
}

#[derive(Resource)]

pub struct Health(pub(crate) i32);
#[derive(Event)]

pub struct healthEvent(pub i32);
fn health(commands: Commands, mut score: ResMut<Health>) {}
fn on_unhealth_event(
    event: On<healthEvent>,
    mut s: ResMut<NextState<crate::GameState>>,
    mut score: ResMut<Health>,
) {
    score.0 -= event.0;
    if score.0 < 0 {
        NextState::set_if_neq(&mut s, crate::GameState::GameOver);
    }
}
#[derive(Component)]
struct HealthText;

fn update_health(health: Res<Health>, mut text: Single<&mut Text, With<HealthText>>) {
    text.0 = format!("{}", health.0);
}

fn rehealth(commands: Commands, mut score: ResMut<Health>) {
    *score = Health::default();
}
