use std::{fs, io::stdin};

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");
  let mut input = String::new();
  match read_user_input_2(&mut input) {
    Err(e) => panic!("Error while reading your input: {}", e),
    _ => (),
  }

  let number_of_files: u32 = input
    // .trim() // Changed read_user_input function to cover this
    .parse()
    .expect("Error while turning your input into a number!");

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  let mut counter = 0;
  while counter != number_of_files {
    println!("What is the name of the file?");
    let mut filename = String::new();
    match read_user_input_2(&mut input) {
      Err(e) => panic!("Error while reading your input: {}", e),
      _ => (),
    }
    // filename.pop(); // Changed read_user_input function to cover this

    println!("{} needs to open this file: {}", KITTEN, filename);

    let contents = fs::read_to_string(filename).expect("Error while reading the file");
    println!("The file has the following content:\n{}", contents);

    counter += 1;
  }
}

fn read_user_input_2(input: &mut String) -> Result<(), std::io::Error> {
  input.clear();
  stdin().read_line(input)?;
  input.pop();
  Ok(())
}

fn _read_user_input_1(input: &mut String) -> Result<(), std::io::Error> {
  input.clear();
  match stdin().read_line(input) {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}

fn _read_user_input_0(input: &mut String) {
  input.clear();
  match stdin().read_line(input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
    // input will be empty, error must be handled
  }
}
