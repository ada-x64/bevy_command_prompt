use bevy::text::ComputedTextBlock;
use strum::IntoEnumIterator;

use crate::prelude::*;

fn on_call_console_command(
    trigger: On<CallCommandEvent>,
    cmds: Res<ConsoleCommands>,
    mut commands: Commands,
    console: Query<&Console>,
) {
    let split = trigger.command_name.split(" ").collect::<Vec<_>>();
    let name = r!(split.first());
    let console = console.get(trigger.console_id).unwrap();
    commands.trigger(ConsolePrintln {
        message: format!("{}{}", console.prompt, trigger.command_name),
        console_id: trigger.console_id,
    });
    if let Some(cmd) = cmds.get(*name) {
        commands.run_system_with(cmd.dispatch, trigger.event().clone());
    } else if let Some(cmd) = ConsoleBuiltin::iter().find(|b| b.to_string() == *name) {
        commands.run_system_cached_with(builtins, (cmd, trigger.console_id));
    } else {
        commands.trigger(ConsolePrintln {
            message: format!("Unknown command '{name}'"),
            console_id: trigger.console_id,
        });
    }
}

fn builtins(
    input: In<(ConsoleBuiltin, Entity)>,
    mut console_q: Query<(Entity, &mut Console, &ConsoleBufferView)>,
    mut commands: Commands,
) {
    match input.0.0 {
        ConsoleBuiltin::Clear => {
            let (entity, mut console, view) = console_q.get_mut(input.0.1).unwrap();
            console.buffer.clear();
            commands
                .entity(entity)
                .insert(view.jump_to_bottom(&console));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_call_console_command);
}
