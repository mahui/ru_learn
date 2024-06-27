# function

## rust 方法声名格式：

```
fn 方法名 (参数名1: 参数类型1, 参数名2: 参数类型2) -> 返回值类型 {

}
```

## 表达式：

表达式会计算并产生一个值。结尾不用分号。
如：
```
let y = {
    let x = 10;
    x + 1
}
//此时y = 11;
```

## Loop循环

rust loop循环与其他语言一样，break遵循就近原则。如果两个loop嵌套，在最内侧loop代码块中执行break，只会终止最内侧loop，如果要在内侧loop中终止外侧loop，可以使用循环标签。语法如下：
```

// 'counting_up: 是一个循环标签
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            // 这个break虽然在内侧loop代码块，但因为制定了循环标签，所以会把最外侧的循环终止掉。
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
}
```

## for循环

```
for item in list{

}
```

```
//for 循环
for i in 0..4  {
    println!("{i}");
}
```

```
//倒序循环
for i in (0..4).rev() {
    println!("{i}");
}
```


