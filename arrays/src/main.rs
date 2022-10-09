fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    for i in 0..numbers.len() {
        println!("{} element = {}", i, numbers[i]);
    }

    access_array_element();
}

fn access_array_element() -> () {
    use std::io;

    let a = [1, 2, 3 , 4, 5];

    let mut index = String::new();

    println!("Please enter the value of the index you would like to access: ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
