#![allow(unused)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// NOTE: now we are bring in the 'hosting' module into scope by using the 'use' keyword but we are
// also making this public, so external code could reference 'hosting' as well.
pub use crate::front_of_house::hosting;
// 'self' specify the current module
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// using the 'as' keyword
mod example1 {
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        // --snip--
        Ok(())
    }

    fn function2() -> io::Result<()> {
        // --snip--
        Ok(())
    }
}

mod example2 {
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        // --snip--
        Ok(())
    }

    fn function2() -> IoResult<()> {
        // --snip--
        Ok(())
    }
}
