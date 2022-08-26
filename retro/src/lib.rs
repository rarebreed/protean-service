use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CharacteristicType {
    PrimaryCharacteristics,
    SecondaryCharacteristics,
    MentalPrimaryCharacteristics,
    SocialCharacteristics,
    Psyche,
    Principle,
    PersonalityType,
    Principles,
    Equipment,
}

/// A Trait is a trait for specifying what you can do with generic data
///
/// Many values that make up a character, or other entities like weapons, vehicles, or gadgets are built by implementing
/// this trait
pub trait Trait {
    type Value;
    type Range;
    type Parent;

    /// the name is a read-only field, so there's only a getter
    fn name(self: &Self) -> &str;

    /// The value is a _score_ defining some value on a Range.  The Range has a range defined as a Tuple of a
    /// minimum and a maximum (if the upper has a value), or an enum defining possible values
    fn set_value(self: &mut Self, val: Self::Value) -> ();
    fn value(self: &Self) -> &Self::Value;
    fn set_value_range(self: &mut Self, range: Ranged<Self::Range>) -> ();
    fn value_range(self: &Self) -> &Ranged<Self::Range>;

    /// The Parent category of a Trait
    fn parent(self: &Self) -> &Self::Parent;
    fn set_parent(self: &mut Self, parent: Self::Parent) -> ();

    /// Some values are determined randomnly, but there is always a cost/value associated with the trait/value
    fn cost<F>(val: Self::Value, fun: F) -> f32
    where
        F: Fn(Self::Value) -> f32;
}

#[derive(Serialize, Deserialize, Clone)]
/// Specifies the range that an Attribute can take
pub enum Ranged<T> {
    /// An Attribute that varies across a range of values with a min and max (inclusive)
    Spectrum { min: T, max: T },
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

    fn cost<F>(val: Self::Value, fun: F) -> f32
    where
        F: Fn(Self::Value) -> f32,
    {
        fun(val)
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
pub mod engine;
