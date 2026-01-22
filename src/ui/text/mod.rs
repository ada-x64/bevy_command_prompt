//! Custom text pipeline implementation.
use crate::prelude::*;

mod data;
mod systems;
use bevy::{render::RenderApp, text::detect_text_needs_rerender, ui_render::RenderUiSystems};
use systems::*;

pub mod prelude {
    pub use super::data::*;
    pub(crate) use super::systems::*;
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        ((update_buffer, update_console_text_layout)
            .after(bevy::text::free_unused_font_atlases_system)
            .before(bevy::asset::AssetEventSystems)
            // these are separate entities.
            .ambiguous_with(detect_text_needs_rerender::<Text2d>)
            .ambiguous_with(detect_text_needs_rerender::<Text>)
            .ambiguous_with(bevy::sprite::update_text2d_layout)
            .ambiguous_with(bevy::sprite::calculate_bounds_text2d),),
    );

    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
        return;
    };
    render_app.add_systems(
        ExtractSchedule,
        extract_console_text_sections.in_set(RenderUiSystems::ExtractText),
    );
}
