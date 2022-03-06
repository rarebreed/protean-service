//! Modules for the retro table top role playing game

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// The differnt characteristics that define a Character
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CharacteristicType {
    /// primary characteristics (eg power, constitution)
    PrimaryCharacteristics,
    /// secondary characteristics which are either derived or of not as common use
    SecondaryCharacteristics,
    /// characteristics that are of the mind in nature
    MentalPrimaryCharacteristics,
    /// characteristics which relate to ability to interact with others
    SocialCharacteristics,
    /// characteristics that define ideas to guide a caharcter's life
    Principle,
    /// the behaviours and quirks of a character
    Personality,
    /// gear and external devices
    Equipment,
}

/// A Trait is a trait for specifying what you can do with generic data
///
/// Many values that make up a character, or other entities like weapons, vehicles, or gadgets are built by implementing
/// this trait
pub trait Trait {
    /// the actual score or rating of a Trait
    type Value;
    /// The values that the Trait can take on
    type Range;
    /// What grouping (eg Characteristic, Equipment, etc) the Trait belongs to
    type Parent;

    /// the name is a read-only field, so there's only a getter
    fn name(self: &Self) -> &str;

    /// The value is a _score_ defining some value on a Range.  The Range has a range defined as a Tuple of a
    /// minimum and a maximum (if the upper has a value), or an enum defining possible values
    fn set_value(self: &mut Self, val: Self::Value) -> ();
    /// retrieves the Value
    fn value(self: &Self) -> &Self::Value;
    /// Sets the Range
    fn set_value_range(self: &mut Self, range: Ranged<Self::Range>) -> ();
    /// Retrieves the Range
    fn value_range(self: &Self) -> &Ranged<Self::Range>;

    /// The Parent category of a Trait
    fn parent(self: &Self) -> &Self::Parent;
    /// Sets the Parent
    fn set_parent(self: &mut Self, parent: Self::Parent) -> ();

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
        /// The minimum value in the spectrum (inclusive)
        min: T, 
        /// The maximumu value of the spectrum (inclusive)
        max: T 
    },
    /// An Attribute which can take on an enumerated selection of values
    /// FIXME: I kind of need a higher kinded type here or GAT.  Ie, Categorized(DamageEffects)
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

    /// Determines how many caharcter points the Attribute costs
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

    fn cost<F>(val: Self::Value, fun: F) -> f32
    where
        F: Fn(Self::Value) -> f32,
    {
        fun(val)
    }
}

/// Enumerates the kinds of ElectroMagnetic forms
pub enum EMSpectrum {
    /// Regular light
    Normal,
    /// Infrared
    IR,
    /// Ultraviolet
    UV,
    /// Xrays
    Xray,
    /// gamma rays
    Gamma,
}

/// Enumerations of different sound frequencies
pub enum AuditorySpectrum {
    /// Sound below range of human hearing (approx 20hz)
    Subsonic,
    /// Normal tange of human hearing
    Normal,
    /// Sound above human hearing (approximately 20khz)
    Ultrasound,
}

/// The kinds of (normal) human senses
pub enum Senses {
    /// Sight
    Visual(EMSpectrum),
    /// Hearing
    Audio(AuditorySpectrum),
    /// Smelling
    Olfactory,
    /// Touch
    Tactile,
    /// Taste
    Taste,
}

pub mod characteristics;
pub mod equipment;
/// Module that defines how tashs in the game works
pub mod tasks;
