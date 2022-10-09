fn main() {
    // in this scope, we use std::io
    use std::io;
    // what we did was we added the dependency 'input-stream = "0.3.0"'
    // to the Cargo.toml in order to use 'input_stream'
    use input_stream::InputStream;
    let stdin = io::stdin();
    let mut input = InputStream::new(stdin.lock());

    println!("Enter a value: ");

    let x: i32 = input.scan().expect("An int value");

    println!("Entered value is {}", x);

    if x < 20 {
        println!("Value is less than {}", 20);
    }
    else {
        println!("Value is greater than {}", 20);
    }

    if_in_let()
}

fn if_in_let() -> () {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!();
    println!("The value of another (unrelated) number is: {number}");

    let another_condition = true;
    let num = if another_condition { 5 } else { "six" };    // will result in an error, the 'if' and 'else' arms
                                                            // have value types that are incompatible
}
