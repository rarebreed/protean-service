//! # This module declares a Task
//!
//! A Task defines anything which is uncertain in game terms and how to accomplish it.  Tasks revolve around a die pool
//! and are always opposed.  The die pool is d20 based and is _exploding_.  If a 20 is rolled, the die can be rerolled
//! again.
//!
//! ## Associated Characteristics and Skills
//!
//! All tasks have a set of associated Characteristics, and some tasks have a set of associated skills.  This is an
//! important concept to distinguish.  Some tasks might be based on raw innate abilities.  Examples could include:
//!
//! - Lifting up a non-standard heavy object
//! - Staying awake to study for an exam
//! - Remembering a detail from a conversation
//! - Avoiding catching a cold
//! - Jumping across a chasm
//!
//! Some tasks might have an associated skill, but can be performed without a skill.  Some skills therefore have an
//! innate classifier, meaning anyone can do it without penalty, but also without any advantage.  Examples include:
//!
//! - Singing
//! - Lifting
//! - Persuasion
//!
//! There's always an advantage if one is skilled, but one's skill is based off of how competent the associated
//! characteristic is.  It is thus possible for someone extremely talented but not as skilled, to best someone who is
//! more skilled, but not as talented.  Unlike most games which use an additive system where one adds the relevant skill
//! to the appropriate characteristic(s), retro sets the characteristic as a target number, and one's skill as the
//! number of dice rolled.  One counts how many successes are rolled and this count is compared against a competing
//! value.
//!
//! ## Contests
//!
//! Therefore, all tasks are contested rolls, there is no fixed _threshold_ one must achieve.  Just because the player
//! rolled well doesn't mean he knows if he succeeded or not, because the GM might also have rolled very well.  This
//! design introduces an air of uncertainty about knowing whether the player actually succeeded or not.
//!
//! Contested tasks are relatively easy to figure out when it is one character against another.  But how would one
//! determine the contested roll for say, remembering a detail?  This is where difficulty levels come in.  They always
//! start with a target number of 10, and the number of dice is what will normally vary.  However, there are occasions
//! where the target number may also be adjusted.  This is generally avoided though except for rare circumstances.  This
//! is because it is harder for the human mind to try to estimate the odds of a roll based on 2 changing variables: the
//! number of dice rolled, and the target number for each.  This is exacerbated by the fact that there is more
//! uncertainty induced by having the rolls be contested.
//!
//! ## Law of Averages
//!
//! Attribute ratings are from 0-18 with the average hanging out at 10. Skill levels range from 0 to 9.  
//!
//! ### General Difficulty levels
//!
//! | Difficulty  | Academic    | years training | Successes | 5@10 | 9@18 |
//! |-------------|-------------|----------------|-----------|------|--------------------
//! | Trivial     | Untrained   | 0              | 0         
//! | Easy        | Grade       | <1             | 1
//! | Simple      | Junior      | 1-2            | 2
//! | Average     | High School | 2-3            | 3
//! | Above Avg   | Bachelor    | 3-4            | 4
//! | Difficult   | Post-Bacc   | 4-5            | 6
//! | Challenging | Master      | 6-7            | 8
//! | Epic        | Doctorate   | 8-9            | 10
//! | Legendary   | Post-Doc    | 9+             | 12
//! | Mythic      |             |                | 14
//! | Impossible  |             |                | 16
//!
//! When a system uses both a number of dice and a moveable target number (as retro does), it can be hard to determine
//! how to modify the difficulty of a task.  
//!
//! - Do you adjust the number of dice rolled (for the character, the adversary, or both)?
//! - Do you adjust the target number (for the character, the adversary or both)?
//!
//! Since retro always uses an opposed system, there is never a fixed target.  Rather, the GM should consider the
//! following:
//!
//! - If there is an inherent or internal modifier, it should adjust that actor's roll
//! - If there is an external modifier can adjust either party's roll but never both
//!
//! Examples of external factors are visibility, footing, timing or distance.  Examples of internal factors are being
//! wounded, fatigued, or dizzy.  If someone is shooting in poor visibility, this will be bonus to the defenders roll
//! rather than a minus to the shooter's roll.  Conversely, if the defender is wounded, it will reduce his roll. Another
//! example would be the use of superior tools to perform a task.  Since this is an externl modifier, it reduces the
//! opposing roll.
//!
//! Take for example shooting at a non-moving distant target.  In broad daylight with minimal winds, the various factors
//! are:
//!
//! - distance
//! - size of the target
//! - visibility
//! - accuracy of the weapon
//! - how much time spent aiming
//!
//! Notice that all of these factors are external, and thus become adjustments to the opposing roll.
//!
//! For a moving target we would also factor in:
//!
//! - Variability of movement (what is the sum of the vector changes over time...ie how zig-zaggy?)
//!
//! Actually the distance and size of the target for hitting an object boil down to three values, which are:
//!
//! - The delta angle of the weapon which can produce a hit
//! - The change in delta angle (for moving targets...including estimating lead and drop)
//! - Environmental factors (inherent accuracy of the weapon and things like wind)
//!
//! We will go over more detail about this in the full combat section
//!
//! ### How to determine modifiers
//!
//! As mentioned above, determining how many dice to roll and what the target number is can be unintuitive.  There's
//! also a mathematical affect which must be considered.
//!
//! Increasing the number of dice rolled has two ramifications.  The first, is it improves the odds of getting more
//! successes than the opposition.  The second ramification is implicit from the first: it means you can have better
//! margins of success.  It is not enough to know that you have more successes than your opponent, but also how _many_
//! more successes you have.  Increasing the number of dice rolled means you increase your odds of getting another
//! additional success.
//!
//! Therefore, only increase the number of dice if there is a reason to think that the task modifier would increase not
//! just the likelihood of success, but also better quality of success.  In contrast, increasing the target number only
//! increases the likelihood that you will get more successes in the pool of dice that you already have.  Adjust the TN
//! if you think the odds of getting successes in your pool should be adjusted, but not increasing the total amount of
//! possible successes.
//!
//! To think of it another way, imagine if instead of having 5 cards in your hand in poker, you had 6 or 7 cards.  Now,
//! you increase the odds not only of getting a 2, 3 or 4 of a kind, (and assuming you need only 5 cards) it also
//! improves the odds of getting a straight or a flush.  This would be similar to adding more dice to your pool.
//!
//! Some examples of when you would adjust the number of pool in the dice is when:
//!
//! - You lack tools for a technical task
//! - You are fighting unarmed while injured or immobilized
//! - You have a status condition applied
//! - The equipment you are using is in poor condition
//!
//! Adjusting the Target Number is generally more commonly applied.  These adjust how likely your existing pool will
//! result, but does not affect the possible total magnitude.  For example, firing at night is mostly a matter of sheer
//! luck.  The odds of getting a success in your pool are vastly reduced, but, a lucky shot is still a lucky shot.
//!
//! Examples of when to adjust the target number are
//!
//! - Amount of time to prepare
//!     - Jury rigged repairs (may suffer from lack of tools also)
//!     - Snap shot (firing from the hip, or without sighting)
//! - Poor environmental conditions
//!     - Fighting on shaky ground
//!     - Fighting in poor visibility
//!     - Chase on rainy roads
//!
//! Because retro uses d20 for rolls, each adjust for a single die is a 5% reduction.  However, we roll pools of dice,
//! and not a single die.  Also, adjustments to the TN never affect the exploding number.  The exploding value is always
//! a _natural_ 19 or 20, never a modified result of 19 or better.
//!
//! Tasks define how actions are resolved
//! Die examples:
//!
//! let simpleD6 = die(6)
//! let roll = simpleD6.roll(10)
//!
//! let d6 = die(6)
//!   .exploding(Some(6))
//!   .value(|amt| {
//!      default_rng(5, 2)
//!   })
//!   .roll(10)

