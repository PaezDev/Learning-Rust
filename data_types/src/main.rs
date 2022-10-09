/*
    Integers:
                signed      unsigned
    8-bit  =    i8          u8
    16-bit =    i16         u16
    32-bit =    i32         u32
    64-bit =    i64         u64
    128-bit=    i128        u128
    arch   =    isize       usize
*/
fn main() {
    integer();
    floating_point();
    operations();
    
    let num = 22; 
    let result = less_than_30(num);
    if result {
        println!("{num} is less than 30!");
    }
    else {
        println!("{num} is not less than 30..");
    }

}

fn integer() -> () {
    let mut a:i64 = 2147483647;     // ":i64" overrides normal 32-bit restrictions for int data
                                    // type and gives it 64-bit capacity which allows the following
                                    // statement to print. i = integer, 32 = bit size 
                                    // syntax: ':<DATA_TYPE_PREFIX><BIT_SIZE>'
    a = a + 1;

    print!("The value of a = {}\n", a);   
}

fn floating_point() -> () {
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x = {x}, y = {y}");
}

fn operations() -> () {
    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 ={product}");

    // division
    let quotient = 56.7 / 32.2;
    let floored  = 2 / 3;    // results in 0
    println!("56.7 / 32.2 = {quotient}, 2 / 3 = {floored}");

    // remainder 
    let remainder = 43 % 5;
    println!("The remainder of 43 / 5 = {remainder}");
}

fn less_than_30(num: i32) -> bool {
    if num < 30{
        true
    }    
    else {
        false
    }
}
