use crate::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    text::LineHeight,
};

// TODO: Virtual scrolling requires custom scroll bar.
#[derive(Component, Debug, Clone, Reflect, Copy)]
#[require(
    Node,
    ConsoleBuffer,
    Console,
    ConsolePrompt,
    ComputedConsoleTextBlock,
    TextColor,
    LineHeight,
    // copying `Text`s homework
    FontHinting::Disabled
)]
#[component(on_insert=Self::on_insert)]
pub struct ConsoleBufferView {
    pub console_id: Entity,
    pub start: usize,
    pub range: usize,
}
impl ConsoleBufferView {
    pub fn new(console_id: Entity) -> Self {
        // range tbd after initial render i.e. once ui size is determined
        Self {
            console_id,
            start: 0,
            range: 0,
        }
    }
    fn on_insert(mut world: DeferredWorld, ctx: HookContext) {
        let mut entt = world.entity_mut(ctx.entity);
        let mut block = r!(entt.get_mut::<ComputedConsoleTextBlock>());
        block.trigger_rerender();
    }
    pub(crate) fn jump_to_bottom(self) -> Self {
        Self { start: 0, ..self }
    }

    pub(crate) fn resize(self, container_height: f32, line_height: f32) -> Self {
        let range = (container_height / line_height) as usize;
        ConsoleBufferView {
            start: 0,
            range,
            ..self
        }
    }
    pub(crate) fn scroll(self, value: isize, buffer: &ConsoleBuffer) -> Self {
        let buffer_size = buffer.line_count();
        if buffer_size <= self.range {
            return self;
        }
        let start = self
            .start
            .saturating_add_signed(value)
            .min(buffer_size - self.range);
        Self { start, ..self }
    }
}
