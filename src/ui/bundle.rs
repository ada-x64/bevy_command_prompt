use crate::ui::events::on_scroll_handler;

use crate::prelude::*;
use bevy::{
    color::palettes::tailwind,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
};
use bevy_simple_text_input::TextInput;

// <ConsoleWrapper>
//   <ConsoleBody>
//     <ConsoleBodyText />
//     <ConsoleBodyText />
//     <ConsoleBodyText />
//   </ConsoleBody>
//   <ConsoleInput />
// </ConsoleWrapper>

#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
#[component(on_add = ConsoleInput::add)]
pub struct ConsoleInput;
impl ConsoleInput {
    fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        let mut q = world.try_query::<&ConsoleFontSize>().unwrap();
        let font_size = q.single(&world).unwrap();
        let prompt = world.resource::<ConsoleInputPrompt>();

        let prompt_bundle = (
            Text::new(prompt.0.clone()),
            Node {
                width: Val::Px(font_size.width * prompt.len() as f32),
                height: Val::Px(font_size.height),
                ..Default::default()
            },
        );

        let bundle = (
            Name::new("ConsoleInput"),
            Node {
                height: Val::Px(font_size.height),
                display: Display::Flex,
                ..Default::default()
            },
            children![prompt_bundle, (TextInput, ConsoleInputValue)],
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

#[derive(Component, Debug, Reflect, Clone, Copy)]
#[component(on_add=ConsoleWrapper::add)]
#[require(ConsoleFontSize, ConsoleBackground)]
pub struct ConsoleWrapper {
    left: Val,
    right: Val,
    top: Val,
    bottom: Val,
    /// Number of characters to display per line
    width: u32,
    /// Number of lines to display
    height: u32,
}
impl Default for ConsoleWrapper {
    fn default() -> Self {
        Self {
            left: Val::Vw(20.),
            right: Default::default(),
            top: Val::Vh(20.),
            bottom: Default::default(),
            width: 50,
            height: 10,
        }
    }
}
impl ConsoleWrapper {
    fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        let data = {
            let entt = world.get_entity(ctx.entity).unwrap();
            *entt.get::<Self>().unwrap()
        };
        let mut q = world.try_query::<&ConsoleFontSize>().unwrap();
        let font_size = q.single(&world).unwrap();

        let bundle = (
            BackgroundColor(tailwind::SLATE_950.with_alpha(0.75).into()),
            Name::new("Console"),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                flex_wrap: FlexWrap::NoWrap,
                overflow: Overflow::hidden(),
                left: data.left,
                right: data.right,
                top: data.top,
                bottom: data.bottom,
                width: Val::Px(data.width as f32 * font_size.width),
                height: Val::Px(data.height as f32 * font_size.height),
                ..Default::default()
            },
            children![ConsoleInput, ConsoleBody],
        );
        world.commands().entity(ctx.entity).insert(bundle);
    }
}
