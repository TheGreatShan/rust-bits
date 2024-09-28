use std::io;

fn main() {
    println!("Guess a number and let's see if it matches with ours");
    println!("Input:");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("MATEEEE thats not a number");

    println!("Your number was: {guess}");
}
