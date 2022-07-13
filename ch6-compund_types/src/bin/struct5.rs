// Fill the blank
struct Person {
    name: String,
    age: u8,
}
fn main() {
    build_person("name".to_string(), 18);
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name
    }
}
