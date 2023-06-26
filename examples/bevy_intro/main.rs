mod greet_people;

use bevy::prelude::*;
use greet_people::HelloPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .run();
}
