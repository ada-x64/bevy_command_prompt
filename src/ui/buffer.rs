use crate::prelude::*;
use ringbuf::{HeapRb, traits::*};

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
            .field("line_count", &self.line_count())
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
    /// Collects the ring buffer into a vec of vecs of chars. This function does
    /// not allocate, so it does not return Strings. Note that the '\n' characters are _not_ attached.
    pub fn as_lines(&self) -> Vec<Vec<&char>> {
        let mut outer: Vec<Vec<&char>> = vec![];
        let mut iter = self.0.iter().peekable();
        let iter = iter.by_ref();
        while iter.peek().is_some() {
            let val: Vec<&char> = iter.take_while(|c| **c != '\n').collect();
            if !val.is_empty() {
                outer.push(val);
            }
        }
        outer
    }
    /// Returns the the specified range. Start index is at the _end_ of the buffer. Returns results reversed so that
    /// the last visible line is at the start of the vec.
    pub fn range(&self, start: usize, range: usize) -> Vec<Vec<&char>> {
        let mut iter = self.0.iter().peekable();
        let iter = iter.by_ref();
        iter.rev()
            .skip(start)
            .take(range)
            .fold(vec![], |mut accum, char| {
                let needs_new_vec = *char == '\n';
                if needs_new_vec {
                    accum.push(vec![]);
                } else if let Some(last) = accum.last_mut() {
                    last.insert(0, char);
                }
                accum
            })
    }
    /// Collects the ring buffer into a vec of vecs of chars, hard wrapped at the specified width. This function does
    /// not allocate, so it does not return Strings. Note that the '\n' characters are _not_ attached.
    pub fn as_wrapped_lines(&self, width: usize) -> Vec<Vec<&char>> {
        let mut outer = vec![];
        let mut iter = self.0.iter().peekable();
        let iter = iter.by_ref();
        while iter.peek().is_some() {
            let mut count = 0;
            let mut inner = vec![];
            while count < width {
                count += 1;
                if let Some(val) = iter.peek()
                    && **val != '\n'
                {
                    inner.push(iter.next().unwrap());
                }
            }
            outer.push(inner);
        }
        outer
    }
    /// Removes all items from the buffer. Returns the number of dropped items.
    pub fn clear(&mut self) -> usize {
        self.0.clear()
    }
}
#[test]
fn test_wrapped_lines() {
    let size = 256;
    let width = 10;
    let test_val = "hello!\nthis line is more than 10 characters.\n";
    let mut buffer = ConsoleBuffer::new(size);
    buffer.write(test_val).unwrap();
    let lines = buffer
        .as_wrapped_lines(width)
        .into_iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<_>>();
    assert_eq!(
        lines,
        vec![
            "hello!",
            "this line ",
            "is more th",
            "an 10 char",
            "acters."
        ]
    );
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
    assert_eq!(lines, vec!["hello!".to_string()]);
    let mut count = 0;
    while let Ok(size) = buffer.write(format!("{count}\n").as_str()) {
        // properly writes
        let found = buffer
            .as_lines()
            .into_iter()
            .find(|str| str.iter().cloned().collect::<String>() == format!("{count}"));
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
