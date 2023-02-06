//! Tasks define how actions are resolved
//! Die examples:
//!
//! ```
//! use retro::engine::probability::{default_rng, die, DieTraits, DiePool};
//!
//! let simpleD6 = die(6);
//! let roll = simpleD6();
//! println!("roll is {roll}");
//!
//! let d6 = DiePool::new(6)
//!   .value(Box::new(|amt| {
//!      default_rng(amt, 5, 2)
//!   }));
//!
//! let roll =  d6.roll(10);
//! println!("roll is {roll:#?}");
//! ```

use log::debug;
use rand::Rng;

/// The kind of a dice roll, either the original roll, or after checking for exploded values
pub enum Roll {
    /// the first roll
    Original(Vec<u32>),
    /// exploded rolls
    Exploded(Vec<(u32, u32)>),
}

/// Functionality of a die
pub trait DieTraits {
    /// Generates a roll of the die
    fn roll(&self, num: u32) -> Vec<u32>;

    /// Calculates how many successes there are
    fn get_successes(&self, dice: u32, accum: impl FnMut(u32, &u32) -> u32) -> (u32, Vec<u32>) {
        let roll = self.roll(dice);
        let successes = roll.iter().fold(0, accum);
        (successes, roll)
    }

    /// Rolls the poll, and returns a tuple of (success values, all_values)
    fn get_all_successes(
        &self,
        dice: u32,
        accum: impl FnMut(Vec<u32>, &u32) -> Vec<u32>,
    ) -> (Vec<u32>, Vec<u32>) {
        let roll = self.roll(dice);
        let successes = roll.iter().fold(vec![], accum);
        (successes, roll)
    }

    /// A die can "explode", meaning if it rolls some value, it can be rerolled.
    ///
    /// How the re-rolling is accounted for is handled by the `die` fn.  In some cases you may want
    /// to sum up the values, in other cases, you may want to add to the number of successes.
    fn exploding(
        &self,
        orig_roll: &mut Roll,
        thresh: u32,
        die: impl Fn(u32) -> Vec<u32>,
    ) -> Vec<(u32, u32)> {
        let mut eroll: Vec<(u32, u32)> = vec![];

        // convert orig_roll to a &mut Vec<(u32, u32)>
        let mut converted = vec![];
        let (roll, subtract) = match orig_roll {
            Roll::Original(roll) => {
                println!("Original roll: {roll:?}");
                for val in roll {
                    if *val >= thresh {
                        converted.push((0u32, *val))
                    } else {
                        converted.push((*val, 0u32))
                    }
                }
                (&mut converted, 1)
            }
            Roll::Exploded(roll) => (roll, 0),
        };

        // Calculate which elements in roll
        for (orig, new) in roll.iter_mut() {
            if *new >= thresh {
                let val = die(1)[0];
                if *orig == 0 {
                    eroll.push((*new + val - subtract, val));
                } else {
                    eroll.push((*orig + val - subtract, val));
                }
            }
        }
        if !eroll.is_empty() {
            println!("    Rolled an extra {eroll:?}");
        };
        // Take out any that have a 0
        let mut roll: Vec<(u32, u32)> = roll
            .iter_mut()
            .filter(|(orig, _)| *orig != 0)
            .map(|(orig, new)| (*orig, *new))
            .collect();

        let roll_more = eroll.iter().any(|(_, new)| *new >= thresh);
        let mut exploded = if roll_more {
            let mut extended = Roll::Exploded(eroll);
            self.exploding(&mut extended, thresh, die)
        } else {
            eroll
        };

        // This is kind of expensive to do, but I think this is better than returning a reference
        roll.append(&mut exploded);
        debug!("after exploding: {roll:?}");
        roll
    }
}

/// A virtual pool of dice
///
/// In many RPG's, you can roll a number of dice simultaneously.  Sometimes the die value for
/// each dice are summed up, and sometimes each die has to be compared against some threshold
/// and the total counted.  Some dice also can "explode".
///
/// The `calculate` trait object tells us what to with all the dice, while the `die` trait object
/// tells us how each die individually behaves.
pub struct DiePool {
    /// How many sides the dice have
    pub facings: u32,
    exploding: Option<u32>,
    calculate: Box<dyn Fn(u32) -> u32>,
    dice: Box<dyn Fn(u32) -> Vec<u32>>,
}

impl DiePool {
    /// Create a DiePool
    pub fn new(facings: u32) -> Self {
        let dice = Box::new(move |num| dice(num, facings));

        DiePool {
            facings,
            exploding: None,
            calculate: Box::new(id),
            dice,
        }
    }

