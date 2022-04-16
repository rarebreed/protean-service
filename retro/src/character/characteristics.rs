//! This module describes characteristics in game terms


use serde::{Deserialize, Serialize};
use crate::{
    Attribute,
    CharacteristicType::{
        self,
        MentalPrimaryCharacteristics as Mental,
        PrimaryCharacteristics as Primary,
        // Principle as PrincipalType,
        // Personality as PersonalityType,
        // SecondaryCharacteristics as Secondary,
        SocialCharacteristics as Social,
    },
    Ranged::{self, Categorized, Spectrum},
    Trait,
};

/// Psyche is a kind of Attribute that deals with the psychology and personality of a character
pub struct Psyche(Attribute<f32, f32>);

impl Trait for Psyche {
    type Value = f32;
    type Range = f32;
    type Parent = CharacteristicType;

    fn name(self: &Self) -> &str {
        &self.0.name
    }

    fn set_value(self: &mut Self, val: Self::Value) -> () {
        self.0.value = val;
    }

    fn value(self: &Self) -> &Self::Value {
        &self.0.value
    }

    fn set_value_range(self: &mut Self, range: Ranged<Self::Range>) -> () {
        self.0.range = range;
    }

    fn value_range(self: &Self) -> &Ranged<Self::Range> {
        &self.0.range
    }

    fn parent(self: &Self) -> &Self::Parent {
        &self.0.parent
    }

    fn set_parent(self: &mut Self, parent: Self::Parent) -> () {
        self.0.parent = parent
    }

    fn cost<F>(val: Self::Value, fun: F) -> f32
    where
        F: Fn(Self::Value) -> f32,
    {
        todo!()
    }
}

/// The primary physical characteristics of a character
#[derive(Serialize, Deserialize, Debug)]
pub struct PhysicalPrimaryCharacteristics {
    /// How quikcly the character can move move the limbs
    pub speed: Attribute<f32, f32>,
    /// How much energy can be exerted through the limbs (ie, stength) (power is speed * force)
    pub force: Attribute<f32, f32>,
    /// Hand eye coordination
    pub dexterity: Attribute<f32, f32>,
    /// Ability to control the entire body and spatial awareness
    pub kinesthesis: Attribute<f32, f32>,
    /// Measure of constitution and hardiness to disease
    pub health: Attribute<f32, f32>,
    /// A measure of physical fitness and endurance
    pub fitness: Attribute<f32, f32>,
    /// Weight of character in kg
    pub mass: Attribute<f32, f32>,
    /// Height of character in meters
    pub height: Attribute<f32, f32>,
}

impl Default for PhysicalPrimaryCharacteristics {
    fn default() -> Self {
        let range = Spectrum {
            min: 0.00f32,
            max: 100.00f32,
        };

        PhysicalPrimaryCharacteristics {
            speed: Attribute::new("speed".into(), 0.0, Primary, range.clone()),
            force: Attribute::new("force".into(), 0.0, Primary, range.clone()),
            dexterity: Attribute::new("dexterity".into(), 0.0, Primary, range.clone()),
            kinesthesis: Attribute::new("kinesthesis".into(), 0.0, Primary, range.clone()),
            health: Attribute::new("health".into(), 0.0, Primary, range.clone()),
            fitness: Attribute::new("fitness".into(), 0.0, Primary, range.clone()),
            mass: Attribute {
                name: "mass".into(),
                value: 70.0f32,
                range: Spectrum {
                    min: 0.00f32,
                    max: std::f32::MAX,
                },
                parent: Primary,
            },
            height: Attribute::new(
                "height".into(),
                150.0,
                Primary,
                Spectrum {
                    min: 0.00,
                    max: std::f32::MAX,
                },
            ),
        }
    }
}

