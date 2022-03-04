use std::io::stdin;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");

  // match stdin().read_line(&mut input) {
  //   Ok(_) => (),
  //   Err(e) => println!("Error while reading your input: `{}`", e),
  // }

  let mut input = String::new();

  input = read_user_input_4(input);

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Your input could not be turned into a number!");

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  println!("What is the name of the file?");

  input = read_user_input_4(input);

  // match stdin().read_line(&mut input) {
  //   Ok(_) => (),
  //   Err(e) => println!("Error while reading your input: `{}`", e),
  // }

  // read_user_input(input);

  println!("{} needs to open this file: {}", KITTEN, input);
}

fn read_user_input_4(mut input: String) -> String {
  input.clear(); // Ex: Why do we need this?
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }
  // Ex: why &input would not work
  input
}

fn _read_user_input_3() -> String {
  let mut input = String::new();
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }
  input
}

// fn _read_user_input_2<'lifetime>() -> &'lifetime str {
//   let mut input = String::new();
//   match stdin().read_line(&mut input) {
//     Ok(_) => (),
//     Err(e) => println!("Error while reading your input: `{}`", e),
//   }
//   &input
// }

fn _read_user_input_1(mut input: String) {
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }
}
