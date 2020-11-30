//! This module declares a Task
///
/// A Task defines anything which is uncertain in game terms and how to accomplish it.  Tasks revolve around a die pool
/// and are always opposed.  The die pool is d10 based and is _exploding_.  If a 10 is rolled, the die can be rerolled
/// again.  


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
