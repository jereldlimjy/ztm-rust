// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
  name: String,
  age: i32,
  favourite_color: String,
}

impl Person {
  fn print(&self) {
    println!("name is: {}", self.name);
    println!("favourite color is: {}", self.favourite_color);
  }
}

// * The name and colors should be printed using a function

fn main() {
  // * Create and store at least 3 people in a vector
  let people = vec![
    Person {
      name: "Jereld".to_owned(),
      age: 23,
      favourite_color: "Blue".to_owned(),
    },
    Person {
      name: String::from("Elon"),
      age: 50,
      favourite_color: String::from("Crimson"),
    },
    Person {
      name: "Bill".to_owned(),
      age: 8,
      favourite_color: "Black".to_owned(),
    }
  ];

  // * Iterate through the vector using a for..in loop
  // * Use an if expression to determine which person's info should be printed
  for person in people {
    if person.age <= 10 {
      person.print();
    }
  }
}
