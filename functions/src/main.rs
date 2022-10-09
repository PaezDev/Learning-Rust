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

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let num = input
        .trim()
        .parse::<i64>()
        .unwrap_or_else(|_| {
            eprintln!("- Entered input is not valid!");
            drop(input);
            process::exit(1);
        });
    

    my_function();
    let user_sum:i64 = my_function2(num);

    println!("u look like ur {}!!", user_sum);

    print_labeled_measurements(5, 'h');

    statements();

    println!("{}", expressions(30));

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

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements() {
    // statement, ends with semicolon
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");
}
fn expressions(x: i32) -> i32 {
    // example of expression
    // types of expressions in rust: calling a function, calling a macro, a new scope block with
    // curly brackets (like the one below)
    //
    // Expressions do not include semicolons. If you add a semicolon to the end of an expression,
    // you turn it into a statement, and it will then not return a value.
    (x * 3) + 1
}
