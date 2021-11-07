use std::sync::{Arc, Mutex};

pub trait Formula: Send {
    fn get(&self) -> f32;
}

pub struct Const {
    value: f32,
}

impl Const {
    pub fn new(value: f32) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { value }))
    }
}

impl Formula for Const {
    fn get(&self) -> f32 {
        self.value
    }
}

pub struct Add {
    parts: Vec<Arc<Mutex<dyn Formula>>>,
}

pub struct Sub<T: Formula, U: Formula> {
    t: T,
    u: U,
}

pub struct Mul {
    parts: Vec<Arc<Mutex<dyn Formula>>>,
}

pub struct Div<T: Formula, U: Formula> {
    t: T,
    u: U,
}

impl Add {
    pub fn new(parts: Vec<Arc<Mutex<dyn Formula>>>) -> Self {
        Self { parts }
    }
}

impl<T: Formula, U: Formula> Sub<T, U> {
    pub fn new(t: T, u: U) -> Self {
        Self { t, u }
    }
}

impl Mul {
    pub fn new(parts: Vec<Arc<Mutex<dyn Formula>>>) -> Self {
        Self { parts }
    }
}

impl<T: Formula, U: Formula> Div<T, U> {
    pub fn new(t: T, u: U) -> Self {
        Self { t, u }
    }
}

impl Formula for Add {
    fn get(&self) -> f32 {
        self.parts
            .iter()
            .map(|part| part.lock().unwrap().get())
            .sum()
    }
}

impl<T: Formula, U: Formula> Formula for Sub<T, U> {
    fn get(&self) -> f32 {
        self.t.get() - self.u.get()
    }
}

impl Formula for Mul {
    fn get(&self) -> f32 {
        self.parts
            .iter()
            .map(|part| part.lock().unwrap().get())
            .product()
    }
}

impl<T: Formula, U: Formula> Formula for Div<T, U> {
    fn get(&self) -> f32 {
        self.t.get() / self.u.get()
    }
}
