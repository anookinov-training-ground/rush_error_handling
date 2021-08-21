use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
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

    let s = read_username_from_file();
    println!("{:#?}", s);

    let s = read_username_from_file_shortcut();
    println!("{:#?}", s);

    let s = read_username_from_file_shortcut_chain();
    println!("{:#?}", s);

    let s = read_username_from_file_to_string();
    println!("{:#?}", s);

    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{:#?}", home);

    // loop {
    //     // --snip--

    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     if guess < 1 || guess > 100 {
    //         println!("The secret number will be between 1 and 100.");
    //         continue;
    //     }

    //     match guess.cmp(&secret_number) {
    //         // --snip--
    //     }
    // }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let guess = Guess::new(11);
    println!("{:#?}", guess.value());

    let f = File::open("hello.txt")?;
    println!("{:#?}", f);
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortcut_chain() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_to_string() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}