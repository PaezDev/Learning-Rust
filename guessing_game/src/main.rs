use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("\t\t\tGUESSING GAME\n"); 

    loop {
        println!("Guess the number...");
    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small.\n"),
            Ordering::Greater => println!("\nToo Big!!\n"),
            Ordering::Equal => {
                print!("\nYAY!! You Win!!!");
                break;
            }
        }
    }
} 
