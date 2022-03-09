use std::{
  error::Error,
  fs,
  io::{self, stdin},
};

fn main() {
  let mut new_input = String::new();
  match read_user_input_0(&mut new_input) {
    Ok(_) => println!("Read with success.\nNewInput is {}", new_input),
    Err(e) => {
      println!("Encountered the follwoing Error:\n{}", e);
      new_input = String::from("xxx");
      println!("Set new_input to the default value of {}", new_input);
    }
  }
  // TODO: Reading from https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about

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

  let mut counter = 0;

  while counter != number_of_files {
    let mut filename = String::new();

    println!("What is the name of the file?");
    read_user_input(&mut filename);
    filename.pop();

    println!("{} needs to open this file: {}", KITTEN, filename);

    let contents = fs::read_to_string(filename).expect("Error while reading the file");
    println!("The file has the following content:\n{}", contents);

    counter += 1;
  }
}

fn read_user_input(input: &mut String) {
  input.clear();
  match stdin().read_line(input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
    // input will be empty, error must be handled
  }
}

fn read_user_input_0(input: &mut String) -> Result<(), io::Error> {
  input.clear();
  match stdin().read_line(input) {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}

fn _read_user_input_1(input: &mut String) -> Result<(), Box<dyn Error>> {
  input.clear();
  stdin().read_line(input)?;

  Ok(())
}
