use std::io;

fn main() {
  let mut input = String::new();

  println!("How many files do you want 🐱 to open?");
  match io::stdin().read_line(&mut input) {
    // Ok(n) => {
    //     println!("{} bytes read", n);
    //     println!("{}", input);
    // }
    // Err(error) => println!("error: {}", error),
    Ok(_) => println!("Managed to read the input with success."),
    Err(_) => println!("Error while reading the input."),
  }

  let input: u32 = input.trim().parse().expect("Please type a number!");
  println!("🐱 needs to open {} files.", input);
}
