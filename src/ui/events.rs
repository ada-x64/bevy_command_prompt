use crate::prelude::*;

fn on_println(
    trigger: On<ConsolePrintln>,
    mut commands: Commands,
    mut console_q: Query<(&ConsoleBufferView, &mut ConsoleWriteQueue)>,
) {
    if let Ok((view, mut queue)) = console_q.get_mut(trigger.console_id) {
        queue.push(trigger.message);
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
