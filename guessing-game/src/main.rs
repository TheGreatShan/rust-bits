use std::io;
use rand::Rng;
use std::cmp::Ordering;

// cargo doc --open will generate a documentation of the functions and the dependencies used
fn main() {
    println!("Guess a number and let's see if it matches with ours");
    
    // the number 1 and 100 are both also included
    loop{
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Input a number:");
        
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("MATEEEE thats not a number");
    
        //let guess :u32 = guess.trim().parse().expect("Parsing to integer failed");
        // switchting to a match makes error handling better
        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            // Application goes to the next iteration, when an error occurs
            Err(_) => continue
         };
    
        println!("Your number was: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You lost, your number was too small"),
            Ordering::Greater => println!("You lost, your number was too big"),
            Ordering::Equal => {
                println!("YOU WIN!!!!");
                break;
            }
        }
        println!("The correct number was: {secret_number}")
    }

}
