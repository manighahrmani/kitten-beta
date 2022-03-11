use std::{fs, io::stdin, process};

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");
  let mut input = String::new();
  // read_user_input(&mut input).expect("Error while reading your input!");
  read_user_input(&mut input).unwrap_or_else(|e| {
    eprintln!("Error while reading your input: {}", e);
    process::exit(1)
  });

  let number_of_files: u32 = input.parse().unwrap_or_else(|e| {
    println!("Error while turning your input into a number: {}", e);
    println!("Will default to 1 on this occasion.");
    1
  });
  // .trim() // Changed read_user_input function to cover this

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  let mut counter = 0;

  while counter != number_of_files {
    println!("What is the name of the file?");
    let mut filename = String::new();
    read_user_input(&mut filename).unwrap_or_else(|e| {
      eprintln!("Error while reading your input: {}", e);
      process::exit(1)
    });

    println!("{} needs to open this file: {}", KITTEN, filename);

    let contents = fs::read_to_string(filename).unwrap_or_else(|e| {
      eprintln!("Error while reading the file: {}", e);
      process::exit(1)
    });
    println!("The file has the following content:\n{}", contents);

    counter += 1;
  }
}

/// Reads a word from standard input, at the moment it can read multiple words!
/// TODO: https://users.rust-lang.org/t/how-to-split-a-string-by-and-then-print-first-or-last-component/23042
fn read_user_input(input: &mut String) -> Result<(), std::io::Error> {
  input.clear();
  stdin().read_line(input)?;
  input.pop();
  Ok(())
}
