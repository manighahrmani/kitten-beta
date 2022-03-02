use std::io::stdin;

fn main() {
  println!("Welcome to Kitten! 🐱");

  println!("How many files do you want to open?");

  let mut input = String::new();

  match stdin().read_line(&mut input) {
    Ok(n) => {
      println!("{} bytes was read from stdin.", n);
      println!("Your input was: `{}`", input); // Ex: how to remove this trailing newline
    }
    Err(e) => {
      println!("Error while reading your input.");
      println!("Here is the error: `{}`", e); // Ex: how to trigger this error
    }
  }

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Your input could not be turned into a number!");

  println!("🐱 needs to open {} file(s).", number_of_files)
}
