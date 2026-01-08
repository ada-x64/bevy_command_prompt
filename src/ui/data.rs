use bevy::{
    color::palettes::css::{BLACK, WHITE},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
};
use ringbuf::HeapRb;

use crate::prelude::*;

#[derive(Message, Event, Clone, Debug)]
pub struct ConsolePrintln {
    pub message: String,
    pub console_id: Entity,
}

#[derive(Message, Clone, Debug, Reflect)]
pub struct ConsoleScrollMsg {
    pub message: Pointer<Scroll>,
    pub console_id: Entity,
}

#[derive(Message, Clone, Debug, Reflect)]
pub struct ConsoleSubmitMsg {
    pub console_id: Entity,
}

#[derive(Component, Deref, DerefMut)]
pub struct ConsoleBuffer(HeapRb<char>);
impl Default for ConsoleBuffer {
    fn default() -> Self {
        Self(HeapRb::new(1e6 as usize)) // 1MB buffer, abt 500 pgs
    }
}
impl ConsoleBuffer {
    pub fn new(capacity: usize) -> Self {
        Self(HeapRb::new(capacity))
    }
}

#[derive(Component, Debug, Reflect, Clone, Default, Deref, DerefMut)]
pub struct ConsoleWriteQueue(Vec<String>);

#[derive(Component, Default, Deref, DerefMut)]
pub struct ConsoleActionQueue(Vec<(ConsoleActionSystemInput, ConsoleActionSystem)>);

#[derive(Component, Debug, Reflect, Clone, Deref, DerefMut)]
pub struct ConsolePrompt(String);
impl Default for ConsolePrompt {
    fn default() -> Self {
        Self("> ".into())
    }
}

// TODO: Console history should be a file.
#[derive(Component, Debug, Reflect, Clone, Default, Deref, DerefMut)]
pub struct ConsoleHistory(Vec<String>);

#[derive(Component, Debug, Reflect, Clone)]
#[component(immutable, on_insert=Self::on_insert)]
#[require(Node)]
pub struct ConsoleUiSettings {
    pub font: TextFont,
    pub font_color: Color,
    pub background_color: Color,
    pub text_layout: TextLayout,
}
impl Default for ConsoleUiSettings {
    fn default() -> Self {
        Self {
            font: TextFont {
                font_size: 12.,
                ..Default::default()
            },
            font_color: WHITE.into(),
            background_color: BLACK.into(),
            text_layout: TextLayout::default(),
        }
    }
}
impl ConsoleUiSettings {
    pub fn on_insert<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        let bundle = {
            let this = world.get::<Self>(ctx.entity).unwrap();
            (
                BackgroundColor(this.background_color),
                this.font.clone(),
                TextColor(this.font_color),
                this.text_layout,
            )
        };
        world.commands().entity(ctx.entity).insert(bundle);
    }
    pub fn line_height(&self) -> f32 {
        match self.font.line_height {
            bevy::text::LineHeight::Px(px) => px,
            bevy::text::LineHeight::RelativeToFont(scale) => self.font.font_size * scale,
        }
    }
}
