use crate::prelude::*;
use bevy::{
    color::palettes::css::{BLACK, WHITE},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input_focus::InputFocus,
    ui::ui_layout_system,
};

#[derive(Component, Debug, Reflect, Clone)]
#[component(immutable, on_insert=Self::on_insert)]
#[require(Console, Node)]
pub struct ConsoleUiSettings {
    pub font: TextFont,
    pub font_color: Color,
    pub background_color: Color,
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

// TODO: Virtual scrolling requires custom scroll bar.
#[derive(Component, Debug, Clone, Reflect, Copy)]
#[require(Node, Text, ConsoleUiSettings, Console, ScrollPosition)]
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
        let range = (line_height / container_height) as usize - prompt_lines;
        ConsoleBufferView {
            start: lines - range,
            range,
            ..self
        }
    }
    pub fn jump_to_bottom(self, lines: usize) -> Self {
        Self {
            start: lines - self.range,
            ..self
        }
    }
    pub(crate) fn on_resize(
        q: Query<
            (
                Entity,
                &ComputedNode,
                &ConsoleUiSettings,
                &Console,
                &ConsoleBufferView,
            ),
            Changed<ComputedNode>,
        >,
        mut commands: Commands,
    ) {
        for (entity, node, settings, console, view) in q {
            commands.entity(entity).insert(view.resize(
                node.size().y,
                settings.line_height(),
                console.buffer.lines().count(),
                console.prompt.lines().count(),
            ));
        }
    }
}

#[derive(Component, Debug, Reflect, Clone)]
#[require(Node, ConsoleUiSettings)]
#[component(on_add=Self::on_add)]
pub struct Console {
    pub(crate) buffer: String,
    pub(crate) input: String,
    pub prompt: String,
    pub(crate) history: Vec<String>,
}
impl Default for Console {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            input: Default::default(),
            prompt: "> ".into(),
            history: Default::default(),
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
        focus.0 = Some(trigger.entity);
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
