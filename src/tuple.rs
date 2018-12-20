fn tuple_test() {
    let y = (2, "hello world");
    let x: (i32, &str) = y;
    let (w, z) = y;
    let x1 = x.0;
    let x2 = x.1;
}