use std::time::Duration;

use num_traits::NumOps;

/// A modifier for a task can be internal or external to the character
pub enum ModifierType {
    /// An Internal modifier is something innate or internal to the character (ie, wounded)
    Internal,
    /// An external modifier is external to the character (ie, darkness)
    External,
}

/// Modifiers are not just scalar, they have a direction too
pub enum ModifierDirection {
    /// a Positive modifier helps make a task easier
    Positive,
    /// a Negative Modifier makes a task harder
    Negative,
}

/// Defines a modifier for a Task
pub struct TaskModifier<T>
where
    T: NumOps,
{
    /// Name of the modifier
    pub name: String,
    /// a unique identifier
    pub id: String,
    /// the scalar magnitude of a modifier
    pub magnitude: T,
    /// the Positive or Negative direction
    pub direction: ModifierDirection,
    /// the Internal or External type
    pub mod_type: ModifierType,
}

/// Defines a Task
pub struct Task {
    /// Descriptive name for the task
    pub name: String,
    /// a unique identifer for this Task
    pub id: String,
    /// The task type (Normal, Gradual, or Team)
    pub t_type: TaskType,
    /// modifiers: A vector of modifiers for this task
    pub modifiers: Vec<TaskModifier<u32>>,
    /// The time it normally takes to perform the task
    pub time: f64,
}

/// A Task can be normal (which is always a contested roll), take a long time to perform or
/// require a team effort
pub enum TaskType {
    /// A Task which takes a long time to perform
    Gradual(Duration),
    /// a team oriented task
    Team(usize),
    /// the default contested form of one-on-one or one-against-environment task
    /// Normal tasks are all opposed, even against just the environment
    Normal,
}
