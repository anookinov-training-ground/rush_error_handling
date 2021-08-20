use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");

    // run with RUST_BACKTRACE=1 cargo run
    // let v = vec![1, 2, 3];
    // v[99];

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let f: u32 = File::open("hello.txt");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("{:#?}", f);

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:#?}", f);

    let f = File::open("hello.txt").unwrap();
    println!("{:#?}", f);

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("{:#?}", f);
}
