use enum_iterator::Sequence;
use serde::Serialize;

#[derive(Copy, Clone, PartialEq, Eq, Sequence, Serialize)]
pub enum Race {
    Blessed,
    Monks,
    Felines,
    Navigators,
    Goblins,
    Omar,
    Illusionists,
    Inventors,
    Philosophers,
    Lizards,
    Psychics,
    Moles,
}
