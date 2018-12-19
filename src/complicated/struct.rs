use std::cell::Cell;

fn struct_test() {
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point{x: 1, y: 2};

    // tuple struct
    struct Color(u8, u8, u8);
    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, green, blue) = android_green;
    println!("{}", red);

    // Tuple struct's constructor as a function
    #[derive(Debug)]
    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();
    println!("{:?}", d);

    #[derive(Default)]
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    impl fmt::Display for Point3 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x={}, y={}, z={}", self.x, self.y, self.z)
        }
    }

    let origin = Point3::default();
    println!("origin: {}", origin);
    let point = Point3 {y: 1, ..origin};
    println!("point: {}", point);
    let Point3 {x: x0, y: y0, ..} = point;
    println!("x0: {}, y0: {}", x0, y0);

    struct Point_mut {
        x: i32,
        y: Cell<i32>,
    }
    let point = Point_mut{x: 1, y: Cell::new(2)};
    point.y.set(3);
}
