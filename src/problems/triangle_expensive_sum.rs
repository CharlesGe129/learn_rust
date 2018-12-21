fn triangle_expensive_sum() {
    let content = read_file("src/problems/triangle_expensive_path.txt");
    cal_path(reverse(content.split("\n").collect()));
}

fn read_file(filename: &str) -> String {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("File not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("read failed");
    content
}

fn reverse(arr: Vec<&str>) -> Vec<&str> {
    let mut rs = Vec::new();
    let mut i = arr.len();
    while i > 0 {
        i -= 1;
        rs.push(arr[i])
    }
    rs
}

fn add(s: &str, n1: i32, n2: i32) -> i32 {
    let n = s.to_string().parse::<i32>().unwrap();
    if n1 >= n2 { n1 + n } else { n2 + n}
}

fn cal_path(arr: Vec<&str>) {
    let mut cur: Vec<i32> = Vec::new();
    for i in 0..arr.len() {
        let temp = String::from(arr[i]);
        let numbers: Vec<&str> = temp.split(" ").collect();
        if i == 0 {
            for each in numbers {
                cur.push(each.to_string().parse::<i32>().unwrap())
            }
            continue
        }
        for j in 0..numbers.len() {
            cur[j] = add(numbers[j], cur[j], cur[j+1])
        }
        println!("numbers={:?}, cur={:?}", numbers, cur);
    }
    println!("sum={}", cur[0]);
}