use bevy::prelude::*;

#[derive(Default)]
struct PlaySound;

#[derive(Resource)]
struct EventTriggerState {
  event_timer: Timer,
}

impl Default for EventTriggerState {
  fn default() -> Self {
    EventTriggerState {
      event_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    }
  }
}

fn event_trigger(
  time: Res<Time>,
  mut state: ResMut<EventTriggerState>,
  mut play_sound_events: EventWriter<PlaySound>,
) {
  if state.event_timer.tick(time.delta()).finished() {
    play_sound_events.send_default();
  }
}

fn sound_player(mut play_sound_events: EventReader<PlaySound>) {
  for _ in play_sound_events.iter() {
    info!("Playing a sound");
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<PlaySound>()
    .init_resource::<EventTriggerState>()
    .add_system(event_trigger)
    .add_system(sound_player)
    .run();
}
