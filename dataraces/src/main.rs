/*
 * A data race is similar to a race condition
 * and happens when these three behaviors occur:
 *
 *  - Two or more pointers access the data at the same time
 *  - At least one of the pointers is being used to write to the data
 *  - There's no mechanism being used to synchronize access to data
 *
*/

fn main() {
   let _m = main1();

   println!("{}", _m);
}

fn main1() -> String  {
   let s = String::from("Hello der"); 

   s
}
