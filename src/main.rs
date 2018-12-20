include!("display.rs");
include!("complicated/list.rs");
include!("literals_operators.rs");
include!("array.rs");
include!("str.rs");
include!("struct.rs");
include!("control_flow.rs");
include!("function.rs");
include!("method.rs");
include!("trait.rs");
include!("inheritance.rs");
include!("generics.rs");
include!("io.rs");
include!("problems/verify_closable.rs");

use std::fmt;
use std::io;

fn main() {
    examples();
//    problems();
}

fn examples() {
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    #[derive(Debug)]
    struct Structure(i32);
    println!("Now {:?} is printable!", Structure(3));
    println!("And pretty print version: \n {:#?}", Structure(3));

    display_test();
    list_test();
    literal_operator_test();
    array_test();
    str_test();
    struct_test();
    control_flow_test();
    function_test();
    method_test();
    trait_test();
    inheritance_test();
    generics_test();
    io_test();
}

fn problems() {
    verify_closable_test();
}
