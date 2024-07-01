use std::{cmp::Ordering, num::IntErrorKind};

use rand::Rng;

fn main() {

    //猜数字游戏。包含 io 输入，loop循环，match控制流解构
    // guess_number();

    //元组结构
    let tup: (i32, f64, i8) =  (5112, 3.14, 100);

    println!("first value of tup is {}", tup.0);

    let (x, y, z) = tup;

    println!("x is {x}");
    
    println!("Hello, world!");


    //for 循环
    for i in 0..4  {
        println!("{i}");
    }

    //倒序循环
    for i in (0..4).rev() {
        println!("{i}");
    }

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{} world!",s2);

}





fn guess_number(){
    let secret_number: u8 = rand::thread_rng().gen_range(0..=100);
    println!("secret number is {}",secret_number);

    loop {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("something went wrong!");
        let guess: u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(e) => {
                match e.kind(){
                    IntErrorKind::Empty => println!("没有输入"),
                    IntErrorKind::InvalidDigit => println!("请输入数字"),
                    IntErrorKind::PosOverflow => println!("数字范围应该在0到255之间"),
                    IntErrorKind::NegOverflow => println!("NegOverflow"),
                    IntErrorKind::Zero => println!("Zero"),
                    _ => println!("_"),

                    
                }
                // println!("{}",e);
                println!("your number range must in 0 and 100");
                continue;
            }
        };
        
        
        println!("your number is {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("your number is less"),
            Ordering::Greater => println!("youer number is too big"),
            Ordering::Equal => {
                println!("congratulations!");
                break;
            }
        }
    }
}
