fn generics_test() {
    generics();
    polymorphism();
}

fn generics() {
    enum Option<T> {
        Some(T),
        None,
    }

    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0_f64);

    fn make_pair<T, U>(a: T, b: U) -> (T, U) {
        (a, b)
    }

    let couple = make_pair("male", "female");

    struct Point<T> {
        x: T,
        y: T,
    }

    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };
}

fn polymorphism() {
    // Unable to compile. More on: http://wiki.jikexueyuan.com/project/rust-primer/quickstart/trait.html
    /*
    {
        // use generic type
        trait Graph<N, E> {
            fn has_edge(&self, &N, &N) -> bool;
            fn edges(&self, &N) -> Vec<E>;
        }

        fn distance<N, E, G: Graph<N, E>> (graph: &G, start: &N, end: &N) -> u32 {}
    }

    {
        // use associated types
        trait Graph<N, E> {
            type N;
            type E;

            fn has_edge(&self, &Self::N, &Self::N) -> bool;
            fn edges(&self, &Self::N) -> Vec<Self::E>;
        }

        fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> uint {};
    }
    */
}