use crate::barracuda::b_core::algorithms::ucb::formula::Formula;
use std::sync::{Arc, Mutex};

pub struct Ucb {
    formula: Arc<Mutex<dyn Formula>>,
}

impl Ucb {
    pub fn new(formula: Arc<Mutex<dyn Formula>>) -> Self {
        Self { formula }
    }
}

impl Ucb {
    pub fn ucb(&self) -> f32 {
        self.formula.lock().unwrap().get()
    }
}
