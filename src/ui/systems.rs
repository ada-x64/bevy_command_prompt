use crate::prelude::*;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::MouseScrollUnit,
    },
    input_focus::InputFocus,
    platform::collections::HashMap,
    ui::ui_layout_system,
};

/// submission event reader
/// this must occur after [TextInputSystem](bevy_simple_text_input::TextInputSystem),
/// but that's in the Update schedule so no need to worry
fn on_submit_msg(
    mut reader: MessageReader<ConsoleSubmitMsg>,
    mut query: Query<&mut Console>,
    mut commands: Commands,
) {
    for msg in reader.read() {
        if let Ok(mut console) = query.get_mut(msg.console_id) {
            commands.trigger(CallCommandEvent {
                command_name: console.input.clone(),
                console_id: msg.console_id,
            });
            let history_value = std::mem::take(&mut console.input);
            console.history.push(history_value);
        } else {
            error!("Could not submit from console with id {}", msg.console_id);
        }
    }
}

fn keyboard_input(
    key_codes: Res<ButtonInput<KeyCode>>, // for modifiers
    keys: Res<ButtonInput<Key>>,          // for modifiers
    mut input_events: MessageReader<KeyboardInput>,
    actions: Res<ConsoleActionCache>,
    focus: Res<InputFocus>,
    mut q_console: Query<(&mut Console, &ConsoleBufferView)>,
    mut commands: Commands,
) {
    if !input_events.is_empty()
        && let Some(console_id) = focus.0
        && let Ok((mut console, buffer_view)) = q_console.get_mut(console_id)
    {
        let mut needs_refresh = false;

        let current_actions = actions.keys().filter_map(|action| {
            if action
                .bad_keys
                .iter()
                .any(|or_group| or_group.iter().any(|key| keys.just_pressed(key.clone())))
            {
                return None;
            }
            if action
                .bad_mods
                .iter()
                .any(|or_group| or_group.iter().any(|key_code| key_codes.pressed(*key_code)))
            {
                return None;
            }

            let matched_keys = action.keys.iter().try_fold(vec![], |mut res, or_group| {
                or_group
                    .iter()
                    .find(|key| keys.just_pressed((**key).clone()))
                    .map(|found| {
                        res.push(found.clone());
                        res
                    })
            });

            let matched_mods = action
                .modifiers
                .iter()
                .try_fold(vec![], |mut res, or_group| {
                    or_group
                        .iter()
                        .find(|key| key_codes.pressed(**key))
                        .map(|found| {
                            res.push(*found);
                            res
                        })
                });
            matched_keys
                .zip(matched_mods)
                .map(|(keys, mods)| (action.clone(), keys, mods))
        });
        for (action, matched_keys, matched_mods) in current_actions {
            info!("Firing action {action:?}");
            let id = actions.get(&action).unwrap();

            commands.run_system_with(
                *id,
                ConsoleActionInput {
                    action,
                    console_id,
                    matched_keys,
                    matched_mods,
                },
            );
            needs_refresh = true;
        }

        // assumes console buffer can be written to right now
        // this is always true in real terminals
        for event in input_events.read() {
            if key_codes.just_pressed(event.key_code) {
                match event.logical_key {
                    Key::Enter => {
                        commands.write_message(ConsoleSubmitMsg { console_id });
                        needs_refresh = true;
                    }
                    Key::Character(ref c) => {
                        console.input = format!("{}{}", console.input, c);
                        needs_refresh = true;
                    }
                    Key::Space => {
                        console.input.push(' ');
                        needs_refresh = true;
                    }
                    _ => {}
                }
            }
        }
        if needs_refresh {
            commands
                .entity(console_id)
                .insert(buffer_view.jump_to_bottom(&console));
        }
        debug!(?console);
    }
}

fn on_scroll(
    mut reader: MessageReader<ConsoleScrollMsg>,
    mut commands: Commands,
    settings_q: Query<&ConsoleUiSettings>,
    console_q: Query<(&Console, &ConsoleBufferView)>,
) {
    let mut map = HashMap::<Entity, Vec2>::new();
    for msg in reader.read() {
        let settings = settings_q.get(msg.console_id).unwrap();
        let delta = match msg.message.unit {
            MouseScrollUnit::Line => Vec2::new(msg.message.x, msg.message.y),
            MouseScrollUnit::Pixel => {
                Vec2::new(msg.message.x, msg.message.y / settings.line_height())
            }
        };
        map.entry(msg.console_id)
            .and_modify(|d| *d -= delta)
            .or_insert(delta);
    }
    for (console_id, delta) in map.into_iter() {
        let (console, view) = console_q.get(console_id).unwrap();
        let range = view.range;
        let buffer_size = console.buffer.lines().count();
        let prompt_size = console.prompt.lines().count();
        if buffer_size <= range {
            continue; // disable scroll. this should be handled elsewhere.
        }
        let start = view
            .start
            .saturating_add_signed(delta.y as isize)
            .min(buffer_size - range + prompt_size);
        let new_view = ConsoleBufferView { start, ..*view };
        commands.entity(console_id).insert(new_view);
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<ConsolePrintln>();
    app.add_message::<ConsoleScrollMsg>();
    app.add_message::<ConsoleSubmitMsg>();
    app.add_systems(
        PostUpdate,
        (
            keyboard_input.run_if(
                resource_changed::<ButtonInput<KeyCode>>.and(resource_exists::<InputFocus>),
            ),
            on_submit_msg.before(ui_layout_system),
            on_scroll.before(ui_layout_system),
        ),
    );
}
