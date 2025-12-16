use crate::prelude::*;

fn on_call_console_command(
    trigger: On<CallConsoleCommand>,
    cmds: Res<ConsoleCommands>,
    mut commands: Commands,
    ui_settings: Res<ConsoleUiSettings>,
) {
    let split = trigger.0.split(" ").collect::<Vec<_>>();
    let name = r!(split.first());
    commands.trigger(ConsolePrint(format!("{}{}", ui_settings.prompt, trigger.0)));
    if let Some(cmd) = cmds.get(*name) {
        commands.run_system_with(cmd.dispatch, trigger.0.clone());
    } else {
        commands.trigger(ConsolePrint(format!("Unknown command '{name}'")));
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_call_console_command);
}
