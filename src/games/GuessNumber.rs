use std::{cmp::Ordering, num::IntErrorKind};

use rand::Rng;


pub fn start_game(){
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