    /// Set whether the dice can explode and what the threshold is
    ///
    /// None means no exploding, while 19 means on a 19+ the die will explode
    pub fn exploding(mut self, val: Option<u32>) -> Self {
        self.exploding = val;
        self
    }

    ///
    pub fn value(mut self, calc: Box<dyn Fn(u32) -> u32>) -> Self {
        self.calculate = calc;
        self
    }
}

impl DieTraits for DiePool {
    fn roll(&self, num: u32) -> Vec<u32> {
        let dice_roll = (self.dice)(num);
        let roll: Vec<u32> = dice_roll
            .into_iter()
            .map(|amt| (self.calculate)(amt))
            .collect();
        debug!("original roll: {roll:?}");
        if let Some(thresh) = self.exploding {
            let final_roll = self.exploding(&mut Roll::Original(roll), thresh, |_| (self.dice)(1));
            final_roll
                .into_iter()
                .map(|(orig, _)| orig)
                .collect::<Vec<u32>>()
        } else {
            roll
        }
    }
}

/// Function that will return from 1-size to simulate a single die
pub fn die(size: u32) -> impl Fn() -> u32 {
    move || {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..size + 1)
    }
}

/// Rolls a dum of dice of a certain size. dice(10, 6) means roll 10d6.  dice(6, 10) means roll 6d10
pub fn dice(num: u32, size: u32) -> Vec<u32> {
    let d = die(size);
    (0..num).map(|_| d()).collect()
}

/// Returns 2 if val >= thresh_high, 1 if val <= thresh_low, and 0 otherwise  
pub fn default_rng(val: u32, thresh_high: u32, thresh_low: u32) -> u32 {
    if thresh_low >= thresh_high {
        panic!("thresh_low {thresh_low} can not be >= to thresh_high {thresh_high}");
    }
    match val {
        x if x >= thresh_high => 2,
        x if x <= thresh_low => 1,
        _ => 0,
    }
}

/// Identity function (passthrough)
pub fn id<T>(x: T) -> T {
    x
}

/// Returns a new Vec of u32, where for each value in the original roll, if it is >= thresh, a new die roll is added
pub fn explode(roll: &Vec<u32>, thresh: u32, die: impl Fn(u32) -> Vec<u32>) -> Vec<u32> {
    let mut eroll: Vec<u32> = vec![];
    for d in roll {
        eroll.push(*d);
        if *d >= thresh {
            eroll.append(&mut die(1));
        }
    }
    eroll
}

