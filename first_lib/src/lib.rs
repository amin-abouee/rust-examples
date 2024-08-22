use std::fmt;
use rand::Rng;


use std::{collections::HashMap, io};

use std::fmt::{self, DebugMap};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add to waitlist");
        }
    }
}


mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_resturant() {
        hosting::add_to_waitlist();
    }
}
