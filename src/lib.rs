mod front_of_house;

pub mod back_of_the_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches")
      }
    }
  }
}

pub use crate::front_of_house::hosting as FrontHosting;

mod customer {
  use crate::{back_of_the_house as BackOfTheHouse, front_of_house::hosting as FrontHosting};
  pub fn eat_at_restaurant() {
    FrontHosting::add_to_waitlist();
  
    let mut meal = BackOfTheHouse::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
  }
}
