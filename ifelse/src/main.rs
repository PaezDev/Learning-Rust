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
}
