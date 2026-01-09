use bevy::text::ComputedTextBlock;
use strum::IntoEnumIterator;

use crate::prelude::*;

fn on_call_console_command(
    trigger: On<CallCommandEvent>,
    cmds: Res<ConsoleCommands>,
    mut commands: Commands,
    mut console_q: Query<(&ConsolePrompt, &mut ConsoleWriteQueue)>,
) {
    let split = trigger.command_name.split(" ").collect::<Vec<_>>();
    let name = r!(split.first());
    let (prompt, mut queue) = console_q.get_mut(trigger.console_id).unwrap();
    queue.push(format!("{}{}", **prompt, trigger.command_name));
    if let Some(cmd) = cmds.get(*name) {
        commands.run_system_with(cmd.dispatch, trigger.event().clone());
    } else if let Some(cmd) = ConsoleShellCommands::iter().find(|b| b.to_string() == *name) {
        commands.run_system_cached_with(shell_commands, (cmd, trigger.console_id));
    } else {
        commands.trigger(ConsolePrintln {
            message: format!("Unknown command '{name}'"),
            console_id: trigger.console_id,
        });
    }
}

fn shell_commands(
    input: In<(ConsoleShellCommands, Entity)>,
    mut console_q: Query<(
        Entity,
        &mut ConsoleBuffer,
        &ConsoleBufferView,
        &ConsolePrompt,
        &ComputedTextBlock,
    )>,
    mut commands: Commands,
) {
    match input.0.0 {
        ConsoleShellCommands::Clear => {
            let (entity, mut buffer, view, prompt, block) = console_q.get_mut(input.0.1).unwrap();
            buffer.clear();
            // TODO: This should be a console action.
            commands
                .entity(entity)
                .insert(view.jump_to_bottom(prompt, block));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_call_console_command);
}
