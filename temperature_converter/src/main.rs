use std::io;

#[quit::main]
fn main() {
    loop {
        let mut input = String::new();

        menu();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        let input: i32 = input
            .trim()
            .parse()
            .expect("Unable to convert to integer");

        if input == 1 {
            println!("{} Celsius\n\n", f_to_c());
        } else if input == 2 {
            println!("{} Fahrenheit\n\n", c_to_f());
        } else if input == 3 {
            println!("\t\t\t      Goodbye!!\n");
            quit::with_code(0);
        } else {
            println!("\n\t\t\t {input} is invalid input.\n");
        }
    }
}

fn f_to_c() -> f32 {
    let mut input = String::new();

    println!("Enter your value...");

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line");

    let input: f32 = input
        .trim()
        .parse()
        .expect("Unable to convert to integer");

    let fahrenheit = input;

    print!("\t\t\t{fahrenheit} Fahrenheit = ");

    (fahrenheit - 32.0) * (0.55555556)
}

fn c_to_f() -> f32 {
    let mut input = String::new();

    println!("Enter your value...");

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line");

    let input: f32 = input
        .trim()
        .parse()
        .expect("Unable to convert to integer");
   
    let celsius = input;

    print!("\t\t\t{celsius} Celsius = ");
    
    (celsius * 1.8) + 32.0
}

fn menu() {
    println!("\t\t\tTEMPERATURE CONVERTER\n");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    println!("3. Quit");
    println!("\nPlease make a selection...");
}


