use crate::ui::events::on_scroll_handler;

use crate::prelude::*;
use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
use bevy_simple_text_input::TextInput;

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
                height: Val::Px(settings.line_height()),
                ..Default::default()
            },
        );

        let bundle = (
            Name::new("ConsoleInput"),
            Node {
                height: Val::Px(settings.line_height()),
                display: Display::Flex,
                ..Default::default()
            },
            children![
                prompt_bundle,
                (
                    TextInput,
                    bevy_simple_text_input::TextInputTextFont(settings.font.clone()),
                    ConsoleInputValue
                )
            ],
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
                overflow: Overflow {
                    x: OverflowAxis::Clip,
                    y: OverflowAxis::Scroll,
                },
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
                flex_wrap: FlexWrap::NoWrap,
                overflow: Overflow::hidden(),
                width: Val::Px(settings.width as f32 * settings.font.font_size),
                height: Val::Px(settings.height as f32 * settings.line_height()),
                ..Default::default()
            },
            BackgroundColor(settings.background_color),
            children![ConsoleInput, ConsoleBody],
        );
        world.commands().entity(ctx.entity).insert(bundle);
    }
}
