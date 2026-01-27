use bevy::input::keyboard::Key;

use crate::prelude::*;

pub fn set_from_history(
    input: In<ConsoleActionSystemInput>,
    mut q_console: Query<(&mut ConsoleInputText, &ConsoleHistory)>,
    mut history_idx: Local<usize>,
    mut filtered_history: Local<Option<Vec<usize>>>,
    mut original_value: Local<Option<String>>,
) {
    let key = input.matched_logical_keys().next();
    if key.is_none() {
        return;
    }
    let key = key.unwrap();
    let mut value = 0;
    match key {
        Key::ArrowUp => value = 1,
        Key::ArrowDown => value = -1,
        Key::Enter => {
            *history_idx = 0;
            *filtered_history = None;
            *original_value = None;
        }
        _ => {}
    }
    if matches!(key, Key::ArrowUp | Key::ArrowDown) {
        let (mut input_text, history) = q_console.get_mut(input.console_id).unwrap();
        if filtered_history.is_none() {
            *original_value = Some(std::mem::take(&mut input_text.text));
            let f = history
                .iter()
                .enumerate()
                .filter_map(|(i, s)| s.starts_with(original_value.as_ref().unwrap()).then_some(i))
                .collect::<Vec<_>>();
            *filtered_history = Some(f);
        }
        let fh = filtered_history.as_ref().unwrap();
        let ov = original_value.as_ref().unwrap();
        *history_idx = history_idx.saturating_add_signed(value).min(fh.len());
        if *history_idx == 0 {
            input_text.text = ov.clone();
        } else {
            let idx = fh[fh.len().saturating_sub(*history_idx)];
            input_text.text = history[idx].clone();
        }
        let end = input_text.text.len();
        input_text.set_cursor(end);
    }
}

pub fn plugin(app: &mut App) {
    app.register_console_action(
        ConsoleActionKeybind::new([Key::ArrowUp, Key::ArrowDown, Key::Enter])
            .without_modifiers([KeyCode::ShiftLeft, KeyCode::ShiftRight]),
        set_from_history,
    );
}

#[cfg(test)]
mod test {
    use std::num::NonZeroU8;

    use crate::prelude::*;
    use crate::test_harness;
    use bevy::ecs::relationship::RelationshipSourceCollection;
    use bevy::input::ButtonState;
    use bevy::input::keyboard::Key;
    use bevy::input::keyboard::KeyboardInput;
    use bevy_test_harness::prelude::*;

    fn key_input(key_code: KeyCode, logical_key: Key, state: ButtonState) -> KeyboardInput {
        KeyboardInput {
            key_code,
            logical_key,
            state,
            text: None,
            repeat: false,
            window: Entity::new(), // shouldn't matter
        }
    }

    macro_rules! check_and_input {
        ($app:ident, $step:expr, $value:expr, $key:ident) => {
            $app.add_step($step, |world: &mut World| {
                let input = world
                    .query::<&ConsoleInputText>()
                    .single_mut(world)
                    .unwrap();
                assert_eq!(input.text, $value);
                world.write_message(key_input(KeyCode::$key, Key::$key, ButtonState::Pressed));
                world.write_message(key_input(KeyCode::$key, Key::$key, ButtonState::Released));
                world.resource_mut::<NextState<Step>>().set(Step($step + 1));
            });
        };
    }

    #[test]
    fn test_history() {
        let mut app = App::new();
        app.add_plugins(test_harness::plugin);
        for step in 0..3 {
            app.add_step(
                step,
                move |mut q: Query<&mut ConsoleInputText>,
                      mut commands: Commands,
                      mut next_step: ResMut<NextState<Step>>| {
                    if let Ok(mut input) = q.single_mut() {
                        input.text = step.to_string();
                        commands.write_message(key_input(
                            KeyCode::Enter,
                            Key::Enter,
                            ButtonState::Pressed,
                        ));
                        commands.write_message(key_input(
                            KeyCode::Enter,
                            Key::Enter,
                            ButtonState::Released,
                        ));
                        next_step.set(Step(step + 1));
                    } else {
                        error!("Failed to get console");
                        commands.write_message(AppExit::Error(NonZeroU8::new(1).unwrap()));
                    }
                },
            );
        }
        app.add_step(3, |world: &mut World| {
            let console_history = world.query::<&ConsoleHistory>().single_mut(world).unwrap();
            assert_eq!(
                **console_history,
                vec!["0".to_string(), "1".to_string(), "2".to_string()]
            );
            world.write_message(key_input(
                KeyCode::ArrowUp,
                Key::ArrowUp,
                ButtonState::Pressed,
            ));
            world.write_message(key_input(
                KeyCode::ArrowUp,
                Key::ArrowUp,
                ButtonState::Released,
            ));
            world.resource_mut::<NextState<Step>>().set(Step(4));
        });
        check_and_input!(app, 4, "2", ArrowUp);
        check_and_input!(app, 5, "1", ArrowUp);
        check_and_input!(app, 6, "0", ArrowDown);
        app.add_step(7, |world: &mut World| {
            let input = world
                .query::<&ConsoleInputText>()
                .single_mut(world)
                .unwrap();
            assert_eq!(input.text, "1");
            world.write_message(AppExit::Success);
        });
        app.run();
    }
}
