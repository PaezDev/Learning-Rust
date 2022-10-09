// this program demonstrates
// how to return a value from
// a function in rust
fn main() {
    let a: i32 = sum(4, 5);
    println!("The sum is: {}", a);
    let b = sum1(4, 5);
    println!("The value is: {}", b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn sum1(a: i32, b: i32) -> i32 {
    let x = a + b;

    // can also use return x * 10;
    x * 10
}
