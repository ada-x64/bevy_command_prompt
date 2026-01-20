use crate::prelude::*;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{AccumulatedMouseScroll, MouseButtonInput},
    },
    input_focus::InputFocus,
    render::RenderApp,
    text::detect_text_needs_rerender,
    ui::ui_layout_system,
    ui_render::RenderUiSystems,
};

pub fn handle_input(
    key_code_input: Res<ButtonInput<KeyCode>>,
    key_input: Res<ButtonInput<Key>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    scroll: Res<AccumulatedMouseScroll>,
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut mouse_events: MessageReader<MouseButtonInput>,
    actions: Res<ConsoleActionCache>,
    focus: Res<InputFocus>,
    mut q_console: Query<&mut ComputedConsoleTextBlock>,
    mut commands: Commands,
) {
    if !keyboard_events.is_empty()
        && let Some(console_id) = focus.0
        && let Ok(mut block) = q_console.get_mut(console_id)
    {
        block.trigger_rerender();
        // want to collect here so we can iterate multiple times.
        let keyboard_events = keyboard_events.read().collect::<Vec<_>>();
        let mouse_events = mouse_events.read().collect::<Vec<_>>();
        actions
            .iter()
            .filter_map(|(keybind, s)| {
                keybind
                    .to_system_input(
                        &keyboard_events,
                        &mouse_events,
                        &key_input,
                        &key_code_input,
                        &mouse_input,
                        (scroll.delta != Vec2::ZERO).then_some(&*scroll),
                        console_id,
                    )
                    .map(|i| (i, *s))
            })
            .for_each(|(input, system)| {
                commands.write_message(ConsoleActionMsg {
                    console_id,
                    input,
                    system,
                });
            });
    }
}

pub fn clear_action_queue(mut reader: MessageReader<ConsoleActionMsg>, mut commands: Commands) {
    for item in reader.read() {
        commands.run_system_with(item.system, item.input.clone());
    }
}

pub fn clear_write_queue(
    mut reader: MessageReader<ConsoleWriteMsg>,
    mut buffer_q: Query<&mut ConsoleBuffer>,
) {
    for item in reader.read() {
        let mut buffer = c!(buffer_q.get_mut(item.console_id));
        c!(buffer.write(&item.message));
    }
}

/// The main entrypoint for bevy_command_prompt.
pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            crate::ui::plugin,
            crate::commands::plugin,
            crate::actions::plugin,
        ));
        app.add_systems(
            PostUpdate,
            (
                (
                    handle_input.run_if(resource_exists::<InputFocus>),
                    clear_action_queue,
                    clear_write_queue,
                )
                    .chain()
                    .before(ui_layout_system),
                (
                    // todo: detect if console text needs rerender
                    measure_console_text_system,
                    update_console_text_layout,
                )
                    .after(bevy::text::free_unused_font_atlases_system)
                    .before(bevy::asset::AssetEventSystems)
                    // these are separate entities.
                    .ambiguous_with(detect_text_needs_rerender::<Text2d>)
                    .ambiguous_with(detect_text_needs_rerender::<Text>)
                    .ambiguous_with(bevy::sprite::update_text2d_layout)
                    .ambiguous_with(bevy::sprite::calculate_bounds_text2d),
            ),
        );

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.add_systems(
            ExtractSchedule,
            extract_console_text_sections.in_set(RenderUiSystems::ExtractText),
        );
    }
}
