use std::io;

fn main() {
  println!("Please input the name of the file to open:");

  let mut filename = String::new();

  io::stdin()
    .read_line(&mut filename)
    .expect("Failed to read the filename");

  println!("You asked for: {}", filename);

  println!("Thanks for using minicat! 🐱")
}
