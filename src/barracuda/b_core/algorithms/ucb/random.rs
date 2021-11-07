use crate::barracuda::b_core::algorithms::ucb::formula::Formula;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Random {
    std_dev: f32,
}

impl Random {
    pub fn new(std_dev: f32) -> Self {
        Self { std_dev }
    }
}

impl Formula for Random {
    fn get(&self) -> f32 {
        rand::thread_rng().gen::<f32>() * self.std_dev
    }
}
