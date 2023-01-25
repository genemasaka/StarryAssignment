//import input-output from standard lib
use std::io;

//entry point of the program
fn main() {
    //prompt to enter input
    println!("Type something and I'll return it back to you");

    //create a new instance of an empty string
    let mut echo = String::new();

    //reads the input from user and assigns it to echo instance
    io::stdin().read_line(&mut echo).expect("Try again...");
    println!("You typed in the following:\n {}", echo);
}
