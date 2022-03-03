use std::io::stdin;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");

  let mut input = read_user_input();

  // match stdin().read_line(&mut input) {
  //   Ok(_) => (),
  //   Err(e) => println!("Error while reading your input: `{}`", e),
  // }

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Your input could not be turned into a number!");

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  input = read_user_input();

  println!("Second input is {}", input);
}

fn read_user_input() -> String {
  let mut input = String::new();
  match stdin().read_line(&mut input) {
    Ok(_) => input,
    Err(e) => {
      println!("Error while reading your input: `{}`", e);
      String::from("🤬")
    }
  }
}
