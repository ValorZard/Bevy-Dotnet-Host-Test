use bevy::prelude::*;

mod hello;
mod sharp_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(hello::HelloPlugin)
        .add_plugins(sharp_plugin::SharpPlugin)
        .run();
}
