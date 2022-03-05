// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Dimensions {
  width: f64,
  height: f64,
  depth: f64,
}

impl Dimensions {
  fn print(&self) {
    println!("width: {}cm", self.width);
    println!("height: {}cm", self.height);
    println!("depth: {}cm", self.depth);
  }
}

// * Use a struct to encapsulate the box characteristics
struct Box {
  dimensions: Dimensions,
  weight: f64,
  color: BoxColor,
}

// * Use an enum for the box color
enum BoxColor {
  Brown,
  White
}

impl BoxColor {
  fn print(&self) {
    match self {
      BoxColor::Brown => println!("color: brown"),
      BoxColor::White => println!("color: white"),
    }
  }
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl Box {
  fn new(weight: f64, color: BoxColor, dimensions: Dimensions) -> Self {
    Self {
      weight,
      color,
      dimensions,
    }
  }

  fn print(&self) {
    self.color.print();
    self.dimensions.print();
    println!("weight: {}g", self.weight);
  }
}


fn main() {
  let box_dimensions = Dimensions {
    width: 100.0,
    height: 100.0,
    depth: 100.0,
  };

  let my_box = Box::new(150.0, BoxColor::Brown, box_dimensions);

  my_box.print();
}
