use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("There's no way you can guess what number I'm thinking of.");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not even a number!");
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("There's no way I'd choose {}. It isn't big enough.", guess),
            Ordering::Greater => println!("Did you really think it was {}...", guess),
            Ordering::Equal => {
                println!("It wasn't {}! It was {}.", guess, secret_number + 1);
                println!("I don't want to play anymore. This is dumb.");
                break;
            },
        }
    }
}
