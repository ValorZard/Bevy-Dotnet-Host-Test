use bevy::prelude::*;

mod hello;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(hello::HelloPlugin)
        .run();
}
