fn trait_test() {
    trait HasArea {
        fn area(&self) -> f64;
    }
    trait HasAngle {
        fn comment(&self) -> String {
            String::from("This shape has several angles")
        }

        fn angle(&self) -> f64;
    }

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    impl HasAngle for Square {
        fn angle(&self) -> f64 {
            90.0
        }
    }

    fn print_area<T: HasArea>(shape: T) {
        println!("Shape has an area {}", shape.area())
    }

    fn print_angle<T> (shape: T) where T: HasArea + HasAngle {
        println!("{}: {}", shape.comment(), shape.angle())
    }

    print_area(Circle {x: 0.0, y: 0.0, radius: 2.0});
    print_angle(Square {x: 0.0, y: 0.0, side: 2.0});
}

fn trait_with_generics() {
    use std::fmt::Debug;

    fn foo<T: Debug + Clone>(s: T) {
        s.clone();
        println!("{:?}", s);
    }

    fn foo2(s: impl Debug + Clone) {
        s.clone();
        println!("{:?}", s);
    }
}

// derive
// #[derive(Debug)]
// Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd