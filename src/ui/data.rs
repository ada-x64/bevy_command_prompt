use bevy::color::palettes::css::{BLACK, WHITE};

use crate::prelude::*;

#[derive(Component, Default, Debug, Reflect, Clone, Copy)]
#[component(on_add=Console::add)]
pub struct Console;

#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
pub struct ConsoleBodyTextWrapper;
#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
pub struct ConsoleBodyText;

/// TODO: Need to reactively rerender when this changes.
/// The main settings for the console UI.
/// Positioning etc. is handled _outside_ this crate.
#[derive(Resource, Debug, Reflect, Clone)]
#[reflect(Resource)]
pub struct ConsoleUiSettings {
    pub prompt: String,
    pub font: TextFont,
    pub font_color: Color,
    pub background_color: Color,
}
impl Default for ConsoleUiSettings {
    fn default() -> Self {
        Self {
            prompt: "> ".into(),
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
    pub fn line_height(&self) -> f32 {
        match self.font.line_height {
            bevy::text::LineHeight::Px(px) => px,
            bevy::text::LineHeight::RelativeToFont(scale) => self.font.font_size * scale,
        }
    }
    pub fn text(&self, value: impl ToString) -> impl Bundle {
        (self.font.clone(), Text(value.to_string()))
    }
}

/// Marker struct. Get the sibling TextInput's value.
#[derive(Component, Debug, Reflect, Clone, Copy)]
pub struct ConsoleInputValue;

#[derive(Message, Event, Clone, Debug)]
pub struct ConsolePrint(pub String);
#[derive(Event, Clone, Debug)]
pub struct ClearConsole;
