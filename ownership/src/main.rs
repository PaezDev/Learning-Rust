// each value in rust has a variable that's called its owner 
// there can only be one owner at a time 
// when the owner goes out of scope, the value gets dropped

fn example1() 
{
    {                                       // s is not valid here, its not yet declared
        let mut s = String::from("Hello");  // s is valid from this point forward

        // do stuff with s
        s.push_str(", world");  // push_str() appends a literal to a String
        print!("{s}!!");        // this will print 'Hello, world!!'
        
    }                                       // the scope is now over, and s is no longer valid
}

fn example2() 
{
    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{s1}, world!!");    // this will cause an error
    println!("{s2}");                // s1 is 'moved' into s2, s1 is no longer valid while s2 now
                                     // holds s1 value
}

fn example3()
{
    let s1 = String::from("Hello");     
    let s2 = s1.clone();            // clone() is for when we do want to copy the heap data of a
                                    // String, not just the stack data

    println!("s1 = {s1}, s2 = {s2}");       

    // integer types are an exception to this as well as bool, float, char,
    // and a tuple with one of the mentioned types.
    //
    // This is because they have a known size at compile time are stored
    // entirely on the stack, so copies of the actual values are quick to make. 
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

fn main() 
{
    example1();
    example2();
    example3();
}



