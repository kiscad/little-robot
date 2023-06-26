use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
      .add_startup_system(add_people)
      .add_system(greet_people);
  }
}

#[derive(Component)]
struct Person(u64);

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
  if timer.0.tick(time.delta()).just_finished() {
    for name in &query {
      println!("hello {}!", name.0);
    }
  }
}

fn add_people(mut commands: Commands) {
  commands.spawn((Person(0), Name("Elaina Proctor".to_string())));
  commands.spawn((Person(1), Name("Renzo Hume".to_string())));
  commands.spawn((Person(2), Name("Zayna Nieves".to_string())));
}
