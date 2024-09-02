use crate::garden::vegetables::Asparagus;

pub mod garden; // tells the compiler to include the code it finds in src/garden.rs

fn main() {
    let plant = Asparagus {};
    println!("Im growing {plant:?}!");
}
