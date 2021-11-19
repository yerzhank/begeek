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

    let _i: i32 = 0;

    for _i in 1..10 {
        println!("{}", _i);
        if _i == 5 {
            println!("stop");
            break
        }
    }
    //io::stdin().read_line(&mut guess).expect("Failed");


    let data = "some data".to_string();
    let bigdata = data.clone();
    println!("{}", bigdata);

    let mut s = String::from("hello");
    println!("{}", s);
    s.push_str(", world");

    println!("{}",s);

    let s2 = get_string();
    println!("{}", s2);
    let param_str = String::from("Yerzhan");
    let (a1, a2) = get_cort(param_str);

    println!("a1 = {}, a2 = {}", a1, a2);

}


fn get_string() -> String {
    let res = String::from("get str");
    res
}

fn get_cort(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)


}