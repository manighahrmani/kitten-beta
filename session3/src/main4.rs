use std::{fs, io::stdin, process};

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");
  let number_of_files: u32 =
    read_user_number_2(1).unwrap_or_else(|e| eprintln!("Error while reading your input: {}", e));

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  let mut output = String::new();
  let mut counter = 0;

  while counter != number_of_files {
    let ordinal = match counter {
      0 => String::from("1st"),
      1 => String::from("2nd"),
      2 => String::from("3rd"),
      _ => format!("{}th", counter + 1), // What about a variable here?
    };

    println!("What is the name of the {} file?", ordinal);
    let mut filename = String::new();
    read_user_word(&mut filename).unwrap_or_else(|e| {
      eprintln!("Error while reading your input: {}", e);
      process::exit(1)
    });

    let contents = fs::read_to_string(filename.clone()).unwrap_or_else(|e| {
      println!("Error while reading {}: {}", filename, e);
      println!("Its content will be set to some dummy text.");
      String::from("This is some dummy text. 🤕\n")
    });
    output.push_str(&contents);

    counter += 1;
  }

  println!("The output of {} is:\n{}", KITTEN, output);
}

// Reads a single number from standard input, returns default on error
fn read_user_number_2(default: u32) -> Result<u32, std::io::Error> {
  let mut input = String::new();
  read_user_word(&mut input)?;
  let mut number: u32 = input.parse().unwrap_or_else(|e| {
    println!("Error while turning your input into a number: {}", e);
    println!("Will default to {} on this occasion.", default);
    default
  });
  Ok(number)
}

fn read_use_number_1(default: u32) -> Result<u32, std::io::Error> {
  let mut input = String::new();
  match read_user_word(&mut input) {
    Ok(()) => {
      let mut number: u32 = input.parse().unwrap_or_else(|e| {
        println!("Error while turning your input into a number: {}", e);
        println!("Will default to {} on this occasion.", default);
        default
      });
      Ok(number)
    }
    Err(e) => Err(e),
  }
}

fn _read_use_number_0(default: u32) -> u32 {
  let mut input = String::new();
  let number = match read_user_word(&mut input) {
    Ok(()) => input.parse().unwrap_or_else(|e| {
      println!("Error while turning your input into a number: {}", e);
      println!("Will default to {} on this occasion.", default);
      default
    }),
    Err(e) => {
      println!("Error while reading your input: {}", e);
      println!("Will default to {} on this occasion.", default);
      default
    }
  };
  number
}

// Reads a single word from standard input
fn read_user_word(input: &mut String) -> Result<(), std::io::Error> {
  read_user_input(input)?; // returns the error from read_user_input
  *input = String::from(input.split(" ").next().unwrap_or_else(|| input)); // or .unwrap_or(input)
  Ok(())
}

/// Reads a word from standard input
fn read_user_input(input: &mut String) -> Result<(), std::io::Error> {
  input.clear();
  stdin().read_line(input)?;
  input.pop();
  Ok(())
}
