use std::{
    io::{
        self,
        Write,
    },
    process,
};

fn main() {
    let mut input = String::new(); 

    println!("Hello, User!");
    
    println!("Please Enter the magic number: ");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let num = input.trim().parse::<i64>().unwrap_or_else(|_| {
        eprintln!("- Entered input is not valid!");
        drop(input);
        process::exit(1);
    });
    

    my_function();
    let user_sum:i64 = my_function2(num);

    println!("u look like ur {}!!", user_sum);
}

fn my_function() {
    println!("I am a new function!");
}

fn my_function2(a: i64) -> i64 {
    let mut sum: i64 = 0;

    for i in 0..=a {
        sum += a * i;     
    }

    return sum;
}
