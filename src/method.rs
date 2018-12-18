fn method_test() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius
            }
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    let c = Circle {x: 0.0, y: 0.0, radius: 2.0};
    println!("Circle area: {}", c.area());
    println!("New circle area: {}", Circle::new(0.0, 0.0, 2.0).area());
}