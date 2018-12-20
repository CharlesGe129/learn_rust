fn str_test() {
    let hello = "hello world!";
    // static
    let hello: &'static str = "hello world!";

    let mut hello = String::new();
    hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld!");

    assert_eq!(hello.pop(), Some('!'));
    assert_eq!(hello.pop(), Some('d'));
    assert_eq!(hello.pop(), Some('l'));

    assert_eq!(r"\n", "\\n");

    let mut s = format!("{1} is a person with the weight {0:^0width$}kg, and the height {height:?}cm",
                    81, "A", width=4, height=178);
    println!("{}", s);

    s = format!("rust.cc社区的唐{CaiNiao:?}眼睛度数足有{0:0>4$.1}度却还是每天辛苦码代码才能赚到100个{3}。
但是{2}却只需睡{1:^6}个小时就可以迎娶白富美了。", 500.0, 12, "ELTON", "QB", 4, CaiNiao="Mike");
    println!("{}", s);

    // More on format: http://wiki.jikexueyuan.com/project/rust-primer/type/operator-and-formatting.html
}
