fn main() {
    
    use std::io;
    use input_stream::InputStream;
    
    let stdin = io::stdin();
    let mut input = InputStream::new(stdin.lock());

    loop {
        println!("Hello");
        break;
    }

    let mut i: i32 = input.scan().expect("int");

    while i != 0 {
        println!("{}", i);
        i = i - 1; 
    }
}
