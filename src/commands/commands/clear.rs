use clap::Parser;

use crate::prelude::*;

#[derive(Parser, Debug, Message, Clone)]
#[command(name = "clear")]
struct ClearCmd;

pub fn clear_buffer(
    id: In<Entity>,
    mut console_q: Query<(&mut ConsoleBuffer, &mut ConsoleInputText)>,
    mut commands: Commands,
) {
    let (mut buffer, mut input_text) = console_q.get_mut(*id).unwrap();
    buffer.clear();
    commands.write_message(ConsoleViewMsg::jump_to_bottom(*id));
    input_text.text.clear();
    input_text.anchor = 0;
}

fn on_find_msg(mut reader: MessageReader<CommandMsg<ClearCmd>>, mut commands: Commands) {
    for msg in reader.read() {
        commands.run_system_cached_with(clear_buffer, msg.console_id);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, on_find_msg);
    app.add_console_command::<ClearCmd>();
}
