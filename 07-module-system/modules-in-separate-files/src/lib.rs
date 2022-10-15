// NOTE: this tells rust: define the front_of_house module here, but get the contents from a
// different file with the same name as the module;
mod front_of_house;

pub use crate::front_of_house::{hosting, serving};

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}
