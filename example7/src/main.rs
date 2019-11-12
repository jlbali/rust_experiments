mod front_of_house; // Para traer el archivo.

use front_of_house::hosting;

fn main() {
    eat_at_restaurant();
}




mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        pub fn show_toast(&self) {
            println!("I would like to toast with {}", self.toast);
        }
    }
}

use back_of_house::Breakfast;

pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer("Rye"); // Sin el use.
    // Cambio de planes con le brindis.
    meal.toast = String::from("Wheat");
    meal.show_toast();

    let mut meal2 = Breakfast::summer("Wine"); // usando el atajo del use.
    meal2.show_toast();

    hosting::add_to_waitlist("Arturo");
}