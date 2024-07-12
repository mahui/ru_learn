
use games::GuessNumber;


pub mod games;

fn main() {

    ru_learn::eat_at_restaurant();
    //猜数字游戏。包含 io 输入，loop循环，match控制流解构
    GuessNumber::start_game();

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


    let mut s1 = String::from("hello world");
    let s2 = &mut s1;
   

    
    
    println!("str1={},Pstr1={:p}\n\rstr2={},Pstr2={:p}",*s2,s2,s2,&s2);

    let str3 = &mut s1;
    
    println!("str3={},Pstr3={:p}",str3,&str3);

    //slice
    println!("{}",first_word(&s1));

    assert_eq!(first_word(&s1),"hello");

    //Struct
    let mut user = User{
        name : String::from("张三"),
        email : String::from("zhangsan@test.com")
    };
    user.email = String::from("ddd");
    println!("name is {}",user.name);
    println!("email is {}",user.email);
    let mut user2 = build_user(String::from("lisi"));
    println!("name is {}",user2.name);

    //使用另一个struct初始化新的struct
    let user3 = User{
        name : String::from("wangwu"),
        ..user
    } ;
    println!("u3 name is {}\n\ru3 email is {}",user3.name,user3.email);

    let user4 = User::noname();
    user4.whoami();


    //Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(112);
    println!("first values is {}",&v[0]);
    let item = v.get(0);
    match item {
        Some(item)=> println!("get first values is {item}"),
        None => println!("no such value")
    }

}


struct User{
    name :String,
    email :String
}

impl User {
    fn whoami(&self) {
        println!("I am {}",self.name);
    }
    fn noname() -> User{
        build_user(String::from("无名"))
    }
}

fn build_user(name :String) ->User {
    User { 
        name: name, 
        email: String::from("build") 
    }
}


fn first_word(s: &String)-> &str{
    for (i,item) in s.bytes().into_iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s;
}


