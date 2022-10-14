//! This module describes characteristics in game terms

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::{
    Attribute,
    CharacteristicType::{
        self,
        MentalPrimaryCharacteristics as Mental,
        PrimaryCharacteristics as Primary,
        //Principle as PrincipalType, Psyche as PsycheType, SecondaryCharacteristics as Secondary,
        SocialCharacteristics as Social,
    },
    Ranged::{self, Spectrum},
    Trait,
};

/// A psychological trait a character has
pub struct Psyche {
    attr: Attribute<f32, f32>,
}

impl Trait for Psyche {
    type Value = f32;
    type Range = f32;
    type Parent = CharacteristicType;

    fn name(&self) -> &str {
        &self.attr.name
    }

    fn set_value(&mut self, val: Self::Value) {
        self.attr.value = val;
    }

    fn value(&self) -> &Self::Value {
        &self.attr.value
    }

    fn set_value_range(&mut self, range: Ranged<Self::Range>) {
        self.attr.range = range;
    }

    fn value_range(&self) -> &Ranged<Self::Range> {
        &self.attr.range
    }

    fn parent(&self) -> &Self::Parent {
        &self.attr.parent
    }

    fn set_parent(&mut self, parent: Self::Parent) {
        self.attr.parent = parent;
    }

    fn cost<F>(val: Self::Value, fun: F) -> f32
    where
        F: Fn(Self::Value) -> f32,
    {
        fun(val)
    }
}

/// The PhysicalPrimaryCharacteristics describes character attributes that are expressed physically in the real world
#[derive(Serialize, Deserialize, Debug)]
pub struct PhysicalPrimaryCharacteristics {
    /// speed is a measure of how quickly one can move.  It is a combination of reflexes and quickness
    pub speed: Attribute<f32, f32>,
    /// force is similar to strength in that it determines energy that can be transmitted. force * speed = power
    pub force: Attribute<f32, f32>,
    /// dexterity measures hand-eye coordination.  Think surgeon, watch maker, pickpocket
    pub dexterity: Attribute<f32, f32>,
    /// kinesthesis the ability to sense and control the body in space.  Think gymnast, acrobat, martial artist
    pub kinesthesis: Attribute<f32, f32>,
    /// health is a measure of hardiness and resistance to damage or disease
    pub health: Attribute<f32, f32>,
    /// fitness is a measure of endurance
    pub fitness: Attribute<f32, f32>,
    /// mass measures the weight of the character in kilograms
    pub mass: Attribute<f32, f32>,
    /// height measures the height of the character in meters
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
            fitness: Attribute::new("fitness".into(), 0.0, Primary, range),
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

/// MentalPrimaryCharacteristics covers attributes that are of the mind in nature
#[derive(Serialize, Deserialize, Debug)]
pub struct MentalPrimaryCharacteristics {
    /// memory is a measure of recall, and how well a character remembers things
    pub memory: Attribute<f32, f32>,
    /// analysis is what most games would call intelligence.  It is the ability to solve
    pub analysis: Attribute<f32, f32>,
    /// insight is a combination of wisdom and the ability to see patterns and relationships
    pub insight: Attribute<f32, f32>,
    /// creativity measures how well one can think outside the box, or come up with new ideas or expressions
    pub creativity: Attribute<f32, f32>,
    /// discipline measures mental fortitude
    pub discipline: Attribute<f32, f32>,
    /// focus is concentration and ability to keep track of one thing
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
            focus: Attribute::new("focus".into(), 0.0, Mental, range),
        }
    }
}

/// The main social characteristics that influence other people's behavior or attitudes
#[derive(Serialize, Deserialize, Debug)]
pub struct SocialPrimaryCharacteristics {
    /// comelines measure physical beauty (by the default society's standards)
    pub comeliness: Attribute<f32, f32>,
    /// presence is silent charisma and physical magnetism
    pub presence: Attribute<f32, f32>,
    /// eloquence is a measure of how well one speaks and can influence others
    pub eloquence: Attribute<f32, f32>,
}

impl Default for SocialPrimaryCharacteristics {
    fn default() -> Self {
        let range = Spectrum {
            min: 0.00f32,
            max: 100.00f32,
        };

        SocialPrimaryCharacteristics {
            comeliness: Attribute::new("comliness".into(), 0.0, Social, range.clone()),
            presence: Attribute::new("presence".into(), 0.0, Social, range.clone()),
            eloquence: Attribute::new("eloquence".into(), 0.0, Social, range),
        }
    }
}

/// The PrimaryCharacteristics are the most looked at and salient attributes of a character
#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryCharacteristics {
    /// Physical attributes such as speed, power or endurance
    pub physical: PhysicalPrimaryCharacteristics,
    /// Mental attributes such as analysis, insight or creativity
    pub mental: MentalPrimaryCharacteristics,
    /// Social attributes such as charm or looks
    pub social: SocialPrimaryCharacteristics,
}

/// Human senses
pub struct SensoryCharacteristics {
    /// How well an entity can see
    pub sight: Attribute<f32, f32>,
    /// How well an entity can hear
    pub hearing: Attribute<f32, f32>,
    /// How sensitive an entity's touch is
    pub tactile: Attribute<f32, f32>,
    /// How sensitive an entity's smell is
    pub olfactory: Attribute<f32, f32>,
    /// How well an entity can taste
    pub taste: Attribute<f32, f32>,
    /// A kind of gut feeling or sixth sense
    pub intuition: Attribute<f32, f32>,
}

/// Characteristics which are derived from other characteristics
pub struct SecondaryCharacteristics {
    /// How well an entity can handle stressful situations
    pub coolness: Attribute<f32, f32>,
}

#[cfg(test)]
mod characteristic_tests {
    use super::*;

    #[test]
    fn test_create_character() {}
}
