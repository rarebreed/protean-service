//! Module declares how equipment works, including damage

use std::collections::HashMap;

use crate::{Attribute, Senses};

/// Broad classes of weapon types
pub enum WeaponClass {
    /// No weapom
    Unarmed,
    /// Hand to hand weapon (eg, sword, spear, knife)
    Melee,
    /// Projectile weapon (eg bow, pistol, thrown spear)
    Ranged,
}

/// How damage is effected
pub enum DamageMedium {
    /// Damage from a non-kinetic source (eg electrical or fire)
    Energy,
    /// Damage through kinetic energy (most weapons in Earth's history)
    Kinetic,
    /// Special form of energy damage through radiation
    Radiation,
    /// Virus, bacteria or other contagious disease
    Pathogen,
    /// Includes poisons or caustic chemical weapons (mustard gas)
    Chemical,
    /// Non-contagious biological (eg parasites, fungi)
    Biological,
}

/// Enumerates the ways structural damage can happen to a character
///
/// This does not include non-structual damage, such as bleeding or asphyxiation
pub enum StructuralDamageType {
    /// Damage to muscle such as tearing or cutting a muscle
    Muscular,
    /// Breaking of bones
    Bones,
    /// Ligaments, cartilage fascia and other kinds of sprains, strains and pulls
    ConnectiveTissue,
    /// Severing off of a limb
    Severing,
}

/// Trauma is immediate and destructive forms of damage.  It is how damage works
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
    /// The freezing of tissue causing its destruction
    Freezing,
    /// Poisons, either ingested or contact
    Poison,
    /// Diesease borne effect
    Pathological,
}

/// Effects that effect the brain or nervous system
pub enum NeurologicalEffect {
    /// pain
    Pain,
    /// unconsciousness or knocked out
    Unconsciousness,
    /// Dizzyness, vertigo, or other not full in control action affects
    Stunned,
    /// Being nauseous or otherwise overriding impulse on not feeling well
    Sickened,
    /// Inabilty to summon full muscular power.  Enervated
    Weakened,
    /// A sensory system incapacitation (eg blindness or deafness)
    Sensory(Senses),
}

/// The ways damage can be inflicted
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
        /// How the damage is inflicted
        damage_type: StructuralDamageType,
        /// The condition it imposes
        condition: Condition,
    },
    /// Damage to sensory input
    Sensory {
        /// Which sense it affects
        sense: Senses,
        /// The condition it imposes
        condition: Condition,
    },
    /// Various neurologic effects like pain, unconsciousness, or sickened
    Neurological {
        /// What neurological effect it inflicts
        effect: NeurologicalEffect,
        /// How severe the damage is
        severity: u16,
    },
    /// Shock and systemic failure
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

/// Defines how damage is done
pub struct DamageClass {
    /// How is the damage done
    pub damage_type: DamageMedium,
    /// how powerful is the damage can be
    pub power: f32,
}

impl Into<f32> for DamageClass {
    fn into(self) -> f32 {
        self.power
    }
}

/// Defines a weapon
pub struct Weapon {
    /// The condition of the weapom
    pub integrity: Condition,
    /// What kind of damage it does
    pub damage: HashMap<String, Attribute<DamageClass, DamageEffect>>,
}
