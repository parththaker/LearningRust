extern crate rand;

use std::io;
use std::process;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("What did you give man?");

        println!("You guess this, {}", guess);
        println!("But the secret number was : {}", secret_number);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Number dammit");
                process::exit(1);
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Less  =>  println!("Too Big"),
            Ordering::Greater   =>  println!("Too Small"),
            Ordering::Equal =>  {
                println!("You won... Now screw you.. Exiting the loop :P");
                break;
            },
        };
    }
}
