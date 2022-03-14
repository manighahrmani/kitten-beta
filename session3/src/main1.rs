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
    let ordinal = match counter {
      0 => String::from("1st"),
      1 => String::from("2nd"),
      2 => String::from("3rd"),
      _ => format!("{}th", counter + 1), // What about a variable here?
    };

    println!("What is the name of the {} file?", ordinal);
    let mut filename = String::new();
    read_user_input(&mut filename);
    filename.pop();

    let contents = fs::read_to_string(filename).expect("Error while reading the file");
    output.push_str(&contents);

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
