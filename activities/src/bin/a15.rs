// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// my attempt
// struct TicketInfo {
//   holder: String,
//   price: i32,
// }

// enum Ticket {
//   Backstage(TicketInfo),
//   Vip(TicketInfo),
//   Standard(i32),
// }

// fn main() {
//   let tickets = vec![
//     Ticket::Backstage(TicketInfo {
//       holder: String::from("Justin"),
//       price: 50,
//     }),
//     Ticket::Vip(TicketInfo {
//       holder: "Jereld".to_owned(),
//       price: 30,
//     }),
//     Ticket::Standard(10),
//   ];

//   for ticket in tickets {
//     match ticket {
//       Ticket::Standard(price) => println!("standard ticket price: {}", price),
//       Ticket::Backstage(TicketInfo { holder, price }) => {
//         println!("backstage ticket price: {}, holder: {}", price, holder)
//       }
//       Ticket::Vip(TicketInfo { holder, price }) => {
//         println!("vip ticket price: {}, holder: {}", price, holder)
//       }
//     }
//   }
// }

// instructor's answer
// ordered alphabetically
enum Ticket {
  Backstage(f64, String),
  Standard(f64),
  Vip(f64, String),
}

fn main() {
  let tickets = vec![
    Ticket::Backstage(50.0, "Billy".to_owned()),
    Ticket::Standard(15.0),
    Ticket::Vip(30.0, "Amy".to_owned()),
  ];

  for ticket in tickets {
    match ticket {
      Ticket::Backstage(price, holder) => {
        println!("Backstage ticket holder: {}, price: {}", holder, price)
      }
      Ticket::Standard(price) => println!("Standard ticket price: {}", price),
      Ticket::Vip(price, holder) => println!("VIP ticket holder: {}, price: {}", holder, price),
    }
  }
}
