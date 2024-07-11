use front_of_house::hosting;

mod front_of_house{
    pub mod hosting{
        pub(crate) fn add_to_waitlist(){
            println!("add to wait list");
        }
        pub(crate) fn seat_to_table(){
            println!("seat to table");
        }
    }
    mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
    hosting::seat_to_table();
}