use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("welcome to the guessing game");

    let secret_number  : u32 = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("please input your guess");

        let mut guess: String = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read number");

        println!("you guessed: {}", guess);

        match guess.trim().parse::<u32>().expect("not a number").cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }

}
