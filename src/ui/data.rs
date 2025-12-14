use bevy::{
    color::palettes::tailwind,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
};

use crate::prelude::*;

#[derive(Component, Default, Debug, Reflect, Clone, Copy)]
#[require(ConsoleWrapper, ConsoleFontSize)]
pub struct Console;

#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
pub struct ConsoleBodyTextWrapper;
#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
pub struct ConsoleBodyText;

#[derive(Resource, Debug, Reflect, Clone, Deref)]
pub struct ConsoleInputPrompt(pub String);
impl Default for ConsoleInputPrompt {
    fn default() -> Self {
        Self("> ".into())
    }
}
/// Marker struct. Get the sibling TextInput's value.
#[derive(Component, Debug, Reflect, Clone, Copy)]
pub struct ConsoleInputValue;

#[derive(Message, Event, Clone, Debug)]
pub struct AppendToConsole(pub String);
#[derive(Event, Clone, Debug)]
pub struct ClearConsole;

/// Width, height in pixels.
#[derive(Component, Debug, Reflect, Clone, Copy)]
pub struct ConsoleFontSize {
    pub width: f32,
    pub height: f32,
}
impl Default for ConsoleFontSize {
    fn default() -> Self {
        Self {
            width: 12.,
            height: 24.,
        }
    }
}

#[derive(Component, Debug, Reflect, Clone)]
pub struct ConsoleFont(pub Handle<Font>);

#[derive(Component, Debug, Reflect, Clone, Copy)]
pub struct ConsoleTextColor(pub Color);
impl Default for ConsoleTextColor {
    fn default() -> Self {
        Self(tailwind::SLATE_100.into())
    }
}

#[derive(Component, Debug, Reflect, Clone, Copy)]
#[component(on_insert=ConsoleBackground::add)]
pub struct ConsoleBackground(pub Color);
impl Default for ConsoleBackground {
    fn default() -> Self {
        Self(tailwind::SLATE_950.with_alpha(75.).into())
    }
}
impl ConsoleBackground {
    fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        let data = {
            let entt = world.get_entity(ctx.entity).unwrap();
            *entt.get::<Self>().unwrap()
        };
        world
            .commands()
            .entity(ctx.entity)
            .insert(BackgroundColor(data.0));
    }
}
