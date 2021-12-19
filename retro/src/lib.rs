use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CharacteristicType {
    PrimaryCharacteristics,
    SecondaryCharacteristics,
    MentalPrimaryCharacteristics,
    SocialCharacteristics,
    Psyche,
    Principle,
}
/// A Trait is a trait for specifying what you can do with generic data for many values that make up a character, or
/// other entities like weapons, vehicles, or gadgets
pub trait Trait {
    type Value;
    type Range;
    type Parent;

    /// the name is a read-only field, so there's only a getter
    fn name(self: &Self) -> &str;

    /// The value is a _score_ defining some value on a spectrum.  The spectrum has a range defined as a Tuple of a
    /// minimum and a maximum (if the upper has a value), or an enum defining possible values
    fn set_value(self: &mut Self, val: Self::Value) -> ();
    fn value(self: &Self) -> &Self::Value;
    fn set_value_range(self: &mut Self, range: Ranged<Self::Range>) -> ();
    fn value_range(self: &Self) -> &Ranged<Self::Range>;

    /// The Parent category of a Trait
    fn parent(self: &Self) -> &Self::Parent;
    fn set_parent(self: &mut Self, parent: Self::Parent) -> ();

    fn cost(val: Self::Value) -> f32;
}

#[derive(Serialize, Deserialize, Clone)]
/// Specified the range that an Attribute can take
pub enum Ranged<T> {
    /// An Attribute that varies across a range of values with a min and max (inclusive)
    Spectrum { min: T, max: T },
    /// An Attribute which can take on an enumerated selection of values
    /// FIXME: Not sure if rust can specify a type bound that T must be of type enum
    Categorized(T),
    /// The Attribute can only take on a singular value
    Absolute(T),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Attribute<T, R>
where
    T: Into<f32>,
{
    name: String,
    value: T,
    range: Ranged<R>,
    parent: CharacteristicType,
}

/// An Attribute is the most basic building block for nearly all game data.
///
/// # What is built on Attributes
///
/// There are many things which are composed of Attributes, including, but not limited to:
/// - Characteristics like force, memory, insight, etc
/// - Psyche traits like selfishness or adaptability
/// - Principles like honor or duty
/// -
impl<T, R> Attribute<T, R>
where
    T: Into<f32>,
{
    /// A default kind of Attribute mostly used for Character Attribute
    pub fn new(name: String, value: T, parent: CharacteristicType, range: Ranged<R>) -> Self {
        Attribute {
            name,
            value,
            range,
            parent,
        }
    }

    pub fn cost(self: &Self) -> f32 {
        0.0
    }
}

impl<T, R> Trait for Attribute<T, R>
where
    T: Into<f32>,
{
    type Value = T;
    type Range = R;
    type Parent = CharacteristicType;

    fn name(self: &Self) -> &str {
        &self.name
    }

    fn set_value(self: &mut Self, val: Self::Value) -> () {
        self.value = val;
    }

    fn value(self: &Self) -> &Self::Value {
        &self.value
    }

    fn set_value_range(self: &mut Self, range: Ranged<Self::Range>) -> () {
        self.range = range;
    }

    fn value_range(self: &Self) -> &Ranged<Self::Range> {
        &self.range
    }

    fn parent(self: &Self) -> &Self::Parent {
        &self.parent
    }

    fn set_parent(self: &mut Self, parent: Self::Parent) -> () {
        self.parent = parent;
    }

    fn cost(val: Self::Value) -> f32 {
        0.0
    }
}

pub enum EMSpectrum {
    Normal,
    IR,
    UV,
    Xray,
}

pub enum AuditorySpectrum {
    Infrasound,
    Normal,
    Ultrasound,
}

pub enum Senses {
    Visual(EMSpectrum),
    Audio(AuditorySpectrum),
    Olfactory,
    Tactile,
    Taste,
}

pub mod characteristics;
pub mod equipment;
pub mod tasks;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
