use crate::prelude::*;

fn on_call_console_command(
    trigger: On<CallConsoleCommand>,
    cmds: Res<ConsoleCommands>,
    mut commands: Commands,
    ui_settings: Res<ConsoleUiSettings>,
) {
    let split = trigger.0.split(" ").collect::<Vec<_>>();
    let name = r!(split.first());
    commands.trigger(AppendToConsole(format!(
        "{}{}",
        ui_settings.prompt, trigger.0
    )));
    if let Some(cmd) = cmds.get(*name) {
        commands.run_system_with(cmd.dispatch, trigger.0.clone());
    } else {
        commands.trigger(AppendToConsole(format!("Unknown command '{name}'")));
    }
}

fn on_clear(mut reader: MessageReader<ClearCmd>, mut commands: Commands) {
    if !reader.is_empty() {
        reader.clear();
        commands.trigger(ClearConsole);
    }
}

fn on_show_cmds(
    mut reader: MessageReader<ShowCommandsCmd>,
    cmds: Res<ConsoleCommands>,
    mut commands: Commands,
) {
    for msg in reader.read() {
        let str = if msg.long {
            cmds.values()
                .map(|cmd| cmd.cmd.clone().render_usage().to_string())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            cmds.values()
                .map(|cmd| cmd.cmd.get_name())
                .collect::<Vec<_>>()
                .join("\n")
        };
        commands.trigger(AppendToConsole(str));
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_call_console_command);
    app.add_systems(Update, (on_clear, on_show_cmds));
}
