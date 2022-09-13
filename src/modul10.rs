#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

//Working with files
//Errors
//Helper methods
//? operator

//-----------------------------------------Working with files
//Create a file
//use std::fs::File
//let mut file = File::create(“name.txt”).expect(“failed”);

// Write to a file
//use std::io::Write;
//file.write_all("Hello".as_bytes()).expect("failed");

// Append content to file
//use std::fs::OpenOptions;
// let mut file = OpenOptions::new().append(true)
//    .open("name.txt").expect("failed");
// file.write_all(" world!\n".as_bytes()).expect("failed");

//Read from a file
//use std::io::Read;
//let mut file = std::fs::File::open("name.txt").unwrap();
//file.read_to_string(&mut contents).unwrap();

// Delete a file
//use std::fs;
//fs::remove_file("name.txt").expect("failed");

use std::fs::{File, OpenOptions, remove_file};
use std::io::{Write, Read};

pub fn file_hand() {
    // let mut file = File::create("src/example.txt").expect("create failed");
    // file.write_all("Hello World!\n".as_bytes()).expect("write failed");

    // let mut file = OpenOptions::new().append(true)
    //     .open("src/example.txt")
    //     .expect("cannot open file");
    // file.write_all("Adding content to the file.\n".as_bytes()).expect("write failed");

    let mut file = File::open("src/example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    // remove_file("src/example.txt").expect("delete failed");
}

//-----------------------------------------Errors
//2 types of errors
// Recoverable			- Result enum
// Unrecoverable		- panic! macro
//Unrecoverable errors
//panic!(message);
//Panic will terminate the program or thread

//Recoverable errors
//Result enum
//enum Result<T,E> {
//    OK(T),
//    Err(E)
// }
//match res {
//    Ok(a) => { ... }
//    Err(b) => { ... }
// }
//Option enum
//enum Option<T,E> {
//    Some(T),
//    None
// }
//match res {
//    Some(a) => { ... }
//    None => { ... }
// }


pub fn mod10_errors() {
    // let v = vec![1, 2, 3];
    // v[10];
    // panic!("Something went wrong. cannot proceed");

    let f = File::open("main.jpg");
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        },
        Err(e) => {
            println!("file not found \n{:?}", e);
        }
    }
    println!("Continuing on with the execution");

    divide(Some(1));
    divide(Some(10));
    divide(None);
    //divide(Some(0));
}

const ANSWER_TO_LIFE: i32 = 42;

fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("Cannot divide by 0"),
        Some(x) => println!("result is {}", ANSWER_TO_LIFE / x),
        None => println!("None received, the answer is {}", ANSWER_TO_LIFE)
    }
}

//-----------------------------------------Helper methods
//unwrap - will return the data if it’s available or panic! of it’s not
//File::open("example.txt").unwrap();

//expect - similar to unwrap but allows us to set a custom error message
//File::open("example.txt").expect(“Unable to open file”);

pub fn help_meth() {
    // let f = File::open("main.jpg").unwrap();
    //let f = File::open("main.jpg").expect("Unable to open file");
}

//-----------------------------------------? operator
//A shorthand for an entire match statement
//let mut f = File::open("example.txt")?;

use std::error::Error;
use std::io;

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("src/username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("src/example.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn q_oper() {
    let a = read_username_from_file();
    println!("{:?}", a);
}











