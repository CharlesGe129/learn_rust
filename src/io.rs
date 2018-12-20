use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn io_test() {
    std_in();
    file_in();
    file_out();
}

fn std_in() {
    // example 1
    fn read_input() -> io::Result<()> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        println!("You typed: {}", input.trim());
        Ok(())
    }

//    read_input();

    // example 2
    let mut input = String::new();
//    io::stdin().read_line(&mut input).expect("Exception!");
    println!("You typed: {}", input.trim());
}

fn file_out() {
    let path = Path::new("src/hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => println!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => println!("{} contains: \n{}", display, s),
    }
}

fn file_out_example2() {
    let mut f = File::open("src/hello.txt").expect("File not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("read failed");
}

fn file_in() {
    static INPUT_CONTENT: &'static str = "I love Rust.";

    let path = Path::new("src/output.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(INPUT_CONTENT.as_bytes()) {
        Err(why) => println!("couldn't write to {}: {}", display, Error::description(&why)),
        Ok(_) => println!("successfully write to {}", display)
    }
}