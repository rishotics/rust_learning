use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Hello, world! please enter your number");

    let secret_number = rand::thread_rng().gen_range(1..=101);
    
    loop{
        let mut a = String::new();

        io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
        println!("You typed: {a}", );

        let a: u32= match a.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
    
        match a.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    
        println!("The secret number is: {secret_number}");
    }
}
