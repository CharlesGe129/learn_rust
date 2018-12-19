fn inheritance_test() {
    trait Foo {
        fn foo(&self);

        fn bar(&self) {
            println!("default method");
        }
    }

    trait FooBar : Foo {
        fn foobar(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn foo(&self) {
            println!("foo");
        }
    }

    let mut baz = Baz;
    println!("This is Foo implementation: ");
    baz.foo();
    baz.bar();

    impl FooBar for Baz {
        fn foobar(&self) {
            println!("foobar");
        }
    }

    baz = Baz;
    println!("This is FooBaz implementation: ");
    baz.foo();
    baz.bar();
    baz.foobar();

    // std traits: Drop, Borrow, AsRef, Deref<Target=T>, Iterator, Sized
}