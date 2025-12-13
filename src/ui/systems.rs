use bevy::{prelude::*, ui::ui_layout_system};
use bevy_simple_text_input::TextInputSubmitMessage;

use crate::ui::prelude::*;

/// submission event reader
/// this must occur after [TextInputSystem](bevy_simple_text_input::TextInputSystem),
/// but that's in the Update schedule so no need to worry
fn on_submit_msg(
    mut reader: MessageReader<TextInputSubmitMessage>,
    filter: Query<Entity, With<ConsoleInputValue>>,
    mut commands: Commands,
) {
    for msg in reader.read() {
        // note: txtinput value cleared on submit
        if filter.iter().any(|e| e == msg.entity) {
            commands.trigger(CallConsoleCommand(msg.value.clone()));
            commands.trigger(AppendToConsole(msg.value.clone()));
        }
    }
}

fn on_append_to_console(
    mut reader: MessageReader<AppendToConsole>,
    q: Query<(&mut ScrollPosition, &ComputedNode), With<ConsoleBodyTextWrapper>>,
) {
    // NOTE: The intention here is to allow for multiple console.
    // This code doesn't quite work for that.
    // Are we intended to allow multiples? I don't really see the use case,
    // but maximal flexibility in a library is probably best.
    let msgs = reader.read().collect::<Vec<_>>();
    for (mut pos, cnode) in q {
        for _msg in &msgs {
            let max_offset = (cnode.content_size() - cnode.size()) * cnode.inverse_scale_factor();
            pos.x = 0.;
            pos.y = max_offset.y;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<AppendToConsole>();
    app.add_systems(
        PostUpdate,
        (
            on_submit_msg.before(ui_layout_system),
            on_append_to_console.after(ui_layout_system),
        ),
    );
}
