use bevy::{
    color::palettes::css::{BLACK, WHITE},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
};

use crate::prelude::*;

#[derive(Message, Clone, Debug, Reflect)]
pub struct ConsoleScrollMsg {
    pub message: Pointer<Scroll>,
    pub console_id: Entity,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum ConsoleViewAction {
    Scroll(i32),
    Resize { width: u32, height: u32 },
    JumpToBottom,
}

#[derive(Message, Clone, Debug, Reflect)]
pub struct ConsoleViewMsg {
    action: ConsoleViewAction,
    console_id: Entity,
}
impl ConsoleViewMsg {
    /// Scroll the current start position by the specified amount.
    /// A negative ydelta will scroll the view _up,_ backwards in time,
    /// while a postivie ydelta will scroll the view _down_, forwards in time.
    pub fn scroll(ydelta: i32, console_id: Entity) -> Self {
        Self {
            action: ConsoleViewAction::Scroll(ydelta),
            console_id,
        }
    }

    /// Resizes the view. Units are _characters._
    pub fn resize(width: u32, height: u32, console_id: Entity) -> Self {
        Self {
            action: ConsoleViewAction::Resize { width, height },
            console_id,
        }
    }

    /// Brings the console view's position as far down as it will go.
    pub fn jump_to_bottom(console_id: Entity) -> Self {
        Self {
            action: ConsoleViewAction::JumpToBottom,
            console_id,
        }
    }
}

#[derive(Message, Clone, Debug, Reflect)]
pub struct ConsoleSubmitMsg {
    pub console_id: Entity,
}

#[derive(Message, Debug, Clone)]
pub struct ConsoleWriteMsg {
    pub console_id: Entity,
    pub message: String,
}

#[derive(Message, Debug)]
pub struct ConsoleActionMsg {
    pub console_id: Entity,
    pub input: ConsoleActionSystemInput,
    pub system: ConsoleActionSystem,
}

#[derive(Component, Debug, Reflect, Clone, Deref, DerefMut)]
pub struct ConsolePrompt(pub String);
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
    pub text_font: TextFont,
    pub font_color: Color,
    pub background_color: Color,
}
impl Default for ConsoleUiSettings {
    fn default() -> Self {
        Self {
            text_font: TextFont {
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
                this.text_font.clone(),
                TextColor(this.font_color),
            )
        };
        world.commands().entity(ctx.entity).insert(bundle);
    }
}
