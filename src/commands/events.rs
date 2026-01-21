use strum::IntoEnumIterator;

use crate::prelude::*;

fn on_submit(trigger: On<SubmitEvent>, cmds: Res<ConsoleCommands>, mut commands: Commands) {
    let name = r!(trigger.args().first());
    if let Some(cmd) = cmds.get(name) {
        commands.run_system_with(cmd.dispatch, trigger.event().clone());
    } else if let Some(cmd) = ConsoleShellCommands::iter().find(|b| b.to_string() == *name) {
        commands.run_system_cached_with(shell_commands, (cmd, trigger.console_id));
    } else {
        commands.write_message(ConsoleWriteMsg {
            message: format!("Unknown command '{name}'\n"),
            console_id: trigger.console_id,
        });
    }
}

// TODO: This should be just a regular command. /usr/bin/clear
fn shell_commands(
    // command, console_id
    input: In<(ConsoleShellCommands, Entity)>,
    mut console_q: Query<(&mut ConsoleBuffer, &mut ConsoleInputText)>,
    mut commands: Commands,
) {
    let In((cmd, console_id)) = input;
    match cmd {
        ConsoleShellCommands::Clear => {
            let (mut buffer, mut input_text) = console_q.get_mut(console_id).unwrap();
            buffer.clear();
            commands.write_message(ConsoleViewMsg::jump_to_bottom(console_id));
            input_text.text.clear();
            input_text.anchor = 0;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_submit);
}
