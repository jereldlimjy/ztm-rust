// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
  id: i32,
  quantity: i32,
}

fn display_qty(item: &GroceryItem) {
  println!("quantity: {}", item.quantity);
}

fn display_id(item: &GroceryItem) {
  println!("id: {}", item.id);
}

fn main() {
  let sushi = GroceryItem {
    id: 4,
    quantity: 25,
  };

  display_qty(&sushi);
  display_id(&sushi);
}
