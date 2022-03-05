// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Locker {
  student_name: String,
  assignment: Option<i32>,
}

fn main() {
  let my_locker = Locker {
    student_name: "Jereld".to_owned(),
    assignment: Some(32),
  };

  match my_locker {
    Locker {
      student_name,
      assignment: Some(number),
    } => println!("Locker {} assigned to {}", number, student_name),
    Locker { student_name, .. } => println!("Locker assigned to {}", student_name),
  }
}
