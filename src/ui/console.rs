use crate::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input_focus::InputFocus,
};

#[derive(Component, Debug, Reflect, Clone, Default)]
#[require(
    Node,
    ConsoleUiSettings,
    ConsoleTextLayout,
    ConsoleBuffer,
    ConsoleBufferFlags,
    ConsolePrompt,
    ConsoleHistory,
    ConsoleInputText,
    TextFont
)]
#[component(on_add=Self::on_add)]
pub struct Console;
impl Console {
    pub(crate) fn on_add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
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
            ConsoleBufferView::new(ctx.entity),
            TextFont {
                font_size: 12.,
                ..Default::default()
            },
        );
        world
            .commands()
            .entity(ctx.entity)
            .insert(bundle)
            .observe(Self::on_click)
            .observe(Self::on_scroll);
    }
    fn on_click(trigger: On<Pointer<Click>>, mut focus: ResMut<InputFocus>) {
        focus.set(trigger.entity);
    }

    fn on_scroll(trigger: On<Pointer<Scroll>>, mut commands: Commands) {
        commands.write_message(ConsoleScrollMsg {
            message: trigger.event().clone(),
            console_id: trigger.entity,
        });
    }
}

/// The console's input, excluding the prompt.
#[derive(Component, Debug, Reflect, Default)]
#[require(ConsoleBuffer)]
pub struct ConsoleInputText {
    pub(crate) text: String,
    cursor: usize,
    pub(crate) anchor: usize,
}
impl ConsoleInputText {
    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor = self.text.ceil_char_boundary(pos);
    }
    pub fn move_cursor(&mut self, pos: isize) -> usize {
        self.cursor = self
            .text
            .ceil_char_boundary(self.cursor.saturating_add_signed(pos));
        self.cursor
    }
    pub fn cursor(&self) -> usize {
        self.text.ceil_char_boundary(self.cursor)
    }
}

pub fn update_console_input_text(
    q: Query<(&mut ConsoleBuffer, &ConsoleInputText, &ConsolePrompt), Changed<ConsoleInputText>>,
) {
    for (mut buffer, input, prompt) in q {
        buffer.write_at(input.anchor, &prompt.0).unwrap();
        buffer
            .write_at(input.anchor + prompt.0.len(), &input.text)
            .unwrap();
    }
}
