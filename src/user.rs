use std::collections::HashMap;
use crate::bike::{Bike, self};

use std::fmt::Debug;

pub struct User {
    pub email: String,
    pub bikes: HashMap<String, Bike>,
}

impl User {
    pub fn new (email: &str) -> Self {
        Self {
            email: email.to_string(),
            bikes: HashMap::new()
        }
    }

    pub fn reg (&mut self, name: &str, model: &str, mileage: u64, year: &str, price: f64) {
        let bike: Bike = Bike::new(name.to_string(), model.to_string(), mileage, year.to_string(), price);
        let key = self.email.as_str();
        self.bikes.insert(key.to_string(), bike);
    }
    
    pub fn show_bikes (&self) -> Option<&Bike> {
        match self.bikes.get (&self.email) {
            Some(bike) => Some(bike),
            None => None
        }
    }
    
    pub fn remove_bike (&mut self) {
        self.bikes.remove(&self.email);
    }
}

