// Fix errors and panics to make it work
fn main() {
   let v1 = 251_u8 + 3;
   let v2 = u8::checked_add(251, 3).unwrap();
   println!("{},{}",v1,v2);
}