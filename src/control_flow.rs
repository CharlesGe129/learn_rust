fn control_flow_test() {
    if_test();
    for_test();
    while_test();
    match_test();
}

fn if_test() {
    let y = 1;
    let z = if y == 1 {1} else {0};
    println!("{}", z);
}

fn for_test() {
    for i in 0..2 {
        println!("{}", i);
    }

    for i in [0, 1, 2].iter() {
        println!("{}", i);
    }

    for (no, j) in "line1\nline2\nline3".lines().enumerate() {
        println!("#{}: {}", no, j)
    }
}

fn while_test() {
    'outer: loop {
        println!("Outer loop");
        'inner: loop {
            println!("Inner loop");
            break 'outer
        }
        println!("No print, outer has been break");
        break
    }
}

fn match_test() {
    let day = 5;
    match day {
        0 | 6 => println!("Weekend"),
        e @ 1...5 => println!("Weekday from range {}", e),
        _ => println!("invalid"),
    }

    // ref
    match day {
        ref r => println!("Got a ref to {}", r)
    }

    // pair
    let pair = (0, 1);
    match pair {
        (0, y) => println!("x=0, y={}", y),
        (x, 0) => println!("x={}, y=0", x),
        _ => println!("Pair without zero")
    }

    // match struct
    // http://wiki.jikexueyuan.com/project/rust-primer/quickstart/control-flow.html

    // if let
    let number = Some(7);
    let mut optional = Some(0);

    if let Some(i) = number {
        println!("Matched {}", i);
    } else {
        println!("Didn't match a number");
    }

    while let Some(i) = optional {
        if i > 9 {
            println!("Bigger than 9, break");
            break
        } else {
            println!("While let {}", i);
            optional = Some(i + 1);
        }
    }
}
