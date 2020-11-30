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

use rand::Rng;

pub trait DieTraits {
    // fn value(self: Self, calc: Box<dyn Fn(u32) -> u32>) -> Self;
    // fn exploding(self: Self, opt: Option<u32>) -> Self;
    fn roll(self: &Self, num: u32) -> Vec<u32>;
}

pub struct DiePool {
    pub facings: u32,
    exploding: Option<u32>,
    calculate: Box<dyn Fn(u32) -> u32>,
    dice: Box<dyn Fn(u32) -> Vec<u32>>
}

impl DiePool {
    pub fn pool(facings: u32) -> Self {
        let dice = Box::new(move |num| {
            dice(num, facings)
        });

        DiePool {
            facings,
            exploding: None,
            calculate: Box::new(id),
            dice
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
        let mut rolled = dice_roll.into_iter()
            .map(|amt| {
                (self.calculate)(amt)
            })
            .collect();

        if let Some(thresh) = self.exploding {
            exploding(&mut rolled, thresh, |num_d| { dice(num_d, self.facings) })
        }else {
            rolled
        } 
    }
}

/// Function that will return from 1-size to simulate a single die
pub fn die(size: u32) -> impl Fn() -> u32 {
    move || {
        let mut rng = rand::thread_rng();
        rng.gen_range(1, size+1)
    }
}

/// Rolls a dum of dice of a certain size. dice(10, 6) means roll 10d6.  dice(6, 10) means roll 6d10
pub fn dice(num: u32, size: u32) -> Vec<u32> {
    let d = die(size);
    (0..num)
        .map(|_| { d() })
        .collect()
}

pub fn default_rng(val: u32, thresh_high: u32, thresh_low: u32) -> u32 {
    if thresh_low >= thresh_high {
        panic!("thresh_low {} can not be >= to thresh_high {}", thresh_low, thresh_high);
    }
    match val {
        x if x >= thresh_high => 2,
        x if x <= thresh_low => 1,
        _ => 0
    }
}

pub fn id<T>(x: T) -> T { x }

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

    let roll_more = eroll.iter().any(|result| {
        *result >= thresh
    });
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
    };
    final_roll
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool4d10() {
        println!("Testing pool");
        let pool = DiePool::pool(20).exploding(Some(19));
        let roll = pool.roll(4);

        println!("roll {:?}", roll);
    }

    #[test]
    fn average() {
        let mut avg: Vec<u32> = vec![];
        let pool = DiePool::pool(20).exploding(Some(19));
        for n in 0..500 {
            let roll = pool.roll(5);
            let successes = roll.iter()
                .fold(0u32, |acc, next| {
                    if *next >= 10 { acc + 1 } else { acc }
                });
            avg.push(successes);
            println!("{}: Successes = {} from {:?}", n, successes, roll);
        }

        let sum_avg = avg.into_iter()
            .fold(0u32, |acc, next| {
                acc + next
            }) as f32;
        let calc_avg = sum_avg / 500.0;
        println!("Calculated average is {}", calc_avg);
    }
}