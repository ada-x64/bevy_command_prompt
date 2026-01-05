use crate::prelude::*;
use bevy::input::keyboard::Key;
use variadics_please::all_tuples;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ConsoleAction {
    pub keys: Vec<Vec<Key>>,
    pub modifiers: Vec<Vec<KeyCode>>,
    pub without_keys: Vec<Vec<Key>>,
    pub without_modifiers: Vec<Vec<KeyCode>>,
}

impl ConsoleAction {
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
            keys: keys.into().0,
            modifiers: vec![],
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
            modifiers: modifiers.into().0,
            ..self
        }
    }

    pub fn without(self, keys: impl Into<KeyInput>) -> Self {
        Self {
            without_keys: keys.into().0,
            ..self
        }
    }

    pub fn without_modifiers(self, keys: impl Into<ModifierInput>) -> Self {
        Self {
            without_modifiers: keys.into().0,
            ..self
        }
    }

    /// Returns the key vec's only member, or None if there are multiple.
    pub fn get_single(&self) -> Option<Key> {
        if self.keys.len() == 1
            && let Some(vec) = self.keys.first()
            && vec.len() == 1
        {
            vec.first().cloned()
        } else {
            None
        }
    }
}

// Wrapper types for Into implementations
pub struct KeyInput(Vec<Vec<Key>>);
pub struct ModifierInput(Vec<Vec<KeyCode>>);

// Helper trait to convert things into OR groups (Vec<Key>)
pub trait IntoKeyGroup {
    fn into_key_group(self) -> Vec<Key>;
}

// Single key becomes a group of one
impl IntoKeyGroup for Key {
    fn into_key_group(self) -> Vec<Key> {
        vec![self]
    }
}

// Array becomes an OR group
impl<const N: usize> IntoKeyGroup for [Key; N] {
    fn into_key_group(self) -> Vec<Key> {
        self.into()
    }
}

// Already a group
impl IntoKeyGroup for Vec<Key> {
    fn into_key_group(self) -> Vec<Key> {
        self
    }
}

// Now implement KeyInput conversions

// Single key
impl From<Key> for KeyInput {
    fn from(key: Key) -> Self {
        KeyInput(vec![vec![key]])
    }
}

// Single array (OR group)
impl<const N: usize> From<[Key; N]> for KeyInput {
    fn from(keys: [Key; N]) -> Self {
        KeyInput(vec![keys.into()])
    }
}

// Direct Vec<Vec<Key>> for backward compatibility
impl From<Vec<Vec<Key>>> for KeyInput {
    fn from(keys: Vec<Vec<Key>>) -> Self {
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
