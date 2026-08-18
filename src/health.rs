use bevy::{prelude::*, state::commands};

use crate::{GameState, pla::Pla};

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
        .add_systems(OnEnter(GameState::GamePlay), startup)
        .add_systems(
            FixedUpdate,
            update_health.run_if(any_match_filter::<(With<Pla>, Changed<Health>)>),
        );
    }
}
impl Default for Health {
    fn default() -> Self {
        Self(3)
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        HealthText,
        Text::new(""),
        TextFont {
            font_size: 60.0,
            ..Default::default()
        },
    ));
}

#[derive(Component)]

pub struct Health(pub(crate) i32);
#[derive(EntityEvent)]

pub struct HealthEvent {
    pub entity: Entity,
    pub value: i32,
}
#[derive(EntityEvent)]

pub struct DespawnEvent {
    pub entity: Entity,
}
pub fn on_health_event(event: On<HealthEvent>, mut commands: Commands, mut a: Query<&mut Health>) {
    if let Ok(mut health) = a.get_mut(event.event_target()) {
        health.0 -= event.value;
        if health.0 <= 0 {
            commands.trigger(DespawnEvent {
                entity: event.event_target(),
            });
        }
    }
}
pub fn on_despawn_event(event: On<DespawnEvent>, mut commands: Commands) {
    commands.entity(event.event_target()).despawn();
}

#[derive(Component)]
struct HealthText;

pub fn update_health(
    health: Single<&Health, With<Pla>>,
    mut text: Single<&mut Text, With<HealthText>>,
) {
    println!("update_health");
    text.0 = format!("{}", health.0);
}

fn rehealth(mut score: Single<&mut Health, With<Pla>>) {
    **score = Health::default();
}
