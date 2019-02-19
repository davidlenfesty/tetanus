use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess: ");

        // Not here, that this has to go inside the loop
        // or else stuff messes up
        // If you initialize outside the loop, then the value is shadowed
        // by a u32, which is not a string and cannot be parsed
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");
        
        println!("Guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("Correct!");
                break;
            }
        }
    }
}
