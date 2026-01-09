use crate::prelude::*;

fn on_println(trigger: On<ConsolePrintln>, mut console_q: Query<&mut ConsoleWriteQueue>) {
    if let Ok(mut queue) = console_q.get_mut(trigger.console_id) {
        // TODO: This is an extra allocation. Could be a large string.
        queue.push(trigger.message.clone());
    } else {
        error!("Couldn't print to console with id {}", trigger.console_id);
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_println);
}
