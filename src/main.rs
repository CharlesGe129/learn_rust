include!("display.rs");


fn main() {
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    #[derive(Debug)]
    struct Structure(i32);
    println!("Now {:?} is printable!", Structure(3));
    println!("And pretty print version: \n {:#?}", Structure(3));

    display();
}
