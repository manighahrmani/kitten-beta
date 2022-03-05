use std::fs;
use std::io::stdin;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Welcome to Kitten! {}", KITTEN);

  println!("How many files do you want to open?");
  let mut input = String::new();
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }

  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Error while turning your input into a number!");

  println!("{} needs to open {} file(s).", KITTEN, number_of_files);

  println!("What is the name of the file?");
  input = String::new();
  match stdin().read_line(&mut input) {
    Ok(_) => (),
    Err(e) => println!("Error while reading your input: `{}`", e),
  }

  println!("{} needs to open this file: {}", KITTEN, input);
  // Ex: Run without, why do we need this? Try putting brackets in above print
  input.pop();

  // See files in cwd, place txt file in correct place
  // println!("Files in cwd are:");
  // for file in fs::read_dir("./").unwrap() {
  //   println!("{}", file.unwrap().path().display());
  // }
  let contents = fs::read_to_string(input).expect("Error while reading the file");
  // Ex: why doesn't this work
  // println!("{} has the following content:\n{}", input, contents);
  println!("The file has the following content:\n{}", contents);
}
