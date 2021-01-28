#[macro_use]
extern crate serde_derive;
use num::rational::{BigRational};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Currency {
    pub code: String,
    pub net_debit_cap: BigRational
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participant {
    pub name: String,
    pub currencies: Vec<Currency>
}

fn main() {
    println!("Hello, world!");
}
