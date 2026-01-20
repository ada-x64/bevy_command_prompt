use bevy::text::{CosmicBuffer, TextEntity, TextLayoutInfo};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Component, Default, Reflect, Debug)]
pub struct ConsoleBufferFlags {
    pub(crate) needs_recompute: bool,
    pub(crate) needs_measure_fn: bool,
}

#[derive(Component, Default, Reflect, Debug)]
#[require(TextLayoutInfo)]
pub struct ConsoleTextLayout {
    pub linebreak: LineBreak,
}

/// Ideally, this would just be a [bevy::text::ComputedTextBlock], but it's fields are currently private.
#[derive(Component, Debug, Clone)]
pub struct ComputedConsoleTextBlock {
    pub(crate) buffer: CosmicBuffer,
    pub(crate) needs_rerender: bool,
    pub(crate) entities: SmallVec<[TextEntity; 1]>,
}

impl ComputedConsoleTextBlock {
    pub fn buffer(&self) -> &CosmicBuffer {
        &self.buffer
    }
    pub fn trigger_rerender(&mut self) {
        self.needs_rerender = true;
    }
}
impl Default for ComputedConsoleTextBlock {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            needs_rerender: true,
            entities: Default::default(),
        }
    }
}
