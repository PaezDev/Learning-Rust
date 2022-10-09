use std::io;

fn main() {

    loop {
        let mut input = String::new();

        menu();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        if input.trim() == "q" {
            break;
        }

        let n: u128 = match input
            .trim()
            .parse() 
            {
                Ok(num) => num,
                Err(_)  => continue,
            };

        println!("\nNumber {} in the fibonacci sequence is {}\n", n, fibonacci(n));

    }
}

fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn menu() {
    println!("\t\t\tFIBONACCI SEQUENCE\n");
    print!("Enter a number...");
    println!("\t\t\t\t enter q to quit");
}
