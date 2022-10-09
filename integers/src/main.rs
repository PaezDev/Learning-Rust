fn main() {
    let mut a:i64 = 2147483647;     // ":i64" overrides normal 32-bit restrictions for int data
                                    // type and gives it 64-bit capacity which allows the following
                                    // statement to print. i = integer, 32 = bit size 
                                    // syntax: :<DATA_TYPE_PREFIX><BIT_SIZE>
    a = a + 1;

    print!("The value of a = {}\n", a);
}
