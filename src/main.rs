use std::io;
use rand::Rng;

fn main() {
//    let x = 6;
//    println!("{}", x);

//    if x == 6 {
//        println!("ok");
//    }

//    let a = 10;
//    println!("{}", a);

    println!("Угадай число!");


    let secret_number = rand::thread_rng().gen_range(1..1001);
    println!("You secret number {}", secret_number);

    println!("Please input your guess:"); 

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed");
    println!("You guessed {}", guess);
}
