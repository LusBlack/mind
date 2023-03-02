extern crate rand;
use std::io;
use std ::cmp::Ordering;
use rand::Rng;

 
 
 
 
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 10);

      loop{
    println!("Please input your guess. ");

    let mut guess = String::new();
    //"String :: new" serves as a container
    

   io::stdin()
    //listen for user input
    //the stdin function allows us to handle user input.
    //we get it from the io module
       .read_line(&mut guess)
        //this calls the read_line method on the standard input handle 
        //to get input from the user.
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

 println!("You guessed: {guess}");
    match guess.cmp(& secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
      }
    }
}
}