/// The primary mental characteristics of a character
pub struct MentalPrimaryCharacteristics {
    /// How well one can remember or recall details
    pub memory: Attribute<f32, f32>,
    /// Ability to come up with deductions and analyze data
    pub analysis: Attribute<f32, f32>,
    /// Wisdom and the ability to see the bigger picture
    pub insight: Attribute<f32, f32>,
    /// Being able to come up with novel ideas or expressions
    pub creativity: Attribute<f32, f32>,
    /// Willpower and mental discipline
    pub discipline: Attribute<f32, f32>,
    /// Ability to not get distracted
    pub focus: Attribute<f32, f32>,
}

impl Default for MentalPrimaryCharacteristics {
    fn default() -> Self {
        let range = Spectrum {
            min: 0.00f32,
            max: 100.00f32,
        };

        MentalPrimaryCharacteristics {
            memory: Attribute::new("memory".into(), 0.0, Mental, range.clone()),
            analysis: Attribute::new("analysis".into(), 0.0, Mental, range.clone()),
            insight: Attribute::new("insight".into(), 0.0, Mental, range.clone()),
            creativity: Attribute::new("creativity".into(), 0.0, Mental, range.clone()),
            discipline: Attribute::new("discipline".into(), 0.0, Mental, range.clone()),
            focus: Attribute::new("focus".into(), 0.0, Mental, range.clone()),
        }
    }
}

/// Characteristics which influence interactions with other characters
pub struct SocialPrimaryCharacteristics {
    /// Physical looks
    pub comeliness: Attribute<f32, f32>,
    /// Charisma and eloquence
    pub presence: Attribute<f32, f32>,
    /// Socioeconomic class
    pub standing: Attribute<f32, SocialCategories>,
}

/// The kinds od social classes a charcter can belong to
pub enum SocialCategories {
    /// Hereditary or earned rank of esteem and power
    Noble,
    /// The upper end of social/economic (eg the very wealthey and influential)
    Upper,
    /// Average middle class people
    Common,
    /// On the lower end of social or economic power
    Lower,
    /// Criminals, or others that most society deems undesirable
    Stigmatized,
}

impl Default for SocialPrimaryCharacteristics {
    fn default() -> Self {
        let range = Spectrum {
            min: 0.00f32,
            max: 100.00f32,
        };
        let category = Categorized(SocialCategories::Common);

        SocialPrimaryCharacteristics {
            comeliness: Attribute::new("comliness".into(), 0.0, Social, range.clone()),
            presence: Attribute::new("presence".into(), 0.0, Social, range.clone()),
            standing: Attribute::new("standing".into(), 0.0, Social, category),
        }
    }
}

/// The major primary (non-derived) charcteristics
pub struct PrimaryCharacteristics {
    /// All the PhysicalPrimaryCharacteristics
    pub physical: PhysicalPrimaryCharacteristics,
    /// All the
    pub mental: MentalPrimaryCharacteristics,
    /// All the
    pub social: SocialPrimaryCharacteristics,
}

impl Default for PrimaryCharacteristics {
    fn default() -> Self {
        PrimaryCharacteristics {
            physical: PhysicalPrimaryCharacteristics::default(),
            mental: MentalPrimaryCharacteristics::default(),
            social: SocialPrimaryCharacteristics::default(),
        }
    }
}

/// The ratings of the senses for a charcter
pub struct SensoryCharacteristics {
    /// How well the character can see
    pub sight: Attribute<f32, f32>,
    /// How well the character can hear
    pub hearing: Attribute<f32, f32>,
    /// How sensitive the character's touch is
    pub tactile: Attribute<f32, f32>,
    /// How well the charcter can smell
    pub olfactory: Attribute<f32, f32>,
    /// How well the character can taste
    pub taste: Attribute<f32, f32>,
    /// Sixth sense or premonition of a character
    pub intuition: Attribute<f32, f32>,
}

/// Secondary or derived characteristics
pub struct SecondaryCharacteristics {
    /// How well the character can react or respond under pressure
    pub coolness: Attribute<f32, f32>,
}
