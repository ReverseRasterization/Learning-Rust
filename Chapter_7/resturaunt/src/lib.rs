mod front_of_house;

mod back_of_the_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // unable to use and see because it's private
    }

    pub enum Appetizer { // all of it's values are also public
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }


    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

pub use crate::front_of_house::hosting; // use only creates the shortcut for the particular scope in which the use occurs. If housing::add_to_waitlist() was in a function inside a module this wouldn't work
        // ^^^ look at this for re-exporting https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use 
//use crate::front_of_house::hosting::add_to_waitlist; //idiomatic


pub fn eat_at_resturaunt() {
    /* 
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    */

    let mut meal = back_of_the_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_the_house::Appetizer::Soup;
    let order2 = back_of_the_house::Appetizer::Salad;

    hosting::add_to_waitlist() // this is what use makes easier
    //add_to_waitlist(); // idiomatic
    
}

use std::fmt::Result;
use std::io::Result as IoResult;

use std::io::{self, Write};
use std::collections::*;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}