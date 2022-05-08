//! Tasks define how actions are resolved
//! Die examples:
//!
//! ```
//! use retro::tasks::probability::{default_rng, die, DieTraits, DiePool};
//! 
//! let simpleD6 = die(6);
//! let roll = simpleD6();
//! println!("roll is {roll}");
//!
//! let d6 = DiePool::pool(6)
//!   .exploding(Some(6))
//!   .value(Box::new(|amt| {
//!      default_rng(amt, 5, 2)
//!   }));
//! 
//! let roll =  d6.roll(10);
//! println!("roll is {roll:#?}");
//! ```

use rand::Rng;

pub trait DieTraits {
    // fn value(self: Self, calc: Box<dyn Fn(u32) -> u32>) -> Self;
    // fn exploding(self: Self, opt: Option<u32>) -> Self;

    /// Generates a roll of the die
    fn roll(self: &Self, num: u32) -> Vec<u32>;

    /// Calculates how many successes there are
    fn get_successes(self: &Self, dice: u32, accum: impl FnMut(u32, &u32) -> u32) -> (u32, Vec<u32>) {
        let roll = self.roll(dice);
        let successes = roll.iter().fold(0, accum);
        (successes, roll)
    }

    fn exploding(self: &Self, roll: &mut Vec<u32>, thresh: u32, die: impl Fn(u32) -> Vec<u32>) -> Vec<u32> {
        //&mut let mut roll = self.roll(num);
        let mut eroll: Vec<u32> = vec![];
    
        // Calculate which rolls are greater than threshold.  These dice will get extra rolls
        for d in roll.iter() {
            if *d >= thresh {
                eroll.append(&mut die(1));
            }
        }
    
        let mut exploded = if !eroll.is_empty() {
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
}

pub struct DiePool {
    pub facings: u32,
    exploding: Option<u32>,
    calculate: Box<dyn Fn(u32) -> u32>,
    dice: Box<dyn Fn(u32) -> Vec<u32>>,
}

impl DiePool {
    pub fn pool(facings: u32) -> Self {
        let dice = Box::new(move |num| dice(num, facings));

        DiePool {
            facings,
            exploding: None,
            calculate: Box::new(id),
            dice,
        }
    }

    pub fn exploding(mut self: Self, val: Option<u32>) -> Self {
        self.exploding = val;
        self
    }

    pub fn value(mut self: Self, calc: Box<dyn Fn(u32) -> u32>) -> Self {
        self.calculate = calc;
        self
    }
}

impl DieTraits for DiePool {
    fn roll(self: &Self, num: u32) -> Vec<u32> {
        let dice_roll = (self.dice)(num);
        let mut rolled = dice_roll
            .into_iter()
            .map(|amt| (self.calculate)(amt))
            .collect();

        if let Some(thresh) = self.exploding {
            exploding(&mut rolled, thresh, |num_d| dice(num_d, self.facings))
        } else {
            rolled
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

pub fn default_rng(val: u32, thresh_high: u32, thresh_low: u32) -> u32 {
    if thresh_low >= thresh_high {
        panic!(
            "thresh_low {} can not be >= to thresh_high {}",
            thresh_low, thresh_high
        );
    }
    match val {
        x if x >= thresh_high => 2,
        x if x <= thresh_low => 1,
        _ => 0,
    }
}

pub fn id<T>(x: T) -> T {
    x
}

/// Returns a new Vec of u32, where for each value in the original roll, if it is >= thresh, a new die roll is added
pub fn explode(roll: &Vec<u32>, thresh: u32, die: impl Fn(u32) -> Vec<u32>) -> Vec<u32> {
    let mut eroll: Vec<u32> = vec![];
    for d in roll {
        eroll.push(*d);
        if *d <= thresh {
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

    let roll_more = eroll.iter().any(|result| *result >= thresh);
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
        let pool = DiePool::pool(20).exploding(Some(2));
        let roll = pool.roll(4);

        println!("roll {:?}", roll);
    }

    fn _get_successes(pool: &DiePool, dice: u32, target: u32) -> (u32, Vec<u32>) {
        let roll = pool.roll(dice);
        let successes = roll.iter().fold(
            0u32,
            |acc, next| {
                if *next >= target {
                    acc + 1
                } else {
                    acc
                }
            },
        );
        (successes, roll)
    }

    #[test]
    fn average10of20() {
        let mut avg: Vec<u32> = vec![];
        let pool = DiePool::pool(20).exploding(Some(19));
        for n in 0..100 {
            let (successes, roll) = pool.get_successes(6, |acc, next| {
                if *next >= 11 {
                    acc + 1
                } else {
                    acc
                }
            });
            //let (successes, roll) = get_successes(&pool, 6, 10);
            avg.push(successes);
            println!("{}: Successes = {} from {:?}", n, successes, roll);
        }

        let sum_avg = avg.iter().fold(0u32, |acc, next| acc + next) as f32;
        let calc_avg = sum_avg / 100.0;
        println!("Sum = {}, Calculated average is {}", sum_avg, calc_avg);

        let mut scores: HashMap<u32, u32> = HashMap::new();
        let _foo = avg.into_iter().fold(&mut scores, |acc, next| {
            if !acc.contains_key(&next) {
                acc.insert(next, 1);
            } else {
                acc.insert(next, acc[&next] + 1);
            };
            acc
        });
        println!("Score is {:#?}", scores);
    }
}
