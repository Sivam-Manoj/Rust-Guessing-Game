use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess the number");
    let secret_number: i32= rand::thread_rng().gen_range(1..=100);
    loop{
        println!("type a number");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
        let guess: i32 = guess.trim().parse().expect("please type a number");
        println!("you guessed: {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => println!("Bros Win"),
        }
        if guess == secret_number{
            break;
        }
    }    
}
