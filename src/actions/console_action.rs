use core::fmt;

use crate::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::mouse::{MouseButton, MouseButtonInput};
use variadics_please::all_tuples;

#[derive(Debug, Clone, Reflect)]
pub enum MatchedInput {
    Key(KeyboardInput),
    Mouse(MouseButtonInput),
    Scroll(isize), //lines
}
impl From<KeyboardInput> for MatchedInput {
    fn from(value: KeyboardInput) -> Self {
        MatchedInput::Key(value)
    }
}
impl From<MouseButtonInput> for MatchedInput {
    fn from(value: MouseButtonInput) -> Self {
        MatchedInput::Mouse(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum ConsoleInput {
    AnyKey,
    AnyCharacter,
    Key(Key),
    Mouse(MouseButton),
    Scroll,
}
impl From<Key> for ConsoleInput {
    fn from(key: Key) -> Self {
        ConsoleInput::Key(key)
    }
}
impl From<MouseButton> for ConsoleInput {
    fn from(button: MouseButton) -> Self {
        ConsoleInput::Mouse(button)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Asset, Reflect)]
pub struct ConsoleActionKeybind {
    pub keys: KeyInput,
    pub modifiers: ModifierInput,
    pub bad_keys: KeyInput,
    pub bad_mods: ModifierInput,
}

impl fmt::Display for ConsoleActionKeybind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<")?;

        let mut parts = Vec::new();

        // Add modifiers
        if !self.modifiers.is_empty() {
            parts.push(format_modifier_input(&self.modifiers));
        }

        // Add keys
        if !self.keys.is_empty() {
            parts.push(format_key_input(&self.keys));
        }

        if parts.is_empty() {
            write!(f, "*")?;
        } else {
            write!(f, "{}", parts.join("-"))?;
        }

        // Add negative constraints
        if !self.bad_mods.is_empty() || !self.bad_keys.is_empty() {
            write!(f, " !")?;
            let mut neg_parts = Vec::new();

            if !self.bad_mods.is_empty() {
                neg_parts.push(format_modifier_input(&self.bad_mods));
            }
            if !self.bad_keys.is_empty() {
                neg_parts.push(format_key_input(&self.bad_keys));
            }

            write!(f, "{}", neg_parts.join("-"))?;
        }

        write!(f, ">")
    }
}

fn format_key_input(input: &KeyInput) -> String {
    if input.is_empty() {
        return String::from("*");
    }

    // Each inner Vec is an OR group, multiple Vecs are ANDed together
    input
        .iter()
        .map(|or_group| {
            if or_group.len() == 1 {
                format_console_input(&or_group[0])
            } else {
                format!(
                    "({})",
                    or_group
                        .iter()
                        .map(format_console_input)
                        .collect::<Vec<_>>()
                        .join("|")
                )
            }
        })
        .collect::<Vec<_>>()
        .join("+")
}

fn format_console_input(input: &ConsoleInput) -> String {
    match input {
        ConsoleInput::AnyKey => String::from("Any"),
        ConsoleInput::AnyCharacter => String::from("Char"),
        ConsoleInput::Key(key) => format!("{:?}", key),
        ConsoleInput::Mouse(button) => format!("{:?}", button),
        ConsoleInput::Scroll => String::from("Scroll"),
    }
}
fn format_modifier_input(input: &ModifierInput) -> String {
    if input.is_empty() {
        return String::from("*");
    }

    input
        .iter()
        .map(|or_group| {
            if or_group.len() == 1 {
                format!("{:?}", or_group[0])
            } else {
                format!(
                    "({})",
                    or_group
                        .iter()
                        .map(|k| { format!("{:?}", k) })
                        .collect::<Vec<_>>()
                        .join("|")
                )
            }
        })
        .collect::<Vec<_>>()
        .join("+")
}
impl ConsoleActionKeybind {
    /// Creates a new [ConsoleAction] with the given keys.
    ///
    /// Accepts flexible input formats:
    /// - Single key: `ConsoleAction::new(Key::Escape)`
    /// - Multiple keys (OR): `ConsoleAction::new([Key::Escape, Key::F1])`
    /// - AND groups: `ConsoleAction::new(([Key::ControlLeft, Key::ControlRight], Key::Escape))`
    ///
    /// Arrays are OR groups, tuples combine them with AND.
    ///
    /// These values are logical keyboard inputs, based on the semantic
    /// character value rather than its position on the keyboard. The
    /// [ConsoleCommand] associated with this [ConsoleAction] will fire when
    /// [ButtonInput::just_pressed] matches the given expression.
    pub fn new(keys: impl Into<KeyInput>) -> Self {
        Self {
            keys: keys.into(),
            ..Default::default()
        }
    }

