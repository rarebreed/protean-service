//! Foundational data types for all of the engine

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// All the facets that define a Character
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CharacteristicType {
    /// Characteristics which are independent
    PrimaryCharacteristics,
    /// Derived characteristics from PrimaryCharacteristics like courage and reaction
    SecondaryCharacteristics,
    /// Primary characteristics which are mental in nature like insight or anaytics
    MentalPrimaryCharacteristics,
    /// Characteristics that influence people's behavior or attitude
    SocialCharacteristics,
    /// Inner psychological traits
    Psyche,
    /// Values and beliefs
    Principle,
    /// Equipment for the character
    Equipment,
}

/// A Trait is a trait for specifying what you can do with generic data
///
/// Many values that make up a character, or other entities like weapons, vehicles, or gadgets are built by implementing
/// this trait
pub trait Trait {
    /// The value is a _score_ defining some value on a Range.
    type Value;
    /// The Range is defined as a Tuple of a (min, max) or an enum
    type Range;
    /// The parent type or category of the Traut
    type Parent;

    /// the name is a read-only field, so there's only a getter
    fn name(&self) -> &str;

    /// sets the value of the trait
    fn set_value(&mut self, val: Self::Value);
    /// gets the value
    fn value(&self) -> &Self::Value;
    /// sets the value range
    fn set_value_range(&mut self, range: Ranged<Self::Range>);
    /// gets the range
    fn value_range(&self) -> &Ranged<Self::Range>;

    /// gets he Parent category of a Trait
    fn parent(&self) -> &Self::Parent;
    /// sets the parent category
    fn set_parent(&mut self, parent: Self::Parent);

    /// Some values are determined randomnly, but there is always a cost/value associated with the trait/value
    fn cost<F>(val: Self::Value, fun: F) -> f32
    where
        F: Fn(Self::Value) -> f32;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Specifies the range that an Attribute can take
pub enum Ranged<T> {
    /// An Attribute that varies across a range of values with a min and max (inclusive)
    Spectrum {
        /// minimum value
        min: T,
        /// maximum value
        max: T,
    },
    /// An Attribute which can take on an enumerated selection of values
    /// FIXME: I kind of need a higher kinded type here.  Ie, Categorized(DamageEffects)
    Categorized(T),
    /// The Attribute can only take on a singular value
    Absolute(T),
}

/// An Attribute is used to define some data point
///
/// The most common use of Attributes is to define a character's Characteristics, for example speed or wit.  It is also
/// commonly used to define other statistics, like a weapon's damage, or weight.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attribute<T, R>
where
    T: Into<f64>,
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
    T: Into<f64>,
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
}

impl<T, R> Trait for Attribute<T, R>
where
    T: Into<f64>,
{
    type Value = T;
    type Range = R;
    type Parent = CharacteristicType;

    fn name(&self) -> &str {
        &self.name
    }

    fn set_value(&mut self, val: Self::Value) {
        self.value = val;
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn set_value_range(&mut self, range: Ranged<Self::Range>) {
        self.range = range;
    }

    fn value_range(&self) -> &Ranged<Self::Range> {
        &self.range
    }

    fn parent(&self) -> &Self::Parent {
        &self.parent
    }

    fn set_parent(&mut self, parent: Self::Parent) {
        self.parent = parent;
    }

    fn cost<F>(val: Self::Value, fun: F) -> f32
    where
        F: Fn(Self::Value) -> f32,
    {
        fun(val)
    }
}

/// Sum type for visual electro-magnetic spectrum
pub enum EMSpectrum {
    /// Human range of vision
    Normal,
    /// Infrared
    IR,
    /// Ultraviolet
    UV,
    /// X-ray
    Xray,
}

/// Energy frequencies for sound
pub enum AuditorySpectrum {
    /// below human hearing about 20hz
    Subsonic,
    /// normal range human of hearing
    Normal,
    /// above human hearing about 20,000hz
    Ultrasonic,
}

/// Sum types that define the 5 human senses
pub enum Senses {
    /// sight
    Visual(EMSpectrum),
    /// hearing
    Audio(AuditorySpectrum),
    /// smelling
    Olfactory,
    /// touch
    Tactile,
    /// taste
    Taste,
}

pub mod characteristics;
pub mod engine;
pub mod equipment;
mod io;
