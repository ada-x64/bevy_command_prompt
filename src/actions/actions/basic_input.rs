use bevy::input::keyboard::Key;

use crate::prelude::*;

pub fn delete_char(
    input: In<ConsoleActionSystemInput>,
    mut console_q: Query<&mut ConsoleInputText>,
) {
    if let Ok(mut input) = console_q.get_mut(input.console_id) {
        let popped = input.text.pop();
        if let Some(popped) = popped {
            input.move_cursor(-(popped.len_utf8() as isize));
        }
    } else {
        error!(
            "Could not delete char from console with id {}",
            input.console_id
        );
    }
}
pub fn delete_word(
    input: In<ConsoleActionSystemInput>,
    mut console_q: Query<&mut ConsoleInputText>,
) {
    if let Ok(mut input) = console_q.get_mut(input.console_id) {
        let last_ws = input.text.rfind(char::is_whitespace).unwrap_or_default();
        input.text.truncate(last_ws);
        input.set_cursor(last_ws);
    } else {
        error!(
            "Could not delete word from console with id {}",
            input.console_id
        );
    }
}
pub fn write_char(
    input: In<ConsoleActionSystemInput>,
    mut console_q: Query<&mut ConsoleInputText>,
    mut commands: Commands,
) {
    if let Ok(mut input_text) = console_q.get_mut(input.console_id) {
        let pos = input_text.cursor();
        for key in input.matched_logical_keys() {
            match key {
                Key::Character(c) => {
                    input_text.text.insert_str(pos, c.as_str());
                    input_text.move_cursor(c.len() as isize);
                }
                Key::Space => {
                    input_text.text.insert(pos, ' ');
                    input_text.move_cursor(1);
                }
                Key::Enter => {
                    if input_text.cursor() == input_text.text.len() {
                        input_text.text.insert(pos, '\n');
                    }
                    input_text.text.push('\n');
                }
                _ => {}
            }
        }
        commands.write_message(ConsoleViewMsg::jump_to_bottom(input.console_id));
    } else {
        error!(
            "Could not write char to console with id {}",
            input.console_id
        );
    }
}

pub fn submit(
    input: In<ConsoleActionSystemInput>,
    mut query: Query<(
        &mut ConsoleBuffer,
        &mut ConsoleInputText,
        &mut ConsoleHistory,
    )>,
    mut commands: Commands,
) {
    if let Ok((mut buffer, mut input_text, mut history)) = query.get_mut(input.console_id) {
        buffer.write("\n").unwrap();
        if let Some(event) = SubmitEvent::new(input.console_id, input_text.text.clone()) {
            commands.trigger(event);
        } else {
            commands.write_message(ConsoleWriteMsg {
                message: "Invalid shell expression\n".into(),
                console_id: input.console_id,
            });
        }
        let history_value = std::mem::take(&mut input_text.text);
        input_text.set_cursor(0);
        history.push(history_value);
    } else {
        error!("Could not submit from console with id {}", input.console_id);
    }
}

fn on_scroll(input: In<ConsoleActionSystemInput>, mut commands: Commands) {
    let scroll = r!(input.matched_scroll());
    commands.write_message(ConsoleViewMsg::scroll(scroll, input.console_id));
}

fn clear(input: In<ConsoleActionSystemInput>, mut commands: Commands) {
    commands.run_system_cached_with(clear_buffer, input.console_id);
}

fn clear_input(input: In<ConsoleActionSystemInput>, mut query: Query<&mut ConsoleInputText>) {
    let mut input = query.get_mut(input.console_id).unwrap();
    input.text.clear();
    input.set_cursor(0);
}

pub fn scroll_line(input: In<ConsoleActionSystemInput>, mut commands: Commands) {
    let delta = match input.0.matched_logical_keys().find(|k| **k != Key::Control) {
        Some(Key::ArrowUp) => 1,
        Some(Key::ArrowDown) => -1,
        _ => unreachable!(),
    };
    commands.write_message(ConsoleViewMsg::scroll(delta, input.console_id));
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
    app.register_console_action(ConsoleActionKeybind::new(Key::Enter), submit);
    app.register_console_action(ConsoleActionKeybind::new(ConsoleInput::Scroll), on_scroll);
    app.register_console_action(
        ConsoleActionKeybind::new([Key::ArrowUp, Key::ArrowDown])
            .with_modifiers([KeyCode::ShiftLeft, KeyCode::ShiftRight]),
        scroll_line,
    );
    app.register_console_action(
        ConsoleActionKeybind::new(Key::Character("l".into()))
            .with_modifiers([KeyCode::ControlLeft, KeyCode::ControlRight]),
        clear,
    );
    app.register_console_action(
        ConsoleActionKeybind::new(Key::Character("c".into()))
            .with_modifiers([KeyCode::ControlLeft, KeyCode::ControlRight]),
        clear_input,
    );
}
