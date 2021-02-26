// to obtain and process user input, we need to bring the input/output library into scope
// we also use Ordering to compare our values for the game
// next we have to import the rand library for random number generation
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // use the thread_rng generator and generate a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Uncomment this line if you want to know the secret number ahead of time like some cheater
    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        // creating a place to store user input
        let mut guess = String::new();

        io::stdin()
            // .read_line takes what the user types and places it into a string, so it takes that string as an argument
            .read_line(&mut guess)
            .expect("Failed to read line");

        // we create guess again to turn it into an integer of type u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the curly brackets are placeholders
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
