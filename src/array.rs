fn array_test() {
    let mut array: [i32; 3] = [0; 3];
    for v in &array[1..2] {
        println!("{}", v);
    }
    let v: Vec<i32> = Vec::new();
    let v: Vec<i32> = vec![];
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    v[0] += 1;
    println!("v={:?}", v);
    for each in &mut v {
        *each += 1;
    }
    println!("v={:?}", v);
}
