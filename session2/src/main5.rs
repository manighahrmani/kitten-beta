use std::fs;
// use std::fs::read_to_string;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
  const DEFAULT_FILENAME: &str = "test.txt";
  const KITTEN: &str = "🐱";

  let mut input = String::new();

  println!("How many files do you want to open?");
  match io::stdin().read_line(&mut input) {
    Ok(_) => println!("Managed to read your input with success."),
    // _ => (), // instead of confirming
    Err(_) => println!("Error while reading your input."),
  }

  let input: u32 = input
    .trim()
    .parse()
    .expect("Your input could not be turned into a number!");

  println!("{} needs to open {} files.", KITTEN, input);

  let mut filename = String::new();

  println!("What is the path to the file?");
  match io::stdin().read_line(&mut filename) {
    Ok(_) => println!("{} will open: {}", KITTEN, filename),
    Err(_) => {
      filename = DEFAULT_FILENAME.to_string();
      println!(
        "Error while reading the filename.\n{} will open {} instead.",
        KITTEN, filename
      );
    }
  }

  let file_content: String = fs::read_to_string(filename)?;
  // let file_content = match fs::read_to_string(filename) {
  //   Ok(content) => content,
  //   Err(_) => {
  //     String::from("This is some dummy text …\nFile was not found or could not be opened! 🥸")
  //   }
  // };
  println!("Here are the content of the file:\n{}", file_content);

  println!("Thanks for using {}.", KITTEN);

  Ok(())
}