    /// Adds modifiers.
    /// These values are key code inputs, based on the keys' position on the
    /// keyboard. The [ConsoleCommand] associated with this [ConsoleAction] will
    /// fire when [ButtonInput::pressed] matches the given expression, i.e., if
    /// the modifiers are held.
    pub fn with_modifiers(self, modifiers: impl Into<ModifierInput>) -> Self {
        Self {
            modifiers: modifiers.into(),
            ..self
        }
    }

    pub fn without(self, keys: impl Into<KeyInput>) -> Self {
        Self {
            bad_keys: keys.into(),
            ..self
        }
    }

    pub fn without_modifiers(self, keys: impl Into<ModifierInput>) -> Self {
        Self {
            bad_mods: keys.into(),
            ..self
        }
    }

    fn match_console_input(
        input: &ConsoleInput,
        input_events: &[&KeyboardInput],
        key_input: &ButtonInput<Key>,
        mouse_events: &[&MouseButtonInput],
        mouse_input: &ButtonInput<MouseButton>,
        scroll: isize,
    ) -> Option<MatchedInput> {
        match input {
            ConsoleInput::AnyCharacter => input_events.iter().find_map(|k| {
                if let Key::Character(_) = k.logical_key
                    && key_input.just_pressed(k.logical_key.clone())
                {
                    Some((**k).clone().into())
                } else {
                    None
                }
            }),
            ConsoleInput::AnyKey => input_events.first().map(|k| (**k).clone().into()),
            ConsoleInput::Key(logical_key) => key_input
                .just_pressed(logical_key.clone())
                .then(|| {
                    input_events.iter().find_map(|input| {
                        (input.logical_key == *logical_key).then_some((**input).clone().into())
                    })
                })
                .flatten(),
            ConsoleInput::Mouse(button) => mouse_events.iter().find_map(|m| {
                if m.button == *button && mouse_input.just_pressed(m.button) {
                    Some((**m).into())
                } else {
                    None
                }
            }),
            ConsoleInput::Scroll => (scroll != 0).then_some(MatchedInput::Scroll(scroll)),
        }
    }

    fn match_keys(
        expected_inputs: &[Vec<ConsoleInput>],
        input_events: &[&KeyboardInput],
        mouse_events: &[&MouseButtonInput],
        key_input: &ButtonInput<Key>,
        mouse_input: &ButtonInput<MouseButton>,
        scroll: isize,
    ) -> Vec<MatchedInput> {
        expected_inputs.iter().fold(vec![], |mut accum, or_group| {
            let match_input = |input| {
                Self::match_console_input(
                    input,
                    input_events,
                    key_input,
                    mouse_events,
                    mouse_input,
                    scroll,
                )
            };

            let matched = or_group.iter().find_map(match_input);

            if let Some(key) = matched {
                accum.push(key);
            }
            accum
        })
    }
    fn match_mods(vec: &[Vec<KeyCode>], keys: &ButtonInput<KeyCode>) -> Vec<KeyCode> {
        vec.iter().fold(vec![], |mut res, or_group| {
            let matched = or_group.iter().find(|key| keys.pressed(**key));
            if let Some(key) = matched {
                res.push(*key);
            }
            res
        })
    }

