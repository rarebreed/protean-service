//! Module definitions for a character in retro
//! 
//! # The Character
//! 
//! The Character is the most fundamental aspect of the game, since the game revolves around
//! the caharcter's lives and actions.  The Character holds all the information about what the 
//! character can do, and just as importantly who the character is.  This means concepts like 
//! personality traits, motivations, and principles.
//! 
//! As such a Character's definitions can be roughly divided into several categories
//! 
//! - what the character can do
//! - how a character behaves, thinks, or believes
//! - how society interacts with the character
//! 
//! A fourth broad category exists, but is often intertwined with the other three.  It is the
//! history and background of the character, such as where they were born, what education and
//! training they received, and any past escapades or back story.
//! 
//! ## What a Character can do
//! 
//! The first category are often what players first think about when imagining their character.
//! These are things like, "how strong is my character?" or "how smart is she?".  This also
//! includes skills or talents.  Anything that defines what a character can do and how good they
//! are at it is here, and includes
//! 
//! - All the Attributes
//! - All the Skills
//! - All Talents
//! 
//! ## How a Character behaves
//! 
//! Many games do not even deal with this aspect, but in this design it is equal in importance
//! to the other broad categories.  The world's popular TTRPG has an alignment system, which in 
//! incredib;y broad strokes, paints a guideline for how characters behave.
//! 
//! Retro goes far far deeper than this, and is influenced by some psychological, ethical, 
//! religious and philosophical concerns.  Just as we define how strong a character is, retro
//! also has some baseline personality traits like "how self-centered is the character?"  There
//! are also some freeform attributes that asks the player to consider what principles and beliefs
//! the character adheres to.
//! 
//! Because this is a rare feature of games, some players will be hesistant.  Many players want 
//! absolute control over their characters (unless it is a horror game, where feeling in terror or
//! going insane is part of the trope).  Retro however takes the stance that we often _wish_ we 
//! would act or behave one way, even if reality says otherwise.  Overridding the impulse is
//! possible, but at a cost.  When a character can act bravely without trying, then heroism becomes
//! trivial and cheapened.
//! 
//! So, retro considers the following
//! 
//! - Psyche
//! - Principles
//! - North Stars
//! 
//! ## Social factors
//! 
//! Not only does retro consider how the characters internal personality, beliefs and principles
//! come into play, but also how the external world treats the character.
//! 
//! As in the real world, prejudices exist, as does favoritism.

pub struct Character {
    name: String,
    player: String,
    faction: Faction
}