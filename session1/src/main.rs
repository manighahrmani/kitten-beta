use std::io;

fn main() {
  let mut input = String::new();

  println!("How many files do you want 🐱 to open?");
  
  match io::stdin().read_line(&mut input) {
    // Ok(_) => println!("🐱 needs to open {} files.", input),
    Ok(_) => println!("Managed to read the input with success."),
    Err(_) => println!("Error while reading the input."),
  }

  // Next we trim and parse the input into an integer
  let input : u32 = match input.trim().parse(){
    Ok(_) => println!("🐱 needs to open {} files.", input),
    Err(_) => println!("Input could not be converted into a number"),
  };
}
