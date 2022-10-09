// each value in rust has a variable that's called its owner 
// there can only be one owner at a time 
// when the owner goes out of scope, the value gets dropped
fn main() {
    // value 10 is owned by a
    let a = 10;
    println!("{}", a);

    let s = String::from("Andry"); 
    
    // call the function that will borrow
    // the value of s
    borrowing(&s);

    // after affixing '&' to s and String,
    // we are able to execute the following
    // print statement
    println!("{}", s);
}

/*
fn error_function() -> () {
    // will result in error if called because a is out of scope 
    print!("{}", a);
}
*/

// borrows the value of s
fn borrowing(x: &String) {
    println!("{}", x);
}
