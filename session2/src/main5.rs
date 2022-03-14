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

  let mut output = String::new();
  let mut counter = 0;

  while counter != number_of_files {
    println!("What is the name of the file?");
    let mut filename = String::new();
    read_user_input(&mut filename);
    filename.pop();

    // println!("{} needs to open this file: {}", KITTEN, filename);

    let contents = fs::read_to_string(filename).expect("Error while reading the file");
    // output = output + &contents; // Why does it have to be a reference to contents? See sign of add
    output.push_str(&contents); // Can contents be used after this? Try println

    counter += 1;
  }

  println!("The output of {} is:\n{}", KITTEN, output);
}

fn read_user_input(input: &mut String) {
  input.clear();
  match stdin().read_line(input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }
}
