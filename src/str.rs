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
}
