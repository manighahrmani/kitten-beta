use std::{fs, io::stdin};

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
  let mut file_name = String::new();
  read_user_input(&mut file_name);
  file_name.pop();

  println!("{} needs to open this file: {}", KITTEN, file_name);

  let contents = fs::read_to_string(file_name).expect("Error while reading the file");
  println!("The file has the following content:\n{}", contents);
}

fn read_user_input(input: &mut String) {
  input.clear();
  match stdin().read_line(input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
    // input will be empty, error must be handled
  }
}
