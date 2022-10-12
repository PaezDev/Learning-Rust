/*
 *  A data race is similar to a race condition
 *  and happens when these three behaviors occur:
 *
 *  - Two or more pointers access the data at the same time
 *  - At least one of the pointers is being used to write to the data
 *  - There's no mechanism being used to synchronize access to data
*
*   Note: this program is not meant to compile
*/

fn main() {
   let _m = main1();

   println!("{}", _m);
}

fn main1() -> String  {
   let s = String::from("Hello der"); 

   s
}

fn error_function()
{
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

fn ok_function()
{
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn big_problem()
{
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

fn no_more_problem()
{
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
