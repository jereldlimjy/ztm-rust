// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavour {
  Strawberry,
  Grape,
  Blueberry,
  Orange,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
  flavour: Flavour,
  amount: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
  // * Use a match expression to print the drink flavor
  match drink.flavour {
    Flavour::Strawberry => println!("Strawberry"),
    Flavour::Grape => println!("Grape"),
    Flavour::Blueberry => println!("Blueberry"),
    Flavour::Orange => println!("Orange"),
  }

  println!("Ounces: {}", drink.amount);
}

fn main() {
  let my_drink = Drink {
    flavour: Flavour::Grape,
    amount: 4.0,
  };

  print_drink(my_drink);
}
