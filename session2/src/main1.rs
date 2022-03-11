use std::fs;
use std::io::stdin;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");
  let mut input = String::new();
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Error while turning your input into a number!");

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  println!("What is the name of the file?");
  let mut file_name = String::new();
  match stdin().read_line(&mut file_name) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }

  println!("{} needs to open this file: {}", KITTEN, file_name);
}
