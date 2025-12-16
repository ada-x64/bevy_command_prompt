use crate::ui::events::on_scroll_handler;

use crate::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input_focus::InputFocus,
};
use bevy_ui_text_input::{TextInputMode, TextInputNode};

// <Console>
//   <ConsoleBody>
//     <ConsoleBodyText />
//     <ConsoleBodyText />
//     <ConsoleBodyText />
//   </ConsoleBody>
//   <ConsoleInput />
// </Console>

#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
#[component(on_add = ConsoleInput::add)]
pub struct ConsoleInput;
impl ConsoleInput {
    fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        let settings = world.resource::<ConsoleUiSettings>();

        // TODO: Multiline prompts need better size constraints.
        let prompt_bundle = (
            settings.text(settings.prompt.clone()),
            Node {
                width: Val::Px(
                    settings.font.font_size * settings.prompt.to_ascii_lowercase().len() as f32,
                ),
                ..Default::default()
            },
        );

        let input_bundle = (
            TextInputNode {
                clear_on_submit: true,
                mode: TextInputMode::SingleLine,
                unfocus_on_submit: false,
                ..Default::default()
            },
            settings.font.clone(),
            ConsoleInputValue,
            Node {
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(0.)),
                margin: UiRect::all(Val::Px(0.)),
                ..Default::default()
            },
        );

        let bundle = (
            Name::new("ConsoleInput"),
            Node {
                height: Val::Px(settings.line_height()),
                display: Display::Flex,
                padding: UiRect::all(Val::Px(0.)),
                margin: UiRect::all(Val::Px(0.)),
                ..Default::default()
            },
            children![prompt_bundle, input_bundle],
        );
        world.commands().entity(ctx.entity).insert(bundle);
    }
}

#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
#[component(on_add = ConsoleBody::add)]
pub struct ConsoleBody;
impl ConsoleBody {
    fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        let console_body = (
            Name::new("ConsoleBody"),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.),
                overflow: Overflow::scroll(),
                border: UiRect::all(px(1.)),
                ..Default::default()
            },
            ConsoleBodyTextWrapper,
        );

        world
            .commands()
            .entity(ctx.entity)
            .insert(console_body)
            .observe(on_scroll_handler);
    }
}

impl Console {
    pub(crate) fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        let settings = world.resource::<ConsoleUiSettings>();
        let bundle = (
            Name::new("Console"),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                overflow: Overflow::hidden(),
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            BackgroundColor(settings.background_color),
            children![ConsoleInput, ConsoleBody],
        );
        world
            .commands()
            .entity(ctx.entity)
            .insert(bundle)
            .observe(Self::on_click);
    }
    fn on_click(
        _trigger: On<Pointer<Click>>,
        mut focus: ResMut<InputFocus>,
        target: Single<Entity, With<ConsoleInputValue>>,
    ) {
        focus.0 = Some(*target)
    }
}
