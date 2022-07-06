#[derive(Debug)]
pub struct Bike {
    pub name: String,
    pub model: String,
    pub mileage: u64,
    pub year: String,
    pub price: f64
}

impl Bike {
    pub fn new(name: String, model: String, mileage: u64, year: String, price: f64) -> Self {
        Self { name, model, mileage, year, price }
    }
}