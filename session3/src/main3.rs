// use std::fs;
// use std::io::stdin;
use std::{fs, io::stdin};

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");
  let mut input = String::new();
  read_user_input_4(&mut input);

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Error while turning your input into a number!");

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  println!("What is the name of the file?");
  read_user_input_4(&mut input);
  input.pop();

  println!("{} needs to open this file: {}", KITTEN, input);

  let contents = fs::read_to_string(input).expect("Error while reading the file");
  println!("The file has the following content:\n{}", contents);
}

// borrow a value with a mutable reference and modify it
fn read_user_input_4(input: &mut String) {
  input.clear();
  match stdin().read_line(input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }
}

// takes and gives back
fn _read_user_input_3(mut input: String) -> String {
  input.clear(); // Ex: Why do we need this?
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
    // input will be empty, error must be handled
  }
  // Ex: why &input would not work
  input
}

// attempt to let main borrow (dangling reference)
// fn _read_user_input_2() -> &'static str {
//   let mut input = String::new();
//   match stdin().read_line(&mut input) {
//     Ok(_) => (),
//     Err(e) => println!("Error while reading your input: `{}`", e),
//   }
//   &input
// }

// gives ownership
fn _read_user_input_1() -> String {
  let mut input = String::new();
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }
  input
}

// takes ownership
fn _read_user_input_0(mut input: String) {
  // needs to be changed to mut here
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }
}
