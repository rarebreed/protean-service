//! All characters and most other entities will have equipment of some sort.  This includes weapons, survival gear,
//! clothing, communications, medical, scientific and food supplies among many other broad categories

use crate::{Attribute, Senses};

/// Weapons are broadly categorized as
/// - Unarmed (this can include teeth or claws)
/// - Melee: short ranged hand-to-hand weapons
/// - Ranged: weapons that deal damage at a distance (includes thrown weapons)
pub enum WeaponClass {
    /// only what nature gave
    Unarmed,
    /// hand to hand weapons
    Melee,
    /// weapons that deal damage at range
    Ranged,
}

/// How damage is inflicted
///
/// Depending on the medium, different damage can be delivered
pub enum DamageMedium {
    /// Energy of some sort such as a laser, fire, or electricity
    Energy,
    /// Damage by impact of a physical object
    Kinetic,
    /// A special form of energy which has certain properties
    Radiation,
    /// A disease of some sort
    Pathogen,
    /// Poison or other caustic substances
    Chemical,
}

/// Kinetic damage can cause different kinds of damage
pub enum StructuralDamageType {
    /// Muscle tears or even severing of muscles
    Muscular,
    /// Broken bones
    Bones,
    /// Torn cartilage, ligaments or tendons
    ConnectiveTissue,
}

/// Trauma is immediate and destructive forms of damage
///
/// Weapons and other inflictions (disease, falling, being on fire) all cause certain kinds of damage.  This enum
/// covers the various kinds of damage that can be inflicted
pub enum Trauma {
    /// Damages by slashing, causing secondary Hemorraghic and Structural damage
    Cutting,
    /// Damages from an impactful force.  Causes internal hemorraghing and Structural damage
    Crushing,
    /// A piercing damage.  Causes less structural damage, but can cause deeper bleeding and obstructive shock
    Impaling,
    /// Tissue is destroyed by being burned
    Burning,
    /// Tissue necrosis due to freezing
    Freezing,
    /// Diesease borne effect
    Pathological,
    /// Internal bleeding, including bruising
    Hemorraghing,
    /// Destruction of tissue due to poison, caustic substance or septosos
    Necrosis,
}

/// Damage to the mind can take several forms
pub enum NeurologicalEffect {
    /// We all know what pain is
    Pain(f64),
    /// The mind is no longer active (sleep like state)
    Unconsciousness,
    /// Still conscious but with diminished awareness and control
    Stunned(f64),
    /// Nausea
    Sickened(f64),
    /// Overwhelming sleepiness
    Drowsiness(f64),
    /// Afflication that reduces a sense
    Sensory(Senses, f64),
}

///
pub enum DamageEffect {
    /// unable to breathe
    Asphyxiation,
    /// No nutrients or caloric intake, leading to eventual death
    Starvation,
    /// No water intake, leading to death
    Dehydration,
    /// Hemorraghic shock.  Ex-Sanguination.  How long until bleed out?
    Shock(u32),
    /// Physical damage, eg broken bones, torn
    Structural {
        /// the physical type of damage (broken bone, torn ligament etc)
        damage_type: StructuralDamageType,
        /// severity of damage
        condition: Condition,
    },
    /// Damage to sensory input
    Sensory {
        /// Which sense
        sense: Senses,
        /// how severe
        condition: Condition,
    },
    /// Various neurologic effects like pain, unconsciousness, or sickened
    Neurological(NeurologicalEffect),
    /// Shock
    OrganFailure(u32),
}

/// Condition of equipment, part, organ, etc
pub enum Condition {
    /// The item is totally destroyed, not even salvageable
    Obliterated,
    /// The item is not able to be repaired.  Might be some salvage
    Destroyed,
    /// The item might be able to be repaired, but is essentially unusable
    Broken,
    /// The item is functional, but just barely.  Item needs to be serviced or healed
    Failing,
    /// The item is serviceable, but needs maintainance or rest.  More likely to breakdown
    Worn,
    /// Item is in regular and maintained condition
    Normal,
    /// Factory new
    Pristine,
    /// The equipment has been made more durable
    Reinforced(u16),
}

/// How powerful damage can be
pub struct DamageClass {
    /// The kind of damage
    pub damage_type: DamageMedium,
    /// power level
    pub power: f64,
}

impl From<DamageClass> for f64 {
    fn from(dc: DamageClass) -> Self {
        dc.power
    }
}

/// Defines a weapon
pub struct Weapon {
    /// state of the weapon
    pub integrity: Condition,
    /// kind of damage it does
    pub damage: Attribute<DamageClass, DamageEffect>,
}
