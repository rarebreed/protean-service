//! # This module declares a Task
//!
//! A Task defines anything which is uncertain in game terms and how to accomplish it.  Tasks revolve around a die pool
//! and are always opposed.  The die pool is d10 based and is _exploding_.  If a 10 is rolled, the die can be rerolled
//! again. 
//!
//! ## Associated Characteristics and Skills
//!
//! All tasks have a set of associated Characteristics associated, and some tasks have a set of associated skills.  This
//! is an important concept to distinguish.  Some tasks might be based on raw innate abilities.  Examples could include:
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
//! characteristic is.  It is thus possible, for someone extremely talented but not as skilled, to best someone who is
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
//! The general rule of thumb is that changing the target number for non-adverserial tasks should be changed only when
//! skill matters less, and the task is more one of luck.  For example, what's the difference between a task difficulty
//! with a TN of 15 and a pool of 2, versus a TN of 10 and a pool of 4?  In a non-adversarial contest, what is the pool
//! and TN even based off of?
//!
//! Take for example shooting at a non-moving distant target.  In broad daylight with minimal winds, the various factors
//! are:
//!
//! - distance
//! - size of the target
//! - visibility
//! - inherent accuracy of the weapon
//!
//! Actually the distance and size of the target for a non-moving object boil down to one value, which is _effective
//! size_.  We will go over more detail about this in the full combat section


pub enum TaskType {
    Gradual,   
    Team,
    Normal    // Normal tasks are all opposed, even against just the environment
}


pub struct Task {
    pub name: String,      // A descriptive name
    pub difficulty: u32,   // 
    pub t_type: TaskType,
    pub time: f64
}
