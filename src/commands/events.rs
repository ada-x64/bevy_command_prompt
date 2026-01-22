use crate::prelude::*;

fn on_submit(trigger: On<SubmitEvent>, cmds: Res<ConsoleCommands>, mut commands: Commands) {
    let name = r!(trigger.args().first());
    if let Some(cmd) = cmds.get(name) {
        commands.run_system_with(cmd.dispatch, trigger.event().clone());
    } else {
        commands.write_message(ConsoleWriteMsg {
            message: format!("Unknown command '{name}'\n"),
            console_id: trigger.console_id,
        });
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_submit);
}
