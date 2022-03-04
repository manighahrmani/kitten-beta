use std::io::stdin;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");

  let mut input = String::new();

  input = read_user_input(input);

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Your input could not be turned into a number!");

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  println!("What is the name of the file?");

  input = read_user_input(input);

  println!("{} needs to open this file: {}", KITTEN, input);
}

fn read_user_input(mut input: String) -> String {
  input.clear(); // Ex: Why do we need this?
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }
  // Ex: why &input would not work
  input
}
