use std::io::stdin;

fn main() {
  println!("Welcome to Kitten! 🐱");

  println!("How many files do you want to open?");

  let mut input = String::new();

  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Your input could not be turned into a number!");

  println!("🐱 needs to open {} file(s).", number_of_files)
}
