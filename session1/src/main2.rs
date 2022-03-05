use std::io::stdin;
// use std::io;

fn main() {
  println!("Welcome to Kitten! 🐱");

  println!("How many files do you want to open?");
  // let mut input: String = String::new();
  let mut input = String::new();
  match stdin().read_line(&mut input) {
    Ok(_) => println!("Your input was: `{}`", input),
    Err(_) => println!("Error while reading your input."),
  }

  let number_of_files: u32 = 1;
  println!("🐱 needs to open {} file(s).", number_of_files)
}
