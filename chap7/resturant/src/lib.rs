

mod front_of_house {
    // making the module public doesnt make its contents public
    // the pub keyword on a module means that it lets code in its ancestor modules refer to it,
    // but cant access inner code.
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// the above makes
// crate
//  └── front_of_house
//  ├── hosting
//  │   ├── add_to_waitlist
//  │   └── seat_at_table
//  └── serving
//      ├── take_order
//      ├── serve_order
//      └── take_payment

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super is like same as ../
        //  Using super allows us to reference an item that we know is in the parent module, 
        // which can make rearranging the module tree easier when the module is closely 
        // related to the parent, but the parent might be moved elsewhere in the module tree someday.
        super::deliver_order();
    }

    fn cook_order() {}
}


pub fn eat_at_resturant() {
    // specifying absolute paths is better in case you want to move code around
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
