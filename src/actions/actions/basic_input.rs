use bevy::input::keyboard::Key;

use crate::prelude::*;

pub fn delete_char(input: In<ConsoleActionSystemInput>, mut console_q: Query<&mut Console>) {
    if let Ok(mut console) = console_q.get_mut(input.console_id) {
        let popped = console.input.pop();
        if let Some(popped) = popped {
            console.cursor -= popped.len_utf8();
        }
    } else {
        error!(
            "Could not delete char from console with id {}",
            input.console_id
        );
    }
}
pub fn delete_word(input: In<ConsoleActionSystemInput>, mut console_q: Query<&mut Console>) {
    if let Ok(mut console) = console_q.get_mut(input.console_id) {
        let last_ws = console.input.rfind(char::is_whitespace).unwrap_or_default();
        console.input.truncate(last_ws);
        console.cursor = last_ws;
    } else {
        error!(
            "Could not delete word from console with id {}",
            input.console_id
        );
    }
}
pub fn write_char(input: In<ConsoleActionSystemInput>, mut console_q: Query<&mut Console>) {
    if let Ok(mut console) = console_q.get_mut(input.console_id) {
        let pos = console.cursor;
        for key in input.matched_logical_keys() {
            match key {
                Key::Character(c) => {
                    console.input.insert_str(pos, c.as_str());
                    console.cursor += c.len();
                }
                Key::Space => {
                    console.input.insert(pos, ' ');
                    console.cursor += 1;
                }
                _ => {}
            }
        }
    } else {
        error!(
            "Could not write char to console with id {}",
            input.console_id
        );
    }
}

pub fn submit(
    input: In<ConsoleActionSystemInput>,
    mut query: Query<(&mut Console, &mut ConsoleHistory)>,
    mut commands: Commands,
) {
    if let Ok((mut console, mut history)) = query.get_mut(input.console_id) {
        if let Some(event) = SubmitEvent::new(input.console_id, console.input.clone()) {
            commands.trigger(event);
        } else {
            commands.trigger(ConsolePrintln {
                message: "Invalid shell expression".into(),
                console_id: input.console_id,
            });
        }
        let history_value = std::mem::take(&mut console.input);
        console.cursor = 0;
        history.push(history_value);
    } else {
        error!("Could not submit from console with id {}", input.console_id);
    }
}

// TODO: another console action
fn on_scroll(
    input: In<ConsoleActionSystemInput>,
    mut commands: Commands,
    console_q: Query<(&ConsoleBuffer, &ConsolePrompt, &ConsoleBufferView)>,
) {
    let scroll = input.matched_scroll();
    if scroll.is_none() {
        error!("Tried to scroll console with no scroll value");
        return;
    }
    let scroll = scroll.unwrap();
    let (buffer, prompt, view) = console_q.get(input.console_id).unwrap();
    let range = view.range;
    let buffer_size = buffer.line_count();
    let prompt_size = prompt.lines().count();
    if buffer_size <= range {
        return;
    }
    let start = view
        .start
        .saturating_add_signed(scroll.delta.y as isize)
        .min(buffer_size - range + prompt_size);
    let new_view = ConsoleBufferView { start, ..*view };
    commands.entity(input.console_id).insert(new_view);
}

pub fn jump_to_bottom(
    input: In<ConsoleActionSystemInput>,
    mut console_q: Query<(
        &ComputedConsoleTextBlock,
        &ConsolePrompt,
        &mut ConsoleBufferView,
    )>,
    mut commands: Commands,
) {
    let (block, prompt, view) = console_q.get_mut(input.console_id).unwrap();
    let new_view = view.jump_to_bottom(prompt, block);
    commands.entity(input.console_id).insert(new_view);
}

pub(crate) fn plugin(app: &mut App) {
    app.register_console_action(
        ConsoleActionKeybind::new(Key::Backspace)
            .without_modifiers([KeyCode::ControlLeft, KeyCode::ControlRight]),
        delete_char,
    );
    app.register_console_action(
        ConsoleActionKeybind::new(Key::Backspace)
            .with_modifiers([KeyCode::ControlLeft, KeyCode::ControlRight]),
        delete_word,
    );
    app.register_console_action(
        ConsoleActionKeybind::new([ConsoleInput::AnyCharacter, Key::Space.into()]),
        write_char,
    );
    app.register_console_action(
        ConsoleActionKeybind::new([Key::Enter])
            .without_modifiers([KeyCode::ShiftLeft, KeyCode::ShiftRight]),
        submit,
    );
    app.register_console_action(ConsoleActionKeybind::new(ConsoleInput::Scroll), on_scroll);
    app.register_console_action(
        ConsoleActionKeybind::new(ConsoleInput::AnyKey),
        jump_to_bottom,
    );
}
