use bevy::{
    color::palettes::css::{BLACK, WHITE},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
};
use ringbuf::{HeapRb, traits::*};

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

#[derive(Debug, thiserror::Error)]
pub enum ConsoleBufferError {
    #[error("The attempted write is larger than the buffer capacity.")]
    Overflow,
    #[error("Failed to clear first buffer line.")]
    WriteError,
}

/// A heap-allocated ring buffer containing the characters stored in the
/// buffer's STDOUT. By default, will cache 1MiB of data.
#[derive(Component)]
pub struct ConsoleBuffer(HeapRb<char>);
impl std::fmt::Debug for ConsoleBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConsoleBuffer")
            .field("capacity", &self.0.capacity())
            .field("vacant_len", &self.0.vacant_len())
            .field("occupied_len", &self.0.occupied_len())
            .finish()
    }
}
impl Default for ConsoleBuffer {
    fn default() -> Self {
        Self(HeapRb::<char>::new(1048576)) // 1MiB
    }
}
impl ConsoleBuffer {
    pub fn new(size: usize) -> Self {
        Self(HeapRb::<char>::new(size))
    }
    /// Writes to the buffer. If necessary, this function will erase the first
    /// line string in the ring buffer. Will return an error if the string to
    /// write is larger than the buffer capacity, or if the overwriting fails.
    pub fn write(&mut self, val: &str) -> Result<usize, ConsoleBufferError> {
        if val.len() > self.0.capacity().into() {
            return Err(ConsoleBufferError::Overflow);
        }
        let num_overwritten = val
            .chars()
            .filter(|c| self.0.push_overwrite(*c).is_some())
            .count();
        if num_overwritten > 0 {
            let _ = self.0.pop_iter().take_while(|c| *c != '\n');
            let ok = self.0.try_pop().filter(|c| *c == '\n').is_some();
            if !ok {
                return Err(ConsoleBufferError::WriteError);
            }
        }
        Ok(num_overwritten)
    }
    /// Counts the number of newlines in the buffer.
    pub fn line_count(&self) -> usize {
        self.0.iter().filter(|c| **c == '\n').count()
    }
    /// Collects the ring buffer into a vector of characters. This function does
    /// not allocate, so it does not return Strings.
    pub fn as_lines(&self) -> Vec<Vec<&char>> {
        self.0.iter().fold(vec![vec![]], |mut acc, c| {
            let mut last = acc.last_mut().unwrap();
            if let Some(l) = last.last()
                && **l == '\n'
            {
                acc.push(vec![]);
                last = acc.last_mut().unwrap();
            }
            last.push(c);
            acc
        })
    }
    /// Removes all items from the buffer. Returns the number of dropped items.
    pub fn clear(&mut self) -> usize {
        self.0.clear()
    }
}
#[test]
fn test_buffer() {
    let size = 256;
    let test_val = "hello!\n";
    let mut buffer = ConsoleBuffer::new(size);
    buffer.write(test_val).unwrap();
    assert_eq!(buffer.line_count(), 1);
    let lines: Vec<String> = buffer
        .as_lines()
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect();
    assert_eq!(lines, vec![test_val.to_string()]);
    let mut count = 0;
    while let Ok(size) = buffer.write(format!("{count}\n").as_str()) {
        // properly writes
        let found = buffer
            .as_lines()
            .into_iter()
            .find(|str| str.iter().cloned().collect::<String>() == format!("{count}\n"));
        assert!(found.is_some());
        count += 1;
        if size > 0 {
            // properly overwrites
            let found = buffer
                .as_lines()
                .into_iter()
                .find(|str| str.iter().cloned().collect::<String>() == test_val);
            assert!(found.is_none());
            break;
        }
    }
}

/// Queue with lines to write to the [ConsoleBuffer]
#[derive(Component, Debug, Reflect, Clone, Default, Deref, DerefMut)]
pub struct ConsoleWriteQueue(Vec<String>);

#[derive(Component, Default, Deref, DerefMut)]
pub struct ConsoleActionQueue(Vec<(ConsoleActionSystemInput, ConsoleActionSystem)>);

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
