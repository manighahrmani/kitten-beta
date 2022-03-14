use std::{fs, io::stdin, process};

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");
  let mut input = String::new();
  // read_user_word_2(&mut input).expect("Error while reading your input!");
  read_user_word_2(&mut input).unwrap_or_else(|e| {
    eprintln!("Error while reading your input: {}", e);
    process::exit(1)
  });

  let number_of_files: u32 = input.parse().unwrap_or_else(|e| {
    println!("Error while turning your input into a number: {}", e);
    println!("Will default to 1 on this occasion.");
    1
  });

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
    read_user_word_2(&mut filename).unwrap_or_else(|e| {
      eprintln!("Error while reading your input: {}", e);
      process::exit(1)
    });

    // let contents = fs::read_to_string(filename.clone()).unwrap_or_else(|e| {
    //   println!("Error while reading {}: {}", filename, e);
    //   process::exit(1)
    // });
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

fn read_user_word_2(input: &mut String) -> Result<(), std::io::Error> {
  read_user_input(input)?; // returns the error from read_user_input
  *input = String::from(input.split(" ").next().unwrap_or_else(|| input)); // or .unwrap_or(input)
  Ok(())
}

fn _read_user_word_1(input: &mut String) -> Result<(), std::io::Error> {
  read_user_input(input).unwrap(); // panics, we want to return error instead
  let words: Vec<&str> = input.split(" ").collect();
  *input = words[0].to_string();
  Ok(())
}

// Maybe mention: Performance diff https://doc.rust-lang.org/book/ch13-04-performance.html
fn _read_user_word_0(input: &mut String) -> Result<(), std::io::Error> {
  match read_user_input(input) {
    Ok(()) => {
      // let words: Vec<&Strin> = input.split(" ").collect();
      let mut words = input.split(" ");
      // for word in &words {
      //   println!("- {}", word);
      // }
      match words.next() {
        Some(s) => *input = String::from(s),
        None => (),
      };
      Ok(())
    }
    Err(e) => Err(e),
  }
}

/// Reads a word from standard input, at the moment it can read multiple words!
fn read_user_input(input: &mut String) -> Result<(), std::io::Error> {
  input.clear();
  stdin().read_line(input)?;
  input.pop();
  Ok(())
}
