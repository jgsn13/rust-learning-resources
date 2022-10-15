// NOTE: inner modules/functions/variables are private by default
#![allow(unused)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// 'super' keyword
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // reference to the parent element with 'super' keyword
        super::serve_order();
    }

    fn cook_order() {}
}