/// Searches through the dice roll and checks if any values >= to the threshold
///
/// If any are, it will create roll one die, for each value >= to the thresh.  It will recursively check this new list
/// of die roll(s).  Once there are no more exploding dice, it will append the rolls all together.
pub fn exploding(roll: &mut Vec<u32>, thresh: u32, die: impl Fn(u32) -> Vec<u32>) -> Vec<u32> {
    let mut eroll: Vec<u32> = vec![];

    for d in roll.iter() {
        if *d >= thresh {
            eroll.append(&mut die(1));
        }
    }

    let roll_more = eroll.iter().any(|result| *result <= thresh);
    let mut exploded = if roll_more {
        let mut new_roll = exploding(&mut eroll, thresh, die);
        eroll.append(&mut new_roll);
        eroll
    } else {
        eroll
    };

    // This is kind of expensive to do, but I think this is better than returning a reference
    roll.append(&mut exploded);
    let mut final_roll: Vec<u32> = vec![];
    for r in roll {
        final_roll.push(*r);
    }
    final_roll
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn pool4d20() {
        println!("Testing pool");
        let pool = DiePool::new(6);
        //.exploding(Some(20));
        let roll = pool.roll(2);

        println!("roll {roll:?}");
    }

    /// keep highest X
    fn get_highest(pool: &DiePool, dice: u32, take: usize) -> (Vec<u32>, Vec<u32>) {
        let mut roll = pool.roll(dice);
        roll.sort_unstable();
        roll.reverse();
        let successes = roll.iter().take(take).copied().collect::<Vec<u32>>();
        (successes, roll)
    }

    #[test]
    fn test_3_of_6() {
        let pool = DiePool::new(10).exploding(Some(10));
        let (highest, roll) = get_highest(&pool, 6, 3);
        println!("{highest:?}, {roll:?}");
    }

    fn calculate_average(dice: u32, target: u32, thresh: u32) {
        let mut avg: Vec<u32> = vec![];
        let pool = DiePool::new(10).exploding(Some(thresh));
        for n in 0..100 {
            let (successes, roll) =
                pool.get_successes(
                    dice,
                    |acc, next| if *next >= target { acc + 1 } else { acc },
                );
            //let (successes, roll) = get_successes(&pool, 6, 10);
            avg.push(successes);
            println!("{n}: Successes = {successes} from {roll:?}");
        }

        //let sum_avg = avg.iter().fold(0u32, |acc, next| acc + next) as f32;
        let sum_avg: u32 = avg.iter().sum();
        let calc_avg = sum_avg as f32 / 100.0;
        println!("Sum = {sum_avg}, Calculated average is {calc_avg}");

        let mut scores: HashMap<u32, u32> = HashMap::new();
        let scores = avg.into_iter().fold(&mut scores, |acc, next| {
            acc.entry(next).and_modify(|e| *e += 1).or_insert(1);
            acc
        });

        println!("Score for num dice {dice}, at target {target} with thresh {thresh}:");
        let mut ordered_keys = scores.keys().collect::<Vec<&u32>>();
        ordered_keys.sort();
        for key in ordered_keys {
            let val = scores.get(key).unwrap();
            println!("    {key}: {val}");
        }
    }

    #[test]
    fn test_simple() {
        calculate_average(9, 1, 10);
    }

    #[test]
    fn test_9_dice() {
        calculate_average(9, 1, 19);
        calculate_average(9, 2, 19);
        calculate_average(9, 3, 19);
        calculate_average(9, 4, 19);
        calculate_average(9, 5, 19);
        calculate_average(9, 6, 19);
        calculate_average(9, 7, 19);
        calculate_average(9, 8, 19);
        calculate_average(9, 9, 19);
        calculate_average(9, 10, 19);
        calculate_average(9, 11, 19);
        calculate_average(9, 12, 19);
        calculate_average(9, 13, 19);
        calculate_average(9, 14, 19);
        calculate_average(9, 15, 19);
        calculate_average(9, 16, 19);
        calculate_average(9, 17, 19);
        calculate_average(9, 18, 19);
        calculate_average(9, 19, 19);
        calculate_average(9, 20, 19);
    }

    #[test]
    fn test_8_dice() {
        calculate_average(8, 1, 19);
        calculate_average(8, 2, 19);
        calculate_average(8, 3, 19);
        calculate_average(8, 4, 19);
        calculate_average(8, 5, 19);
        calculate_average(8, 6, 19);
        calculate_average(8, 7, 19);
        calculate_average(8, 8, 19);
        calculate_average(8, 9, 19);
        calculate_average(8, 10, 19);
        calculate_average(8, 11, 19);
        calculate_average(8, 12, 19);
        calculate_average(8, 13, 19);
        calculate_average(8, 14, 19);
        calculate_average(8, 15, 19);
        calculate_average(8, 16, 19);
        calculate_average(8, 17, 19);
        calculate_average(8, 18, 19);
        calculate_average(8, 19, 19);
        calculate_average(8, 20, 19);
    }

    #[test]
    fn test_7_dice() {
        calculate_average(7, 1, 19);
        calculate_average(7, 2, 19);
        calculate_average(7, 3, 19);
        calculate_average(7, 4, 19);
        calculate_average(7, 5, 19);
        calculate_average(7, 6, 19);
        calculate_average(7, 7, 19);
        calculate_average(7, 8, 19);
        calculate_average(7, 9, 19);
        calculate_average(7, 10, 19);
        calculate_average(7, 11, 19);
        calculate_average(7, 12, 19);
        calculate_average(7, 13, 19);
        calculate_average(7, 14, 19);
        calculate_average(7, 15, 19);
        calculate_average(7, 16, 19);
        calculate_average(7, 17, 19);
        calculate_average(7, 18, 19);
        calculate_average(7, 19, 19);
        calculate_average(7, 20, 19);
    }

    #[test]
    fn test_6_dice() {
        calculate_average(6, 1, 19);
        calculate_average(6, 2, 19);
        calculate_average(6, 3, 19);
        calculate_average(6, 4, 19);
        calculate_average(6, 5, 19);
        calculate_average(6, 6, 19);
        calculate_average(6, 7, 19);
        calculate_average(6, 8, 19);
        calculate_average(6, 9, 19);
        calculate_average(6, 10, 19);
        calculate_average(6, 11, 19);
        calculate_average(6, 12, 19);
        calculate_average(6, 13, 19);
        calculate_average(6, 14, 19);
        calculate_average(6, 15, 19);
        calculate_average(6, 16, 19);
        calculate_average(6, 17, 19);
        calculate_average(6, 18, 19);
        calculate_average(6, 19, 19);
        calculate_average(6, 20, 19);
    }

    #[test]
    fn test_5_dice() {
        calculate_average(5, 1, 19);
        calculate_average(5, 2, 19);
        calculate_average(5, 3, 19);
        calculate_average(5, 4, 19);
        calculate_average(5, 5, 19);
        calculate_average(5, 6, 19);
        calculate_average(5, 7, 19);
        calculate_average(5, 8, 19);
        calculate_average(5, 9, 19);
        calculate_average(5, 10, 19);
        calculate_average(5, 11, 19);
        calculate_average(5, 12, 19);
        calculate_average(5, 13, 19);
        calculate_average(5, 14, 19);
        calculate_average(5, 15, 19);
        calculate_average(5, 16, 19);
        calculate_average(5, 17, 19);
        calculate_average(5, 18, 19);
        calculate_average(5, 19, 19);
        calculate_average(5, 20, 19);
    }

    #[test]
    fn test_4_dice() {
        calculate_average(4, 1, 19);
        calculate_average(4, 2, 19);
        calculate_average(4, 3, 19);
        calculate_average(4, 4, 19);
        calculate_average(4, 5, 19);
        calculate_average(4, 6, 19);
        calculate_average(4, 7, 19);
        calculate_average(4, 8, 19);
        calculate_average(4, 6, 19);
        calculate_average(4, 7, 19);
        calculate_average(4, 8, 19);
        calculate_average(4, 9, 19);
        calculate_average(4, 10, 19);
        calculate_average(4, 11, 19);
        calculate_average(4, 12, 19);
        calculate_average(4, 13, 19);
        calculate_average(4, 14, 19);
        calculate_average(4, 15, 19);
        calculate_average(4, 16, 19);
        calculate_average(4, 17, 19);
        calculate_average(4, 18, 19);
        calculate_average(4, 19, 19);
        calculate_average(4, 20, 19);
    }

    #[test]
    fn test_3_dice() {
        calculate_average(3, 1, 19);
        calculate_average(3, 2, 19);
        calculate_average(3, 3, 19);
        calculate_average(3, 4, 19);
        calculate_average(3, 5, 19);
        calculate_average(3, 6, 19);
        calculate_average(3, 7, 19);
        calculate_average(3, 8, 19);
        calculate_average(3, 9, 19);
        calculate_average(3, 10, 19);
        calculate_average(3, 11, 19);
        calculate_average(3, 12, 19);
        calculate_average(3, 13, 19);
        calculate_average(3, 14, 19);
        calculate_average(3, 15, 19);
        calculate_average(3, 16, 19);
        calculate_average(3, 17, 19);
        calculate_average(3, 18, 19);
        calculate_average(3, 19, 19);
        calculate_average(3, 20, 19);
    }

    #[test]
    fn test_2_dice() {
        calculate_average(2, 1, 19);
        calculate_average(2, 2, 19);
        calculate_average(2, 3, 19);
        calculate_average(2, 4, 19);
        calculate_average(2, 5, 19);
        calculate_average(2, 6, 19);
        calculate_average(2, 7, 19);
        calculate_average(2, 8, 19);
        calculate_average(2, 9, 19);
        calculate_average(2, 10, 19);
        calculate_average(2, 11, 19);
        calculate_average(2, 12, 19);
        calculate_average(2, 13, 19);
        calculate_average(2, 14, 19);
        calculate_average(2, 15, 19);
        calculate_average(2, 16, 19);
        calculate_average(2, 17, 19);
        calculate_average(2, 18, 19);
        calculate_average(2, 19, 19);
        calculate_average(2, 20, 19);
    }

    #[test]
    fn test_1_dice() {
        calculate_average(1, 1, 19);
        calculate_average(1, 2, 19);
        calculate_average(1, 3, 19);
        calculate_average(1, 4, 19);
        calculate_average(1, 5, 19);
        calculate_average(1, 6, 19);
        calculate_average(1, 7, 19);
        calculate_average(1, 8, 19);
        calculate_average(1, 9, 19);
        calculate_average(1, 10, 19);
        calculate_average(1, 11, 19);
        calculate_average(1, 12, 19);
        calculate_average(1, 13, 19);
        calculate_average(1, 14, 19);
        calculate_average(1, 15, 19);
        calculate_average(1, 16, 19);
        calculate_average(1, 17, 19);
        calculate_average(1, 18, 19);
        calculate_average(1, 19, 19);
        calculate_average(1, 20, 19);
    }

    #[test]
    fn test_5_at_3() {}

    #[test]
    fn test_rng() {
        let d6 = DiePool::new(6)
            //.exploding(Some(0))
            .value(Box::new(|amt| default_rng(amt, 5, 2)));

        let roll = d6.roll(10);
        println!("roll is {roll:#?}");
    }
}
