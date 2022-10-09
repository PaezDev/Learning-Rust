// A little messy but reference for basic syntax and usage of 
// different loop types in Rust
fn main() {
    let result = random_loop();

    println!("The result is {result}");

    println!();
    nested_loop();

    println!();
    while_loop();

    println!();
    while_loop_array();

    println!();
    for_loop();
}

// will return value from loop

fn random_loop() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    result
}

fn nested_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;  // will exit inner loop
            }
            if count == 2 {
                break 'counting_up;     // will exit outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 { 
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_loop_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is {} at index {}", a[index], index);

        index += 1;
    }
}

fn for_loop() {
    let a = [0, 1 ,2 ,3 ,4, 5];

    for i in a.iter() {
        println!("Value = {}", i);
    }

    println!();

    for number in (1..10).rev() {   // rev() reverses the range
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
} 
