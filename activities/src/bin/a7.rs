// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Colour {
  White,
  Blue,
  Grey,
  Red,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_colour_name(colour: Colour) {
  // * Use a match expression to determine which color
  //   name to print
  match colour {
    Colour::White => println!("white"),
    Colour::Blue => println!("blue"),
    Colour::Grey => println!("grey"),
    Colour::Red => println!("red"),
  }
}

fn main() {
  let my_colour = Colour::White;

  print_colour_name(my_colour);
}
