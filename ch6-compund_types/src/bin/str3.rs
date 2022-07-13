
// Fill the blank
fn main() {
    let mut s = "".to_string();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
