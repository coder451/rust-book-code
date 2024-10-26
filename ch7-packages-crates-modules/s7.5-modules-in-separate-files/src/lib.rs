pub fn deliver_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // <-- super
    }

    fn cook_order() {}

    pub struct Breakfast {        // Public struct
        pub toast: String,        // public member
        seasonal_fruit: String,   // private member
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn meal_type(&self) -> String {
            String::from("breakfast")
        }

        pub fn seasonal_type(&self) -> String {
            String::from("fruit")
        }
        pub fn seasonal_name(&self) -> &String {
            &self.seasonal_fruit
        }
    }

    // A public enum
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

pub mod customer {
    // Bring hosting module into this scope
    use crate::front_of_house::hosting;
    use crate::front_of_house::serving;

    pub fn eat_at_restaurant() {

        // Access a hosting function
        // The hosting module is public and so is the add_to_waitlist 
        // function
        hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        println!("The seasonal {} for {} is {}", 
            meal.seasonal_type(), 
            meal.meal_type(),
            meal.seasonal_name());

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        //meal.seasonal_fruit = String::from("blueberries");

        // Variants of a public enum are public
        let _order1 = super::back_of_house::Appetizer::Soup;
        let _order2 = super::back_of_house::Appetizer::Salad;

        hosting::seat_at_table();
        serving::take_order();
        serving::serve_order();
        serving::take_payment();

    }
}
