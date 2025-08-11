use bevy::prelude::*;
use netcorehost::{nethost, pdcstr};

mod hello;

fn main() {
    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr
        .initialize_for_dotnet_command_line(pdcstr!(
            "dotnet_project\\bin\\Debug\\net9.0\\dotnet_project.dll"
        ))
        .unwrap();
    context.run_app().as_hosting_exit_code().unwrap();
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(hello::HelloPlugin)
        .run();
}
