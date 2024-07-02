use std::{cmp::Ordering, num::IntErrorKind, string};

use rand::Rng;

fn main() {

    //猜数字游戏。包含 io 输入，loop循环，match控制流解构
    // guess_number();

    //元组结构
    let tup: (i32, f64, i32) =  (5112, 3.14, 123);

    let tup1 = tup;

    println!("first value of tup is {}", tup.0);
    println!("first value of tup1 is {}", tup1.0);

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

    println!("s1 is {} , point of s1 is {:p}",s1, &s1);
    let s2 = s1.clone();
    // let s3 = s1;
    
    println!("s1 is {} , point of s1 is {:p}; s2 is {}, point of s2 is {:p}",s1, &s1, s2, &s2);

    // println!("s3 is {} , point of s3 is {:p}",s3, &s3);

    let x = 10;
    let y = x;

    println!("x={}, px={:p}; y={}, py={:p}",x,&x,y,&y);


    let mut str1 = String::from("hello world");
    let str2 = &mut str1;
   

    
    
    println!("str1={},Pstr1={:p}\n\rstr2={},Pstr2={:p}",*str2,str2,str2,&str2);

    let str3 = &mut str1;
    
    println!("str3={},Pstr3={:p}",str3,&str3);

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
