fn main() {
    let mut a = 1;
    println!("The value of a is {}",a);
    a = 2;
    println!("The value of a is {}\n",a);

    shadowing();

    spaces();

    err_spaces();   // this call will cause an error
}
/*

Shadowing is different from marking a variable as mut,
because we’ll get a compile-time error if we accidentally
try to reassign to this variable without using the let keyword.
By using let, we can perform a few transformations on a value
but have the variable be immutable after those transformations have been completed.

*/
fn shadowing() -> () {
    let x = 5;
    let x = x + 1;  // borrows the value of 5, adds 1, then assigns to new x 

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is {x}");
}

fn spaces() -> () {
    let spaces = "     ";
    let spaces = spaces.len();


    println!("{spaces}");
}

// this function will cause an error
fn err_spaces() -> () {
    todo!();
    let mut spaces = "     ";
    spaces = spaces.len();

}
