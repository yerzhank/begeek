use std::io;
use rand::Rng;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Ваше случайно число: {}", secret_number); 

    let mut guess = String::new();
    println!("Введите число"); 
    io::stdin().
        read_line(&mut guess).
        expect("Failed");
    println!("Введенное Вами число: {}", guess);

}
