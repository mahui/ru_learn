# String

## 初始化字符串

```
let mut test = String::new();
```

## 二次释放错误

```
let s1 = String::from("hello");
let s2 = s1;
println!("{} world!",s1);
```
以上代码会发生二次释放错误。 * error[E0382]: borrow of moved value: `s1` *
二次释放：当两个变量的值指针都指向同一块地址，当它们离开作用域时，rust会自动调用 drop 函数并清理变量的堆内存。但内存地址一样，释放两次，会导致严重内存安全漏洞。以上代码，当 let s2 = s1 被执行时；rust认为 s1 不再有效。s1 被移动到 s2 中了。