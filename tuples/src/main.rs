fn main() {
    let tup:(i32, i32, i32) = (1, 2, 3);

    let (x, y, z) = tup;

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("A = {}", a);
    println!("B = {}", b);
    println!("C = {}", c);

    println!("X = {}, Y = {}, and Z = {}", x, y, z);
}
