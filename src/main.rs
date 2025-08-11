use bevy::prelude::*;
use netcorehost::{nethost, pdcstr};

mod hello;

fn main() {
    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr.initialize_for_runtime_config(pdcstr!("dotnet_project\\bin\\Debug\\net9.0\\dotnet_project.runtimeconfig.json")).unwrap();
    let delegate_loader = context
        .get_delegate_loader_for_assembly(pdcstr!(
            "dotnet_project\\bin\\Debug\\net9.0\\dotnet_project.dll"
        ))
        .unwrap();

    let hello_world1 = delegate_loader
        .get_function::<fn()>(
            pdcstr!("dotnet_project.Program, dotnet_project"),
            pdcstr!("HelloWorld1"),
            pdcstr!("dotnet_project.Program+HelloWorld1Delegate, dotnet_project"),
        )
        .unwrap();
    hello_world1();

    let hello_world2 = delegate_loader
        .get_function_with_unmanaged_callers_only::<fn()>(
            pdcstr!("dotnet_project.Program, dotnet_project"),
            pdcstr!("HelloWorld2"),
        )
        .unwrap();
    hello_world2();

    let hello_world3 = delegate_loader
        .get_function_with_default_signature(
            pdcstr!("dotnet_project.Program, dotnet_project"),
            pdcstr!("HelloWorld3"),
        )
        .unwrap();
    unsafe { hello_world3(std::ptr::null(), 0) };
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(hello::HelloPlugin)
        .run();
}
