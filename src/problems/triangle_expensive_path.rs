fn triangle_expensive_path() {
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

fn add(s: String, (n1, s1): (i32, String), (n2, s2): (i32, String)) -> (i32, String) {
    let n = s.parse::<i32>().unwrap();
    if n1 > n2 { (n1 + n, s + "-" + &*s1) } else { (n2 + n, s + "-" +&*s2) }
}

fn cal_path(arr: Vec<&str>) {
    let mut sum: Vec<i32> = Vec::new();
    let mut path: Vec<String> = Vec::new();
    for i in 0..arr.len() {
        let temp = String::from(arr[i]);
        let numbers: Vec<&str> = temp.split(" ").collect();
        if i == 0 {
            for each in numbers {
                sum.push(each.to_string().parse::<i32>().unwrap());
                path.push(each.to_string());
            }
            continue
        }
        for j in 0..numbers.len() {
            let (temp_sum, temp_path) = add(numbers[j].to_string(), (sum[j], path[j].to_string()), (sum[j+1], path[j+1].to_string()));
            sum[j] = temp_sum;
            path[j] = temp_path;
        }
        println!("numbers={:?}, cur={:?}", numbers, sum);
    }
    println!("sum={}, path={}", sum[0], path[0]);
}