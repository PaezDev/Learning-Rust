fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    println!("{s1}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
fn change(s: &mut String) {
    s.push_str(", world!"); // because s is mutable, we are able to append to
                            // this string and s1 still retains ownership because we
                            // passed it as reference
}
