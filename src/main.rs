fn main() {
    let tom = Person {
        name: "John".to_string(),
        age: 21,
        position: "middle".to_string(),
        email: "john@m.com".to_string()
    };
    println!("{}", tom.name);
    println!("{}", tom.age);
    println!("{}", tom.position);
    println!("{}", tom.email);
}


struct Person {
    name: String,
    age: u8,
    position: String,
    email: String
}