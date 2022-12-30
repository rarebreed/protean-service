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
//! more skilled, but not as talented.  
//!
//! Unlike most games which use an additive system where one adds the relevant skill to the appropriate
//! characteristic(s), retro sets the characteristic as a target number, and one's skill as the number of dice rolled.
//! One counts how many successes are rolled and this count is compared against a competing value.
//!
//! ## Task grade
//!
//! The first step in determining whether a task was successful or not is to determine the task grade.  This does not
//! determine success, but merely how good or bad the character did at the task.  The actual resolution is dtermined by
//! comparing the performance of the character versus the antagonist's grade.  The antagonizt could be an NPC, the
//! environment or just a generalized difficulty.
//!
//! The task grade is made by rolling the character's dice pool.  Typically, this is the skill level for any tasks
//! related to a skill, or to the attribute halved (rounded down) for tasks that are only based on a raw attribute.
//!
//! The values in the die pool are then added to the relevant attributes for the skill/task, and then compared to a
//! Target Number which is typically 20.  Any values above this Target Number are successful.  
//!
//! ```ignore
//! Kyle's character wants to shoot at a thug.  His skill level is 4 in rifles and his relevant attribute is 14.  Kyle
//! rolls [16, 5, 14, 5].  Adding his 14 to the rolls, he gets [30, 19, 28, 19], so 2 of his rolls beat the Target
//! Number of 20.
//! ```
//!
//! Alternatively, subtract the controlling attribute from 20, and any values above that are successful.
//!
//! ```ignore
//! From the example above, instead of comparing the Target Number to 20, we subtract the related attribute, So 20 - 14
//! = 6.  Looking at his rolls, the two 5's aren't above the adjusted Target Number of 6, so we still have 2 successes.
//! ```
//!
//! Because retro uses an exploding die pool, any die that rolls a natural 19 or 20 gets to do two things.  It first
//! rolls a new d20.  This repeats if the next die is also a 19+  Add the original value to the extra die that was
//! rolled, and subtract 1.  If the new die is also a 19+, add a new score, which is the previous score + the new die
//! roll. This contingues, until the new die roll is less than 19.
//!
//! ```ignore
//! Kyle rolls again, and this time gets [19, 8, 12, 14, 11].  Because he rolled a 19, he gets to roll it again, and
//! luckily enough, the second roll is a 20!.  So at this point, Kyle now has [8, 12, 14, 11, 38].  The 38 comes from
//! his original roll of 19 + 20 -1 = 38.  Since the additional die was also greater than 19, he gets to add a new
//! score by rolling a new die.  This new die comes up an 11.  So this new value is now 38 + 11 - 1 = 48.  Since the
//! additional die roll was an 11, we stop rolling extra dice.  Kyle's final roll is therefore [8, 12, 14, 11, 38. 48]
//! ```
//!
//! ## Task Resolution
//!
//! Resolution of a task is made by comparing the task grade of the protagonist against the task grade of the
//! antagonist.  A comparison is made against the grades of each opponent to determine the outcome.
//!
//! Therefore, all tasks are contested rolls; there is no fixed _threshold_ one must achieve.  Just because the player
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
//! ### Comparing grades
//!
//! Resolving a task involves comparing the grades of both opponents.  Whomever has the most successes wins, and both
//! the number of successes and the values of the successes determines the final outcome.
//!
//! First, for any roll above 20, find the highest value (and only the highest value) above 20 for either opponent.
//! They may remove the opponents highest rolls by up to that value.  The highest value is then taken off.  The
//! remaining rolls are then compared.  The opponent with the most successes wins.
//!
//! ```ignore
//! Kyle is facing off against a warrior in hand to hand combat.  Kyle's skill in martial arts is a 5, and from his
//! chosen move, his relevant attribute average is a 13.  Kyle ultimate rolls a [12, 17, 9, 16, 26].  His adversary
//! has a skill of 4/12, and in his maneuver to defend rolls [18, 10, 4, 30].  While Kyle has more successes than his
//! adversary, the adversary rolled the highest at 30.  So the adversary's roll of 30 can be used to remove 30 points
//! from Kyle's rolls, starting with his highest.  This removes his 26, and takes 4 off his next highest of 17, which
//! becomes a 13.  So Kyle's effective roll becomes [9, 12, 13, 16].  Since his attribute is 13,his effective Target
//! Number is 7 (20 - 13).  Therefore, Kyle has 4 successes.
//!
//! The adversary has to remove his highest roll, so he now has
//! ```
//!
//! ## Law of Averages
//!
//! Attribute ratings are from 0-18 with the average hanging out at 10. Skill levels range from 0 to 9.  
//!
//! ### General Difficulty levels
//!
//! The table below assumes difficulties for an average skilled person (Skill level 3).
//!
//! | Difficulty  | Academic    | years training | Successes | Skill level | 3@10 | 4@12 | 5@14
//! |-------------|-------------|----------------|-----------|-------------|--------------------
//! | Trivial     | Untrained   | 0              | 0         | 0           |
//! | Easy        | Grade       | <1             | 1         | 1
//! | Simple      | Junior      | 1-2            | 2         | 2
//! | Average     | High School | 2-3            | 3         | 3
//! | Above Avg   | Bachelor    | 3-4            | 4         | 4
//! | Difficult   | Post-Bacc   | 4-5            | 6         | 5
//! | Challenging | Master      | 6-7            | 8         | 6
//! | Epic        | Doctorate   | 8-9            | 10        | 7
//! | Legendary   | Post-Doc    | 9+             | 12        | 8
//! | Mythic      |             |                | 14        | 9
//! | Impossible  |             |                | 16
//!
//! |    | 1     | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | | 1  | 0.054 | 2  | 0.118 | 3  | 0.164 | 4  | 0.216 |
//! 5  | 0.264 | 6  | 0.329 | 7  | | 8  | | 9  |
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
//! Tasks define how actions are resolved Die examples:
//!
//! let simpleD6 = die(6) let roll = simpleD6.roll(10)
//!
//! let d6 = die(6) .exploding(Some(6)) .value(|amt| { default_rng(5, 2) }) .roll(10)

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
