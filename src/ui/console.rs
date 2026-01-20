use crate::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input_focus::InputFocus,
    ui::ui_layout_system,
};

#[derive(Component, Debug, Reflect, Clone, Default)]
#[require(
    Node,
    bevy::ui::ContentSize,
    ConsoleUiSettings,
    ConsoleTextLayout,
    ConsoleBuffer,
    ConsoleBufferFlags,
    ConsolePrompt,
    ConsoleHistory
)]
#[component(on_add=Self::on_add)]
pub struct Console {
    pub(crate) input: String,
    pub(crate) cursor: usize,
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
