use bevy::prelude::*;
use netcorehost::{hostfxr::AssemblyDelegateLoader, nethost, pdcstr};

#[derive(Resource)]
struct DelegateLoader(AssemblyDelegateLoader);

#[derive(Resource)]
struct SharpTimer(Timer);

fn hello_from_dotnet(
    time: Res<Time>,
    mut timer: ResMut<SharpTimer>,
    delegate_loader: Res<DelegateLoader>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let delegate_loader = &delegate_loader.0;
        let hello_world1 = delegate_loader
            .get_function::<fn()>(
                pdcstr!("TestProject.Program, dotnet_project"),
                pdcstr!("HelloWorld1"),
                pdcstr!("TestProject.Program+HelloWorld1Delegate, dotnet_project"),
            )
            .unwrap();
        hello_world1();

        let hello_world2 = delegate_loader
            .get_function_with_unmanaged_callers_only::<fn()>(
                pdcstr!("TestProject.Program, dotnet_project"),
                pdcstr!("HelloWorld2"),
            )
            .unwrap();
        hello_world2();

        let hello_world3 = delegate_loader
            .get_function_with_default_signature(
                pdcstr!("TestProject.Program, dotnet_project"),
                pdcstr!("HelloWorld3"),
            )
            .unwrap();
        unsafe { hello_world3(std::ptr::null(), 0) };
    }
}

pub struct SharpPlugin;

impl Plugin for SharpPlugin {
    fn build(&self, app: &mut App) {
        let hostfxr = nethost::load_hostfxr().unwrap();
        let context = hostfxr
            .initialize_for_runtime_config(pdcstr!(
                "dotnet_project\\bin\\Debug\\net9.0\\dotnet_project.runtimeconfig.json"
            ))
            .unwrap();
        let delegate_loader = context
            .get_delegate_loader_for_assembly(pdcstr!(
                "dotnet_project\\bin\\Debug\\net9.0\\dotnet_project.dll"
            ))
            .unwrap();
        app.insert_resource(DelegateLoader(delegate_loader));
        app.insert_resource(SharpTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Update, (hello_from_dotnet,));
    }
}