    pub fn to_system_input(
        &self,
        input_events: &[&KeyboardInput],
        mouse_events: &[&MouseButtonInput],
        scroll: isize,
        keys: &ButtonInput<Key>,
        key_codes: &ButtonInput<KeyCode>,
        mouse_input: &ButtonInput<MouseButton>,
        console_id: Entity,
    ) -> Option<ConsoleActionSystemInput> {
        let matches =
            |k| Self::match_keys(k, input_events, mouse_events, keys, mouse_input, scroll);
        let mod_matches = |mods| Self::match_mods(mods, key_codes);

        if !matches(&self.bad_keys).is_empty() || !mod_matches(&self.bad_mods).is_empty() {
            return None;
        }

        let (matched_keys, matched_mods) = (matches(&self.keys), mod_matches(&self.modifiers));
        let keys_ok = matched_keys.len() == self.keys.len();
        let mods_ok = matched_mods.len() == self.modifiers.len();
        if keys_ok && mods_ok {
            Some(ConsoleActionSystemInput {
                console_id,
                matched_input: matched_keys,
                matched_mods,
            })
        } else {
            None
        }
    }
}

// Wrapper types for Into implementations
#[derive(Debug, Deref, DerefMut, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub struct KeyInput(Vec<Vec<ConsoleInput>>);
#[derive(Debug, Deref, DerefMut, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub struct ModifierInput(Vec<Vec<KeyCode>>);

// Helper trait to convert things into OR groups (Vec<Key>)
pub trait IntoKeyGroup {
    fn into_key_group(self) -> Vec<ConsoleInput>;
}

// Single key becomes a group of one
impl<K> IntoKeyGroup for K
where
    K: Into<ConsoleInput>,
{
    fn into_key_group(self) -> Vec<ConsoleInput> {
        vec![self.into()]
    }
}

// Array becomes an OR group
impl<const N: usize, K> IntoKeyGroup for [K; N]
where
    K: Into<ConsoleInput>,
{
    fn into_key_group(self) -> Vec<ConsoleInput> {
        self.into_iter().map(|k| k.into()).collect()
    }
}

// Already a group
impl IntoKeyGroup for Vec<ConsoleInput> {
    fn into_key_group(self) -> Vec<ConsoleInput> {
        self
    }
}

// Now implement KeyInput conversions

// Single key
impl<K> From<K> for KeyInput
where
    K: Into<ConsoleInput>,
{
    fn from(key: K) -> Self {
        KeyInput(vec![vec![key.into()]])
    }
}

// Single array (OR group)
impl<const N: usize, K> From<[K; N]> for KeyInput
where
    K: Into<ConsoleInput>,
{
    fn from(keys: [K; N]) -> Self {
        KeyInput(vec![keys.into_iter().map(|k| k.into()).collect()])
    }
}

// Direct Vec<Vec<Key>> for backward compatibility
impl From<Vec<Vec<ConsoleInput>>> for KeyInput {
    fn from(keys: Vec<Vec<ConsoleInput>>) -> Self {
        KeyInput(keys)
    }
}

// Tuple of groups (AND logic) - each element can be a key or array of keys (OR)
macro_rules! impl_key_input_and_tuple {
    ($($T:ident),*) => {
        impl<$($T: IntoKeyGroup),*> From<($($T,)*)> for KeyInput {
            fn from(tuple: ($($T,)*)) -> Self {
                #[allow(non_snake_case)]
                let ($($T,)*) = tuple;
                KeyInput(vec![$($T.into_key_group()),*])
            }
        }
    };
}

all_tuples!(impl_key_input_and_tuple, 2, 12, T);

// Same pattern for modifiers

pub trait IntoModifierGroup {
    fn into_modifier_group(self) -> Vec<KeyCode>;
}

impl IntoModifierGroup for KeyCode {
    fn into_modifier_group(self) -> Vec<KeyCode> {
        vec![self]
    }
}

impl<const N: usize> IntoModifierGroup for [KeyCode; N] {
    fn into_modifier_group(self) -> Vec<KeyCode> {
        self.into()
    }
}

impl IntoModifierGroup for Vec<KeyCode> {
    fn into_modifier_group(self) -> Vec<KeyCode> {
        self
    }
}

impl From<KeyCode> for ModifierInput {
    fn from(key: KeyCode) -> Self {
        ModifierInput(vec![vec![key]])
    }
}

impl<const N: usize> From<[KeyCode; N]> for ModifierInput {
    fn from(modifiers: [KeyCode; N]) -> Self {
        ModifierInput(vec![modifiers.into()])
    }
}

impl From<Vec<Vec<KeyCode>>> for ModifierInput {
    fn from(modifiers: Vec<Vec<KeyCode>>) -> Self {
        ModifierInput(modifiers)
    }
}

macro_rules! impl_modifier_input_and_tuple {
    ($($T:ident),*) => {
        impl<$($T: IntoModifierGroup),*> From<($($T,)*)> for ModifierInput {
            fn from(tuple: ($($T,)*)) -> Self {
                #[allow(non_snake_case)]
                let ($($T,)*) = tuple;
                ModifierInput(vec![$($T.into_modifier_group()),*])
            }
        }
    };
}

all_tuples!(impl_modifier_input_and_tuple, 2, 12, T);
