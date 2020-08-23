//! This module describes characteristics in game terms

use serde::{Serialize, Deserialize};

use crate::{
    CharacteristicType::{
        PrimaryCharacteristics as Primary,
        SecondaryCharacteristics as Secondary,
        MentalPrimaryCharacteristics as Mental,
        SocialCharacteristics as Social,
        Psyche as PsycheType,
        Principle as PrincipalType,
        self
    },
    Attribute,
    Trait,
    Ranged::{
        self,
        Spectrum,
        Absolute,
        Categorized
    }
};

pub struct Psyche {
    attr: Attribute<f32, f32>
}

impl Trait for Psyche {
    type Value = f32;
    type Range = f32;
    type Parent = CharacteristicType;

    fn name(self: &Self) -> &str {
        &self.attr.name
    }

    fn set_value(self: &mut Self, val: Self::Value) -> () {
        self.attr.value = val;
    }

    fn value(self: &Self) -> &Self::Value {
        &self.attr.value
    }

    fn set_value_range(self: &mut Self, range: Ranged<Self::Range>) -> () {
        todo!()
    }

    fn value_range(self: &Self) -> &Ranged<Self::Range>{
        &self.attr.range
    }

    fn parent(self: &Self) -> &Self::Parent {
        todo!()
    }

    fn set_parent(self: &mut Self, parent: Self::Parent) -> () {
        todo!()
    }

    fn cost<F>(val: Self::Value, fun: F) -> f32 
    where F: Fn(Self::Value) -> f32{
      fun(val)
  }
}

#[derive(Serialize, Deserialize)]
pub struct PhysicalPrimaryCharacteristics {
    pub speed: Attribute<f32, f32>,
    pub force: Attribute<f32, f32>,
    pub dexterity: Attribute<f32, f32>,
    pub kinesthesis: Attribute<f32, f32>,
    pub health: Attribute<f32, f32>,
    pub fitness: Attribute<f32, f32>,
    pub mass: Attribute<f32, f32>,
    pub height: Attribute<f32, f32>
}

impl Default for PhysicalPrimaryCharacteristics {
    fn default() -> Self {
        let range = Spectrum {
            min: 0.00f32,
            max: 100.00f32
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
                Spectrum { min: 0.00, max: std::f32::MAX }
            )
        }
    }
}

struct MentalPrimaryCharacteristics {
    pub memory: Attribute<f32, f32>,
    pub analysis: Attribute<f32, f32>,
    pub insight: Attribute<f32, f32>,
    pub creativity: Attribute<f32, f32>,
    pub discipline: Attribute<f32, f32>,
    pub focus: Attribute<f32, f32>
}

impl Default for MentalPrimaryCharacteristics {
    fn default() -> Self {
        let range = Spectrum {
            min: 0.00f32,
            max: 100.00f32
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

pub struct SocialPrimaryCharacteristics {
    pub comeliness: Attribute<f32, f32>,
    pub presence: Attribute<f32, f32>,
}

impl Default for SocialPrimaryCharacteristics {
    fn default() -> Self {
        let range = Spectrum {
            min: 0.00f32,
            max: 100.00f32
        };

        SocialPrimaryCharacteristics {
            comeliness: Attribute::new("comliness".into(), 0.0, Social, range.clone()),
            presence: Attribute::new("presence".into(), 0.0, Social, range.clone()),
        }
    }
}

struct PrimaryCharacteristics {
    pub physical: PhysicalPrimaryCharacteristics,
    pub mental: MentalPrimaryCharacteristics,
    pub social: SocialPrimaryCharacteristics
}


impl Default for PrimaryCharacteristics {
    fn default() -> Self {
        PrimaryCharacteristics {
            physical: PhysicalPrimaryCharacteristics::default(),
            mental: MentalPrimaryCharacteristics::default(),
            social: SocialPrimaryCharacteristics::default()
        }
    }
}

pub struct SensoryCharacteristics {
    pub sight: Attribute<f32, f32>,
    pub hearing: Attribute<f32, f32>,
    pub tactile: Attribute<f32, f32>,
    pub olfactory: Attribute<f32, f32>,
    pub taste: Attribute<f32, f32>,
    pub intuition: Attribute<f32, f32>,
}

struct SecondaryCharacteristics {
    pub coolness: Attribute<f32, f32>
}

