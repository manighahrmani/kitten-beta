use std::io::stdin;

fn main() {
  println!("Welcome to Kitten! 🐱");

  println!("How many files do you want to open?");
  let mut input = String::new();
  match stdin().read_line(&mut input) {
    Ok(n) => {
      println!("{} bytes was read from stdin.", n);
      println!("Your input was: `{}`", input); // Ex: how to remove this trailing newline
    }
    Err(e) => {
      println!("Error while reading your input.");
      println!("Here is the error: `{}`", e); // Ex: how to trigger this error? (Redirecting stdin)
    }
  }
  // stdin()
  //   .read_line(&mut input)
  //   .expect("Error while reading your input!");

  // let number_of_files = match input.trim().parse::<i32>(){
  //   Ok(n) = n,
  //   Err(e) => {
  //     println!("Error while reading your input: `{}`", e);
  //     println!("Will set number_of_files to (-1).");
  //     -1
  //   },
  // };
  let number_of_files: u32 = input
    .trim()
    .parse()
    .expect("Error while turning your input into a number!");

  println!("🐱 needs to open {} file(s).", number_of_files)
}
