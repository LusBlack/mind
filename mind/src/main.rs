 use std::io;
 use rand::Rng;
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng();
    println!("The secret number is: {secret_number}")

    println!("Please input your guess.");

    let mut guess = String ::new();
    //"String :: new" serves as a container
    //"String ::new() is a function that returns a new instance of a string"
    //"::" indicates that new is an associated function of the String type

    io::stdin()
    //listen for user input
    //the stdin function allows us to handle user input.
    //we get it from the io module
        .read_line(&mut guess)
        //this calls the read_line method on the standard input handle 
        //to get input from the user.
        //the full job of read_line is to take whatever the user types into stdin
        //(without overwriting its contents), so we pass the string as an argument.
        //the string needs to be mutable so the method can change the string's content
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
