use std::fs;
use std::io;

fn main() {
  const DEFAULT_FILENAME: &str = "test.txt";
  const KITTEN: &str = "🐱";

  let mut input = String::new();

  println!("How many files do you want {} to open?", KITTEN);
  match io::stdin().read_line(&mut input) {
    Ok(_) => println!("Managed to read the input with success."),
    Err(_) => println!("Error while reading the input."),
  }

  let input: u32 = input.trim().parse().expect("Please type a number!");

  println!("{} needs to open {} files.", KITTEN, input);

  let mut filename = String::new();

  println!("What file should {} open?", KITTEN);
  match io::stdin().read_line(&mut filename) {
    Ok(_) => println!("{} will open {} file.", KITTEN, filename),
    Err(_) => {
      filename = DEFAULT_FILENAME.to_string();
      println!(
        "Error while reading the filename.\n{} will open {} instead.",
        KITTEN, filename
      );
    }
  }

  let file_content = match fs::read_to_string(filename) {
    Ok(content) => content,
    Err(_) => String::from("Failed to read the file's content.\nHere is some dummy text!"),
  };
  println!("Here are the content of the file:\n{}", file_content);
}
