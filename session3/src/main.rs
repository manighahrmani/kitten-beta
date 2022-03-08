use std::{error::Error, fs, io::stdin};

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");
  let mut input = String::new();
  read_user_input(&mut input);

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Error while turning your input into a number!");

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  println!("What is the name of the file?");
  read_user_input(&mut input);
  let input = input.trim();

  println!("{} needs to open this file: {}", KITTEN, input);

  let contents = fs::read_to_string(input).expect("Error while reading the file");
  println!("The file has the following content:\n{}", contents);

  let mut newInput = String::new();
  read_user_input_1(&mut newInput);
}

fn read_user_input(input: &mut String) {
  input.clear();
  match stdin().read_line(input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
    // input will be empty, error must be handled
  }
}

fn read_user_input_1(input: &mut String) -> Result<(), Box<dyn Error>> {
  input.clear();
  stdin().read_line(input);

  Ok(())
}
