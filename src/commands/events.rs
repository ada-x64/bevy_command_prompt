use strum::IntoEnumIterator;

use crate::prelude::*;

fn on_submit(
    trigger: On<SubmitEvent>,
    cmds: Res<ConsoleCommands>,
    mut commands: Commands,
    console_q: Query<&ConsolePrompt>,
) {
    let prompt = r!(console_q.get(trigger.console_id()));

    commands.write_message(ConsoleWriteMsg {
        message: format!("{}{}\n", **prompt, trigger.input()),
        console_id: trigger.console_id(),
    });

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

fn shell_commands(
    input: In<(ConsoleShellCommands, Entity)>,
    mut console_q: Query<(
        Entity,
        &mut ConsoleBuffer,
        &ConsoleBufferView,
        &ConsolePrompt,
        &mut ComputedConsoleTextBlock,
    )>,
    mut commands: Commands,
) {
    match input.0.0 {
        ConsoleShellCommands::Clear => {
            let (entity, mut buffer, view, prompt, mut block) =
                console_q.get_mut(input.0.1).unwrap();
            buffer.clear();
            // TODO: This should be a console action.
            commands
                .entity(entity)
                .insert(view.jump_to_bottom(prompt, &mut block));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_submit);
}
