use crate::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input_focus::InputFocus,
    text::ComputedTextBlock,
    ui::ui_layout_system,
};

// TODO: Virtual scrolling requires custom scroll bar.
#[derive(Component, Debug, Clone, Reflect, Copy)]
#[require(Node, Text)]
#[component(on_insert=Self::on_insert)]
pub struct ConsoleBufferView {
    pub console_id: Entity,
    pub start: usize,
    pub range: usize,
}
impl ConsoleBufferView {
    fn new(console_id: Entity) -> Self {
        // range tbd after initial render i.e. once ui size is determined
        Self {
            console_id,
            start: 0,
            range: 0,
        }
    }
    // TODO: These next two functions need to be rewritten or removed in order to facilitate the new pipeline.
    fn on_insert(mut world: DeferredWorld, ctx: HookContext) {
        let text = {
            let view = world.get::<ConsoleBufferView>(ctx.entity).unwrap();
            let console = world.get::<Console>(ctx.entity).unwrap();
            view.text(console)
        };
        world.commands().entity(ctx.entity).insert(text);
    }
    fn text(&self, console: &Console) -> impl Bundle {
        let view = console
            .buffer
            .lines()
            .skip(self.start)
            .take(self.range)
            .collect::<Vec<&str>>()
            .join("\n");
        Text(format!("{view}\n{}{}", console.prompt, console.input))
    }
    fn resize(
        self,
        container_height: f32,
        line_height: f32,
        lines: usize,
        prompt_lines: usize,
    ) -> Self {
        let range = ((container_height / line_height) as usize).saturating_sub(prompt_lines);
        ConsoleBufferView {
            start: lines.saturating_sub(range),
            range,
            ..self
        }
    }
    pub fn jump_to_bottom(self, prompt: &ConsolePrompt, computed_text: &ComputedTextBlock) -> Self {
        let count = computed_text.buffer().0.layout_runs().count();
        let prompt_size = prompt.lines().count();
        let start = count.saturating_sub(self.range).saturating_add(prompt_size);
        Self { start, ..self }
    }
    pub(crate) fn on_resize(
        q: Query<
            (
                Entity,
                &ComputedNode,
                &ConsoleUiSettings,
                &ConsolePrompt,
                &ComputedTextBlock,
                &ConsoleBufferView,
            ),
            Or<(Changed<ComputedNode>, Added<ConsoleBufferView>)>,
        >,
        mut commands: Commands,
    ) {
        for (entity, node, settings, prompt, block, view) in q {
            let new_view = view.resize(
                node.size().y,
                settings.line_height(),
                block.buffer().layout_runs().count(),
                prompt.lines().count(),
            );
            commands.entity(entity).insert(new_view);
        }
    }
}

#[derive(Component, Debug, Reflect, Clone)]
#[require(
    Node,
    ConsoleUiSettings,
    ConsoleBuffer,
    ConsoleWriteQueue,
    ConsolePrompt,
    ConsoleHistory
)]
#[component(on_add=Self::on_add)]
pub struct Console {
    /// raw output buffer
    /// to get the actual formatted buffer string (e.g. for buffer view)
    /// get the [bevy::text::ComputedTextBlock::buffer] for this entity.
    pub(crate) input: String,
    pub(crate) cursor: usize,
}
impl Default for Console {
    fn default() -> Self {
        Self {
            input: Default::default(),
            cursor: 0,
        }
    }
}
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

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        ConsoleBufferView::on_resize.after(ui_layout_system),
    );
}
