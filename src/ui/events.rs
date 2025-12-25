use std::fmt::Write;

use crate::prelude::*;

fn on_println(
    trigger: On<ConsolePrintln>,
    mut commands: Commands,
    mut console_q: Query<(&ConsoleBufferView, &mut Console)>,
) {
    if let Ok((view, mut console)) = console_q.get_mut(trigger.console_id) {
        write!(&mut console.buffer, "\n{}", trigger.message).unwrap();
        commands
            .entity(trigger.console_id)
            .insert(view.jump_to_bottom(&console));
    } else {
        error!("Couldn't print to console with id {}", trigger.console_id);
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_println);
}
