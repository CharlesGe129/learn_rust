fn function_test() {
    println!("2 + 1 = {}", add_one(2));
//    let x: i32 = diverges();
//    let y: String = diverges();
    closure();
    high_order_function();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// No return
fn diverges() -> ! {
    panic!("No returns, and ! suits for any type");
}

fn closure() {
    let plus_5 = |x: i32| x + 5;
    println!("Closure 1 + 5 = {}", plus_5(1));
    let mut num = 3;

    let mut add_num = move |x: i32| num += x;
    let mut temp = num;
    add_num(3);
    println!("With move, {} + 3 = {}", temp, num);

    let mut add_num_no_move = |x: i32| num += x;
    add_num_no_move(3);
    println!("No move, {} + 3 = {}", temp, num);
}

fn high_order_function() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn apply<F>(f: F, y: i32) -> i32 where F: Fn(i32) -> i32 {
        f(y) * y
    }

    println!("7 * 8 = {}", apply(add_one, 7));

    fn factory(x: i32) -> Box<Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }

    let box_fn = factory(1);
    let b0 = box_fn(2);
    let b1 = (*box_fn)(2);
    let b2 = (&box_fn)(2);
    println!("b0 = {}, b1 = {}, b2 = {}", b0, b1, b2);

    let add_num = &(*box_fn);
    let translate: &Fn(i32) -> i32 = add_num;
    let c0 = add_num(2);
    let c1 = apply(add_num, 2);
    let c2 = apply(translate, 2);
    println!("c0 = {}, c1 = {}, c2 = {}", c0, c1, c2);
}